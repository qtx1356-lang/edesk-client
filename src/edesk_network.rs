//! Network checks for eDesk's same-local-subnet connection policy.

use std::net::IpAddr;
#[cfg(not(target_os = "ios"))]
use std::net::Ipv4Addr;

/// Accept only RFC1918 IPv4 addresses that share a configured local IPv4 subnet.
///
/// This is a network boundary check, not peer authentication. The normal RustDesk
/// identity and session authentication still run after the direct TCP connection.
/// Only physical or ordinary LAN adapters may participate in eDesk discovery.
/// Point-to-point and tunnel interfaces are excluded so VPN/overlay links are not
/// treated as a local Wi-Fi or wired LAN.
#[cfg(not(target_os = "ios"))]
pub(crate) fn is_allowed_lan_interface(interface: &default_net::interface::Interface) -> bool {
    interface.is_up()
        && !interface.is_loopback()
        && !interface.is_point_to_point()
        && !interface.is_tun()
}

#[cfg(not(target_os = "ios"))]
pub(crate) fn is_allowed_lan_ip(ip: IpAddr) -> bool {
    let peer = match ip {
        IpAddr::V4(peer) => peer,
        IpAddr::V6(_) => return false,
    };

    if !peer.is_private() || peer.is_loopback() || is_ipv4_link_local(peer) {
        return false;
    }

    default_net::get_interfaces().iter().any(|interface| {
        is_allowed_lan_interface(interface)
            && interface.ipv4.iter().any(|local| {
                let local_ip: Ipv4Addr = local.addr;
                is_same_private_subnet(local_ip, local.netmask, peer)
            })
    })
}

#[cfg(not(target_os = "ios"))]
fn is_same_private_subnet(local_ip: Ipv4Addr, netmask: Ipv4Addr, peer_ip: Ipv4Addr) -> bool {
    let mask = u32::from(netmask);
    mask != 0
        && local_ip.is_private()
        && !local_ip.is_loopback()
        && !is_ipv4_link_local(local_ip)
        && peer_ip.is_private()
        && !peer_ip.is_loopback()
        && !is_ipv4_link_local(peer_ip)
        && (u32::from(local_ip) & mask) == (u32::from(peer_ip) & mask)
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

#[cfg(all(test, not(target_os = "ios")))]
mod tests {
    use super::*;

    #[test]
    fn allows_only_peers_in_the_same_private_subnet() {
        assert!(is_same_private_subnet(
            "192.168.1.10".parse().unwrap(),
            "255.255.255.0".parse().unwrap(),
            "192.168.1.20".parse().unwrap(),
        ));
        assert!(!is_same_private_subnet(
            "192.168.1.10".parse().unwrap(),
            "255.255.255.0".parse().unwrap(),
            "192.168.2.20".parse().unwrap(),
        ));
        assert!(is_same_private_subnet(
            "10.20.0.10".parse().unwrap(),
            "255.255.0.0".parse().unwrap(),
            "10.20.5.20".parse().unwrap(),
        ));
        assert!(!is_same_private_subnet(
            "192.168.1.10".parse().unwrap(),
            "0.0.0.0".parse().unwrap(),
            "192.168.1.20".parse().unwrap(),
        ));
    }

    #[test]
    fn rejects_non_private_and_non_ipv4_addresses() {
        assert!(!is_allowed_lan_ip("8.8.8.8".parse().unwrap()));
        assert!(!is_allowed_lan_ip("100.64.0.1".parse().unwrap()));
        assert!(!is_allowed_lan_ip("2001:4860:4860::8888".parse().unwrap()));
        assert!(!is_allowed_lan_ip("127.0.0.1".parse().unwrap()));
        assert!(!is_allowed_lan_ip("169.254.1.2".parse().unwrap()));
    }
}
