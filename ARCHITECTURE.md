# eDesk architecture

## Windows client
Use a branded RustDesk fork. Retain upstream screen capture, encoding, input, transport, NAT traversal, and encryption components where possible.

## Connection services
- hbbs: registration, rendezvous, and signalling
- hbbr: relay traffic when direct peer-to-peer connectivity is unavailable

Self-hosting the OSS server is free software-wise, but requires a public server, bandwidth, maintenance, and monitoring.

## File transfer
Prefer the upstream transfer implementation and protocol. Test permissions, large files, progress, cancellation, and interrupted transfers.

## Voice
First inspect and test audio capabilities in the pinned upstream version. Do not assume a separate WebRTC stack can be inserted without protocol and UI integration work.

## Safety baseline
Incoming interactive sessions require local approval by default. Show a persistent active-session indicator and a disconnect action. Unattended access must be opt-in, authenticated, and revocable. Do not implement hidden persistence or silent remote control.
