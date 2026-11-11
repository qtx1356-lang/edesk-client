//! Network checks for eDesk's same-local-subnet connection policy.

use std::net::IpAddr;
#[cfg(not(target_os = "ios"))]
use std::net::Ipv4Addr;

/// Accept only RFC1918 IPv4 addresses that share a configured local IPv4 subnet.
///
/// This is a network boundary check, not peer authentication. The normal RustDesk
/// identity and session authentication still run after the direct TCP connection.
#[cfg(not(target_os = "ios"))]
pub(crate) fn is_allowed_lan_ip(ip: IpAddr) -> bool {
    let peer = match ip {
        IpAddr::V4(peer) => peer,
        IpAddr::V6(_) => return false,
    };

    if !peer.is_private() || peer.is_loopback() || is_ipv4_link_local(peer) {
        return false;
    }

    let peer_bits = u32::from(peer);
    default_net::get_interfaces().iter().any(|interface| {
        interface.ipv4.iter().any(|local| {
            let local_ip: Ipv4Addr = local.addr;
            let mask = u32::from(local.netmask);
            mask != 0
                && local_ip.is_private()
                && !local_ip.is_loopback()
                && !is_ipv4_link_local(local_ip)
                && (u32::from(local_ip) & mask) == (peer_bits & mask)
        })
    })
}

#[cfg(not(target_os = "ios"))]
fn is_ipv4_link_local(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    octets[0] == 169 && octets[1] == 254
}

#[cfg(target_os = "ios")]
pub(crate) fn is_allowed_lan_ip(_ip: IpAddr) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_private_and_non_ipv4_addresses() {
        assert!(!is_allowed_lan_ip("8.8.8.8".parse().unwrap()));
        assert!(!is_allowed_lan_ip("100.64.0.1".parse().unwrap()));
        assert!(!is_allowed_lan_ip("2001:4860:4860::8888".parse().unwrap()));
        assert!(!is_allowed_lan_ip("127.0.0.1".parse().unwrap()));
        assert!(!is_allowed_lan_ip("169.254.1.2".parse().unwrap()));
    }
}
