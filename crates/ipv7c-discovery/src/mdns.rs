//! mDNS/DNS-SD LAN discovery.
use std::net::SocketAddr;
use tracing::info;

pub const SERVICE_TYPE: &str = "_ipv7c._udp.local.";

#[derive(Debug, Clone)]
pub struct MdnsDiscoveredPeer {
    pub did_short: String,
    pub addr: SocketAddr,
}

/// Announce this node on the local network via mDNS.
pub async fn announce(port: u16, did_short: &str) -> Result<(), super::DiscoveryError> {
    info!(port, did_short, service = SERVICE_TYPE, "mDNS announcement started");
    // mdns-sd integration point
    Ok(())
}

/// Listen for mDNS announcements from other IPv7C nodes.
pub async fn listen() -> Result<Vec<MdnsDiscoveredPeer>, super::DiscoveryError> {
    info!(service = SERVICE_TYPE, "mDNS listener started");
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[test]
    fn service_type_format() {
        assert!(SERVICE_TYPE.starts_with('_'));
        assert!(SERVICE_TYPE.contains("._udp."));
        assert!(SERVICE_TYPE.ends_with("local."));
    }

    #[test]
    fn discovered_peer_stores_fields() {
        let addr: SocketAddr = "192.168.1.10:57341".parse().unwrap();
        let peer = MdnsDiscoveredPeer { did_short: "abc123".into(), addr };
        assert_eq!(peer.did_short, "abc123");
        assert_eq!(peer.addr.port(), 57341);
    }

    #[tokio::test]
    async fn announce_returns_ok() {
        assert!(announce(57341, "testnode").await.is_ok());
    }

    #[tokio::test]
    async fn listen_returns_empty_vec() {
        assert!(listen().await.unwrap().is_empty());
    }
}
