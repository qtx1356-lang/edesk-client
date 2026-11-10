# eDesk Architecture

## Product scope

The first target is a Windows desktop client for consent-based remote support. Planned capabilities:

1. Remote screen viewing.
2. Keyboard and mouse control after authorization.
3. File transfer with visible transfer status and user confirmation.
4. Live two-way voice during an active remote session.
5. Device identification and connection through compatible rendezvous/relay infrastructure.

The initial implementation should reuse RustDesk's existing transport, session, input, display, file-transfer, and audio components where supported. Avoid building a parallel networking stack until the upstream implementation has been understood and tested.

## High-level components

- **eDesk Windows client:** user interface, session lifecycle, permission prompts, display/input, file transfer, and audio.
- **Rendezvous service (hbbs):** helps peers discover and establish connections.
- **Relay service (hbbr):** forwards traffic when a direct peer-to-peer path is unavailable.
- **Build and release pipeline:** reproducible build instructions, automated checks, signing, checksums, and release notes.

The exact supported configuration and protocol compatibility must be taken from the pinned RustDesk client/server revisions. Do not assume that arbitrary client and server versions are interchangeable.

## Session flow

1. The user launches eDesk and sees the local device ID and connection state.
2. An incoming connection request identifies the peer and requested session.
3. The local user explicitly approves or rejects the request.
4. eDesk establishes a direct connection where possible, otherwise uses the configured relay.
5. Screen, input, file-transfer, and audio capabilities are enabled only as authorized for that session.
6. The user can end the session immediately and can see when a session is active.

## Security requirements

- No hidden remote sessions.
- No unattended access enabled by default.
- Persistent/unattended access, if introduced, must be opt-in, clearly explained, separately protected, and revocable locally.
- Display a persistent active-session indicator and a prominent disconnect control.
- Do not transmit microphone audio before the user enables voice for the session.
- Show clear file-transfer direction, filename, progress, and completion/failure state.
- Do not store session passwords or secrets in source control or logs.
- Use upstream cryptographic/session mechanisms; do not invent custom encryption.
- Keep dependencies and upstream revisions pinned and review security advisories before releases.

## Voice implementation

First inspect the audio capture/playback and session negotiation already present in the selected RustDesk revision. Verify microphone and speaker permissions, device selection, mute controls, reconnect behavior, and echo/feedback handling. Only consider a separate voice transport if the existing implementation cannot meet the product requirements and a documented design review approves it.

## Deployment boundary

GitHub stores source code and project documentation. It does not provide the live rendezvous/relay infrastructure. Public service operation requires separately provisioned hosting, DNS, firewall configuration, monitoring, updates, and bandwidth.
