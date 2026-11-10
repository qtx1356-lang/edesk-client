# Windows build

Follow the current official guide: https://rustdesk.com/docs/en/dev/build/

The exact toolchain and native dependencies vary by upstream revision. Follow the guide for the commit you pin.

Suggested sequence:
1. Install Git for Windows.
2. Install Visual Studio Build Tools with the C++ desktop workload.
3. Install the Rust toolchain specified by upstream.
4. Install and configure vcpkg as documented upstream.
5. Clone the fork with submodules.
6. Build the untouched upstream fork.
7. Only after a successful baseline build, apply eDesk branding.

Example clone command (replace YOUR_GITHUB_USERNAME):

```powershell
git clone --recurse-submodules https://github.com/YOUR_GITHUB_USERNAME/edesk-client.git
cd edesk-client
```
