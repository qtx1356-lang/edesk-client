# eDesk Automatic Update

**Implementation status:** The Windows x64 update check, in-app notice, signed metadata verifier, verified download flow, and signed-release publishing workflow are connected. The matching Ed25519 key pair is now provisioned, and the release script rejects a signing secret that does not match the public key embedded in the client. The secret must still be configured in GitHub, and the complete flow needs a release build plus a real Windows update test before it is production-verified.

## User experience

- eDesk checks its own stable GitHub Releases feed at startup and approximately every 24 hours when automatic checks are enabled.
- A newer stable version appears on the eDesk home page with the installed and available versions.
- The user can select **Update** to verify, download, and install the update, or close the notice to postpone it until a later check.
- **Settings > General > Other** contains **Automatically check for eDesk updates**.
- **Settings > About** contains **Check for updates** and reports whether eDesk is current.
- eDesk does not silently install an update. Installation is user-initiated.
- Background checks do not install while a remote session is active.

## Trusted release channel

The only update source is:

`https://github.com/qtx1356-lang/edesk-client/releases/latest`

Stable release tags use `edesk-vX.Y.Z`. Drafts and prereleases are excluded. The client constructs and validates the release URL itself rather than trusting a URL from the API response.

The update payload is the self-contained Windows portable package produced by `libs/portable`, named `edesk-X.Y.Z-install.exe`. This package contains the full application runtime and is launched through the existing verified Windows update handoff. The separate `eDesk-Setup.exe` remains the first-install/recovery installer.

## Signed metadata

Every stable release must include:

- `edesk-update.json`
- `edesk-update.json.sig`
- `edesk-X.Y.Z-install.exe`
- `eDesk-Setup.exe`

The metadata binds the eDesk app/package identity, version, release tag, Windows x64 platform, package filename, exact GitHub asset URL, file size, and SHA-256 digest. The detached Ed25519 signature covers the metadata with the context string `eDesk update metadata v1\n`.

The public verification key is embedded in `src/update_metadata.rs`. The matching private seed is not stored in the repository. The GitHub Actions release step requires the repository Actions secret:

`EDESK_UPDATE_SIGNING_KEY`

Its value must be the single base64 line in `EDESK_UPDATE_SIGNING_KEY.txt` supplied separately to the repository owner. In GitHub, open **Settings → Secrets and variables → Actions → New repository secret**, use the exact name above, and paste the file's one-line value. The release workflow derives the public key from this secret and compares it with the embedded trusted public key before signing; a mismatch fails the release. Never commit this file, upload it as a workflow artifact, or paste its contents into source code or a public issue.

## Release procedure

1. Bump the application version in `Cargo.toml` and the matching root package entry in `Cargo.lock`.
2. Push a tag that exactly matches the manifest version, for example `edesk-v1.5.5`.
3. GitHub Actions builds the Windows app, portable update package, and Inno Setup installer.
4. The release job signs the metadata using `EDESK_UPDATE_SIGNING_KEY` and publishes all release assets.
5. The installed eDesk client discovers the stable release and displays the update notice.

A manual workflow run creates short-lived CI artifacts for testing; it does not publish a public release.

## Verification and safety rules

- Verify the metadata signature before trusting any artifact fields.
- Require the exact eDesk package identity, stable version, release tag, platform, architecture, and GitHub release asset URL.
- Verify the downloaded file size and SHA-256 before launching the update handoff.
- Reject malformed metadata, unknown signing keys, invalid signatures, wrong package IDs, mismatched URLs, unexpected filenames, and hash/size mismatches.
- Never use the RustDesk update service, RustDesk release feed, or RustDesk signing identity for eDesk.
- Never downgrade or install the same version again.
- Network or verification failures leave the current installation untouched.

## Acceptance tests

1. A newer stable eDesk release produces one in-app update notice.
2. The About page manual check works even when automatic checks are disabled.
3. Update Now downloads the signed Windows x64 package, verifies its metadata, size, and SHA-256, and starts the eDesk update handoff.
4. The update preserves the eDesk ID and user settings.
5. Closing the notice postpones it; a later launch/check can show it again.
6. No update is offered when the installed version is current or newer.
7. RustDesk releases and services are never considered eDesk updates.
8. Invalid signatures, changed files, wrong package IDs, and mismatched hashes are rejected.
9. Updates are not started during active remote sessions.
10. A failed check or download does not damage the installed application.

**Not yet production-verified:** CI build of the portable update package, GitHub release publication using the configured secret, and end-to-end update from an already installed eDesk version on Windows.
