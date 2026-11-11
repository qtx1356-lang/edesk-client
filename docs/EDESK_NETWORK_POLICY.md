# eDesk Same-LAN-Only Connection Policy

**Status:** Prototype connection gates committed to `edesk-work`; not yet validated for production.

## Goal

eDesk must support remote sessions only when both Windows computers can discover and reach one another on the same local area network (LAN). A connection across separate home/office networks or across the public internet must be rejected.

## User-facing behavior

When the peer cannot be verified as reachable on the same LAN, stop before starting a session and show:

**Connection blocked**

You are on a different network.

Please connect both computers to the same Wi-Fi or wired LAN network and try again.

If the network cannot be verified, use the same message with a short explanation that local-network verification failed.

## Connection policy

1. Use local-network discovery (for example, a LAN-scoped UDP broadcast or multicast discovery exchange) to find the peer.
2. Include a fresh random challenge/nonce in the discovery handshake. The peer must answer the challenge over a direct connection to its private LAN address.
3. Verify that the direct peer connection is reachable through the local interface and that the discovery response matches the expected peer identity. Do not treat matching public IP addresses, SSIDs, or subnet strings alone as proof.
4. Establish the remote session directly over the LAN using the existing RustDesk session protocol and authentication.
5. Disable internet rendezvous and relay fallback for eDesk sessions. Do not silently retry through hbbs/hbbr or another public relay.
6. If local discovery, direct reachability, identity verification, or policy checks fail, deny the session and show the user-facing message.
7. Re-check the local path when reconnecting or when the network interface changes. A session must end or be re-authorized if it loses the verified LAN path.

## Default scope

- Same Wi-Fi network: allowed when local discovery and direct peer verification succeed.
- Same wired LAN: allowed when local discovery and direct peer verification succeed.
- Different routers / separate networks: blocked.
- Public internet connection: blocked.
- VPN / overlay network: blocked by default. Reconsider only if a future explicit setting and threat review define safe behavior.
- Guest Wi-Fi, client isolation, VLAN separation, or firewall rules that prevent peer discovery/direct access: blocked, even if both devices use the same router.

## Security requirements

- LAN discovery is not authentication. Retain RustDesk's peer authentication and session encryption.
- Never rely only on public IP equality, SSID equality, or matching private subnet prefixes.
- Use time-limited nonces to reduce replay risk.
- Validate peer identity before enabling screen, input, file transfer, or audio.
- Do not expose a listening service to public interfaces unnecessarily; bind discovery and local connection listeners to eligible private interfaces where practical.
- Keep local approval, visible session status, and disconnect controls mandatory.
- Do not start microphone capture until the user explicitly enables voice.
- Fail closed: uncertainty means deny, not fall back to an internet route.

## Acceptance tests

1. Two PCs on the same ordinary Wi-Fi can discover and connect directly.
2. Two PCs on the same wired LAN can discover and connect directly.
3. Two PCs on different internet connections cannot connect, even if their public IPs happen to be shared by carrier-grade NAT.
4. A device ID alone cannot establish a remote internet session.
5. If LAN discovery is blocked by guest-network/client isolation, eDesk denies the connection with the popup.
6. If the LAN path drops during a session, eDesk disconnects or requires fresh local verification before reconnecting.
7. A spoofed or replayed discovery response does not bypass peer authentication.
8. Screen, input, file transfer, and voice are all unavailable until the LAN policy and user-approval checks pass.

## Current implementation state

The current eDesk branch now has an initial connection-layer gate:

- Outgoing connections are resolved only from the LAN discovery cache. An ID by itself no longer proceeds to the normal rendezvous path.
- A discovered peer is contacted directly over TCP only when its IPv4 address is private and matches a private IPv4 subnet on a local interface.
- Incoming TCP sessions are rejected unless the source address passes the same private-subnet check.
- Relay requests and relay-connection creation fail closed.
- All rendezvous punch requests are rejected; eDesk does not establish sessions through WebRTC/ICE, IPv6 punch, TURN, or public rendezvous coordination.
- The LAN discovery responder is started in portable desktop mode as well as installed-service mode, and discovery requests carry a fresh UUID nonce that must be echoed in the response.
- Discovery responses are served only to sources that pass the local private-subnet check.
- LAN discovery broadcasts and subnet validation now ignore adapters reported as down, loopback, point-to-point, or tunnel interfaces, reducing accidental use of VPN/tunnel adapters.

The app may still contact the configured rendezvous service for device registration/status, but that path cannot establish or carry a remote session. This is a **first enforcement layer, not final LAN verification**. The nonce reduces acceptance of stale/replayed discovery responses, but does not authenticate the responder or cryptographically prove the peer is on the same Layer-2 broadcast domain, and does not revalidate an active session after network-interface changes. A routed private network or VPN that forwards LAN broadcasts may still require additional controls. Do not describe this build as production-secure until the remaining checks and acceptance tests pass.

## Implementation note

Keep policy enforcement at the connection-establishment/routing layer, not only in the UI. Continue with a challenge-response discovery handshake, explicit transport-path verification, active-session revalidation, and automated tests for same-LAN, routed private networks, VPN/overlay, guest Wi-Fi, and relay rejection. Preserve RustDesk peer authentication and encryption throughout.
