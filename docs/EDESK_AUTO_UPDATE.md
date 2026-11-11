# eDesk Automatic Update Policy

**Status:** Product requirement and implementation plan; eDesk-specific release integration pending.

## Goal

Notify users inside the eDesk desktop application when a newer stable eDesk release is available. The user must be able to install it immediately or postpone the reminder.

## User experience

When a newer stable version is detected, show a dialog:

**eDesk Update Available**

A new version of eDesk is available. Update now to get the latest features, improvements and security fixes.

- Show the installed version and the available version.
- **Update Now** starts the verified download and installation flow.
- **Remind Me Later** closes the dialog and postpones the reminder.
- Provide a release-notes link.
- Do not show the same version notification repeatedly after the user postpones it; remind again on a later launch or after a reasonable interval.
- Show clear progress, cancellation where safe, and an error message if verification or installation fails.

## Check behavior

1. Check for updates after startup, with a short delay so the application can finish opening.
2. Check periodically (default: once every 24 hours) while the app is running.
3. Add a manual **Check for updates** action in Settings > About.
4. Offer an **Automatically check for updates** setting, enabled by default. Turning it off disables background checks but keeps manual checks available.
5. Do not interrupt active remote-control, file-transfer, voice, or other sessions. Defer installation until all sessions have ended.
6. Do not force-install an update. The user confirms installation through **Update Now**.
7. Do not silently downgrade or install a release that is not newer than the installed version.
8. Stable releases only by default; prereleases are excluded unless a future advanced setting is explicitly added.

## Release and verification architecture

- Use the eDesk GitHub repository's own Releases as the authoritative update channel, not RustDesk's update service or RustDesk releases.
- Publish a versioned Windows x64 installer/package and release notes for each eDesk release.
- Publish signed update metadata that identifies the eDesk package, semantic version, release ID, platform, architecture, package format, HTTPS download URL, file size, and SHA-256 digest.
- Sign metadata with a dedicated eDesk Ed25519 release key. Keep the private signing key only in GitHub Actions secrets; never commit it, put it in the app, or ask users to paste it into chat.
- Embed only the matching public verification key in the client. Verify the metadata signature, expected eDesk package identity, version, release URL, file size, and SHA-256 before allowing installation.
- Keep RustDesk's existing verified-update and safe-installation mechanisms where compatible, but do not point eDesk at RustDesk's release feed or trust RustDesk's signing key as an eDesk release key.
- Fail closed: if metadata, signature, package identity, or digest verification fails, do not install the update.
- Never use an unsigned mutable download URL as the trust mechanism.

## Compatibility and implementation notes

The pinned RustDesk 1.5.3 baseline already contains an update-check thread, a Flutter update progress dialog, and verified update artifact handling. However, the upstream common update check explicitly skips custom clients and the upstream metadata currently identifies RustDesk. eDesk must therefore add its own release feed and eDesk trust identity before enabling automatic checks. Reuse compatible UI, download progress, and verified installer code instead of building a parallel updater.

The first eDesk release workflow currently creates a short-lived CI artifact, not a signed public release. A separate release workflow must be added after the eDesk signing public key is embedded and the matching private key is safely configured as a repository Actions secret.

## Acceptance tests

1. A newer eDesk stable release triggers one in-app notification with installed and available versions.
2. Update Now downloads the correct Windows x64 eDesk package, verifies signed metadata and SHA-256, then launches the safe installer.
3. Remind Me Later postpones the same notification.
4. No update is offered when the installed version is current or newer.
5. RustDesk releases and RustDesk update services are never treated as eDesk updates.
6. Invalid signatures, changed artifacts, wrong package IDs, or mismatched hashes are rejected.
7. Background checks can be disabled; manual checks still work.
8. Installation is deferred while any remote session, file transfer, or voice session is active.
9. Network failure does not corrupt the installed application or block normal eDesk use.
