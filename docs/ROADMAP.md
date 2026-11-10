# eDesk Development Roadmap

eDesk is a free Windows remote-support application based on a maintained fork of RustDesk. This roadmap separates project setup, baseline validation, product customization, and release readiness.

## Current status

- [x] Create the eDesk project repository
- [x] Add the initial project README and ignore rules
- [x] Establish a project-foundation branch
- [ ] Import the RustDesk client source while preserving upstream history and license notices
- [ ] Build and run an unmodified Windows baseline
- [ ] Record the upstream commit/tag used as the baseline
- [ ] Apply eDesk name, icons, package metadata, and product wording
- [ ] Verify remote screen viewing and keyboard/mouse control
- [ ] Verify file transfer in both directions
- [ ] Verify two-way audio; document any platform or permission limitations
- [ ] Configure and test the chosen eDesk rendezvous/relay server
- [ ] Add automated Windows build checks
- [ ] Prepare signed, checksummed release artifacts and user documentation

## Milestones

### 1. Foundation
Repository structure, contribution guidance, licensing notes, build instructions, and security expectations.

### 2. Upstream baseline
Fork or import the official RustDesk client repository without flattening its history. Build the unmodified upstream version first. Keep a record of the upstream remote and exact commit.

### 3. eDesk identity
Change product branding only after the baseline works. Keep third-party notices, license files, and attribution intact. Do not imply that eDesk is an official RustDesk product.

### 4. Feature validation
Test remote display, input control, clipboard behavior, file transfer, reconnect behavior, and two-way voice on supported Windows configurations.

### 5. Infrastructure
Deploy and monitor the compatible open-source rendezvous and relay services. Document DNS, ports, firewall rules, backups, updates, and operational costs before inviting public users.

### 6. Release readiness
Add security review, clear session-consent UX, signed binaries, SHA-256 checksums, release notes, and a vulnerability-reporting contact.

## Development rules

- Keep changes small and reviewable.
- Prefer upstream-compatible changes over large rewrites.
- Never commit passwords, private keys, tokens, or server credentials.
- Do not enable unattended access by default.
- Require clear local approval for incoming sessions; make persistent access explicit and revocable.
- Treat voice and screen sharing as sensitive session data.
- Test on a clean Windows machine before publishing a release.
