mod file;

use super::{
    get_custom_client_staging_dir, get_reg_of_hkcr, handle_custom_client_staging_dir_before_update,
    installer_files::ProtectedInstallFile,
    installer_handoff::{run_cmds, InstallCommands},
    installer_shell::{get_system_executable, path_for_cmd_environment},
    remove_custom_client_staging_dir, ResultType, REG_NAME_INSTALL_PRINTER,
};
use hbb_common::{allow_err, anyhow::anyhow, bail, log};
use std::path::Path;

pub use file::{copy_and_verify_update_file_sha256, VerifiedUpdateFile};

const VERIFIED_UPDATE_FILE_PREFIX: &str = "rustdesk-verified-";
const VERIFIED_EXE_FILENAME: &str = "update.exe";
const VERIFIED_MSI_FILENAME: &str = "update.msi";
const VERIFIED_UPDATE_COPY_FAILURE_EXIT_CODE: u32 = 0x5253_0009;
const VERIFIED_EXE_LAUNCH_FAILURE_EXIT_CODE: u32 = 0x5253_000C;

pub fn update_to_verified(file: &str, expected_sha256: &str, expected_size: u64) -> ResultType<()> {
    let extension = file::update_file_extension(file).unwrap_or_default();
    if extension != "exe" && extension != "msi" {
        bail!("Unsupported update file format: {}", file);
    }

    let update_file = copy_and_verify_update_file_sha256(file, expected_sha256)?;
    if let Err(err) = update_file.verify_size(expected_size) {
        update_file.cleanup();
        return Err(err);
    }
    let update_path = match update_file.path_str() {
        Ok(path) => path.to_owned(),
        Err(err) => {
            update_file.cleanup();
            return Err(err);
        }
    };
    let custom_client_staging_dir = get_custom_client_staging_dir();
    if crate::is_custom_client() {
        if let Err(err) = handle_custom_client_staging_dir_before_update(&custom_client_staging_dir)
        {
            update_file.cleanup();
            return Err(err);
        }
    } else {
        // Clean up any residual staging directory from previous custom client.
        allow_err!(remove_custom_client_staging_dir(&custom_client_staging_dir));
    }

    let result = launch_verified_update(&extension, &update_path, expected_sha256);
    clear_custom_client_staging_after_launch_failure(&custom_client_staging_dir, &result);
    update_file.cleanup();
    result
}

fn launch_verified_update(
    extension: &str,
    update_path: &str,
    expected_sha256: &str,
) -> ResultType<()> {
    match extension {
        "exe" => {
            if let Err(err) = launch_verified_exe(update_path, expected_sha256) {
                bail!("Failed to run the update exe: {}", err);
            }
        }
        "msi" => {
            if let Err(err) = install_verified_msi(update_path, expected_sha256, false) {
                bail!("Failed to run the update msi: {}", err);
            }
        }
        _ => bail!("Unsupported update file format: {}", update_path),
    }
    Ok(())
}

fn launch_verified_exe(exe: &str, expected_sha256: &str) -> ResultType<()> {
    // Keep the SHA-256-verified, protected staging copy, but hand it to the
    // elevated command runner rather than ShellExecuteEx directly. run_cmds
    // releases the staging file's retained DELETE handle before cmd.exe starts;
    // otherwise the installer can encounter a sharing violation while Windows
    // opens or replaces the executable during the update.
    let update = ProtectedInstallFile::copy_verified(Path::new(exe), expected_sha256)?;
    let staged_path = update.path();
    let staged_metadata = std::fs::metadata(staged_path).map_err(|err| {
        anyhow!(
            "Verified update staging file is missing or inaccessible (source: {}, staged: {}): {err}",
            exe,
            staged_path.display()
        )
    })?;
    if !staged_metadata.is_file() {
        bail!(
            "Verified update staging path is not a regular file: {}",
            staged_path.display()
        );
    }
    if staged_path.extension().and_then(|value| value.to_str()) != Some("exe") {
        bail!(
            "Verified update staging file does not have an .exe extension: {}",
            staged_path.display()
        );
    }
    let staged_path = path_for_cmd_environment(staged_path)?;
    let current_exe = std::env::current_exe()?;
    let install_dir = current_exe
        .parent()
        .ok_or_else(|| anyhow!("Installed eDesk executable has no parent directory"))?;
    let install_dir = path_for_cmd_environment(install_dir)?;
    log::info!(
        "Handing verified eDesk setup installer to elevated runner: source={}, staged={}, install_dir={}, size={} bytes",
        exe,
        staged_path,
        install_dir,
        staged_metadata.len()
    );

    // The update handler runs inside the GUI process. Other eDesk processes
    // (tray instances and the background service) can still hold installed DLLs,
    // so close the complete eDesk process tree before running the installer.
    // The taskkill commands are best-effort: the installer remains responsible
    // for reporting a real installation failure.
    let taskkill = get_system_executable("taskkill.exe")?;
    let taskkill = path_for_cmd_environment(&taskkill)?;
    let timeout = get_system_executable("timeout.exe")?;
    let timeout = path_for_cmd_environment(&timeout)?;
    let current_pid = std::process::id();
    let setup_log = Path::new(install_dir).join("eDesk-update-setup.log");
    let setup_log = path_for_cmd_environment(&setup_log)?;
    let script = format!(
        "\"{taskkill}\" /F /PID {current_pid} > nul 2>&1\r\n\"{taskkill}\" /F /IM edesk.exe > nul 2>&1\r\n\"{timeout}\" /T 4 /NOBREAK > nul\r\n\"{staged_path}\" /VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /CLOSEAPPLICATIONS /FORCECLOSEAPPLICATIONS /LOG=\"{setup_log}\" /LOGCLOSEAPPLICATIONS /EDESKUPDATE=1 /DIR=\"{install_dir}\"\r\nset \"RUSTDESK_SETUP_EXIT_CODE=%ERRORLEVEL%\"\r\nif not \"%RUSTDESK_SETUP_EXIT_CODE%\"==\"0\" exit /b %RUSTDESK_SETUP_EXIT_CODE%"
    );
    run_cmds(
        InstallCommands {
            script,
            files: vec![update],
        },
        false,
        "update-exe",
    )
    .map_err(|err| anyhow!("Verified eDesk setup installation failed: {err:#}"))?;
    log::info!("Verified eDesk setup installer completed successfully.");
    Ok(())
}

