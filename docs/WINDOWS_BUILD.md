# Windows Build Checklist

This document is a preparation checklist, not a guarantee that the current upstream source builds with every tool version. Follow the build instructions for the exact RustDesk revision pinned by eDesk.

## Before building

Use a 64-bit Windows development machine with:

- Git for Windows and Git Bash.
- Visual Studio Community with the Desktop development with C++ workload / MSVC toolchain.
- Rust installed using the official rustup installer.
- The native dependencies required by the selected RustDesk revision, commonly managed through Microsoft's vcpkg.
- LLVM/Clang if required by the selected revision's bindgen/native dependency setup.
- The GUI runtime/assets required by that revision.

Allow substantial free disk space for the compiler, package manager, and native dependencies.

## Important version note

RustDesk's desktop UI and build workflow have evolved. Do not assume older Sciter-specific instructions apply to a current Flutter-based build. Check the official Windows guide and the CI workflow at the exact upstream commit before installing GUI dependencies or running build commands.

Official guide: https://rustdesk.com/docs/en/dev/build/windows/

## Baseline-first procedure

1. Clone the eDesk repository with all submodules after the upstream source has been imported.
2. Check out the recorded baseline commit or the eDesk integration branch.
3. Read the upstream Windows build guide and repository CI configuration for that revision.
4. Install the exact required Rust, MSVC, vcpkg, LLVM/Clang, and GUI dependencies.
5. Build the unmodified upstream client.
6. Launch it locally and verify basic startup before changing branding.
7. Record tool versions, commands, build output, and any fixes in the project issue tracker.

## Git Bash reminder

The official Windows guide documents shell commands for Git Bash. Do not paste Bash environment-variable syntax directly into Command Prompt or PowerShell.

## Build evidence to record

- Windows edition and version.
- Rust toolchain version.
- Visual Studio/MSVC version.
- vcpkg revision and installed triplets.
- LLVM/Clang version, if used.
- RustDesk upstream commit.
- Exact build command and whether it completed.
- Any runtime DLLs/assets required to launch the build.

Do not publish a binary until it has been tested on a clean Windows environment and its source revision and dependencies are documented.
