# eDesk roadmap

## 0. Repository
- [ ] Fork rustdesk/rustdesk as edesk-client
- [ ] Preserve upstream history, license, and notices
- [ ] Record the upstream commit used

## 1. Windows baseline
- [ ] Set up the official Windows build environment
- [ ] Build the unmodified client
- [ ] Record toolchain and dependency versions
- [ ] Add a reproducible GitHub Actions build

## 2. eDesk branding
- [ ] App name, icons, installer metadata
- [ ] About screen and open-source notices
- [ ] Keep protocol compatibility with the pinned upstream version

## 3. Infrastructure
- [ ] Deploy RustDesk Server OSS hbbs and hbbr
- [ ] Configure DNS, firewall, and server public key
- [ ] Test direct and relayed connections

## 4. Features
- [ ] Verify screen viewing and remote input
- [ ] Verify file transfer, large files, and interrupted transfers
- [ ] Test two-way voice in an active session
- [ ] Implement additional voice support only if required by the chosen upstream revision

## 5. Release
- [ ] Session approval and visible active-session indicator
- [ ] Opt-in, revocable unattended access
- [ ] Security and dependency review
- [ ] Windows installer, checksums, source, and release notes