fn verified_msi_install_commands(msi: &Path, quiet: bool) -> ResultType<String> {
    use windows::Win32::Foundation::{
        ERROR_SUCCESS, ERROR_SUCCESS_REBOOT_INITIATED, ERROR_SUCCESS_REBOOT_REQUIRED,
    };

    // The input is already verified and protected, including its parent path.
    // Execute the copy from the protected runner to exclude adjacent untrusted files.
    let source = path_for_cmd_environment(msi)?;
    let quiet_args = if quiet { " /qn LAUNCH_TRAY_APP=N" } else { "" };
    let app_name = crate::get_app_name();
    let subkey = format!(".{}", app_name.to_lowercase());
    let printer_installed = get_reg_of_hkcr(&subkey, REG_NAME_INSTALL_PRINTER).as_deref()
        != Some("0")
        && remote_printer::is_rd_printer_installed(&app_name).unwrap_or_else(|err| {
            log::warn!(
                "Failed to check printer installation status: {err}; skipping printer installation"
            );
            false
        });
    let printer_args = if printer_installed {
        ""
    } else {
        " INSTALLPRINTER=0"
    };
    Ok(format!(
        "set \"RUSTDESK_VERIFIED_MSI_SOURCE={source}\"\r\n\
         set \"RUSTDESK_VERIFIED_MSI=%RUSTDESK_OUTPUT_DIR%\\{VERIFIED_MSI_FILENAME}\"\r\n\
         copy /B /Y \"%RUSTDESK_VERIFIED_MSI_SOURCE%\" \"%RUSTDESK_VERIFIED_MSI%\" > nul || exit /b {VERIFIED_UPDATE_COPY_FAILURE_EXIT_CODE}\r\n\
         msiexec.exe /i \"%RUSTDESK_VERIFIED_MSI%\"{quiet_args}{printer_args} REBOOT=ReallySuppress /norestart\r\n\
         set \"RUSTDESK_MSI_EXIT_CODE=%ERRORLEVEL%\"\r\n\
         if \"%RUSTDESK_MSI_EXIT_CODE%\"==\"{reboot_initiated}\" exit /b {success}\r\n\
         if \"%RUSTDESK_MSI_EXIT_CODE%\"==\"{reboot_required}\" exit /b {success}\r\n\
         exit /b %RUSTDESK_MSI_EXIT_CODE%",
        success = ERROR_SUCCESS.0,
        reboot_initiated = ERROR_SUCCESS_REBOOT_INITIATED.0,
        reboot_required = ERROR_SUCCESS_REBOOT_REQUIRED.0,
    ))
}

pub(super) fn install_verified_msi(
    msi: &str,
    expected_sha256: &str,
    quiet: bool,
) -> ResultType<()> {
    let update = ProtectedInstallFile::copy_verified(Path::new(msi), expected_sha256)?;
    run_cmds(
        InstallCommands {
            script: verified_msi_install_commands(update.path(), quiet)?,
            files: vec![update],
        },
        false,
        "update-msi",
    )
}

fn clear_custom_client_staging_after_launch_failure(
    staging_dir: &Path,
    launch_result: &ResultType<()>,
) {
    if launch_result.is_ok() {
        return;
    }
    if let Err(err) = remove_custom_client_staging_dir(staging_dir) {
        log::warn!(
            "Failed to clear custom client staging directory {:?}: {}",
            staging_dir,
            err
        );
    }
}

pub fn schedule_current_verified_update_file_cleanup() -> ResultType<()> {
    let current_exe = std::env::current_exe()?;
    if should_schedule_verified_update_cleanup(&current_exe) {
        schedule_file_removal_on_reboot(&current_exe)?;
    }
    Ok(())
}

fn should_schedule_verified_update_cleanup(path: &Path) -> bool {
    // UAC may run under another account whose TEMP directory differs from the caller's.
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with(VERIFIED_UPDATE_FILE_PREFIX) && name.ends_with(".exe"))
}

fn schedule_file_removal_on_reboot(path: &Path) -> ResultType<()> {
    use windows::{
        core::{HSTRING, PCWSTR},
        Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_DELAY_UNTIL_REBOOT},
    };

    unsafe {
        MoveFileExW(
            &HSTRING::from(path),
            PCWSTR::null(),
            MOVEFILE_DELAY_UNTIL_REBOOT,
        )
    }
    .map_err(|err| {
        anyhow!(
            "Failed to schedule verified update file cleanup for {}: {}",
            path.display(),
            err
        )
    })?;
    Ok(())
}
