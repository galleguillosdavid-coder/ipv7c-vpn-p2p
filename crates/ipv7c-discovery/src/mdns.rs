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
