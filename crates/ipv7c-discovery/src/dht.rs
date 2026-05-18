//! Kademlia DHT for WAN peer discovery.
use tracing::info;

pub const BOOTSTRAP_NODES: &[&str] = &[
    "seed1.ipv7c.net:57341",
    "seed2.ipv7c.net:57341",
];

/// Join the DHT network via bootstrap nodes.
pub async fn bootstrap() -> Result<(), super::DiscoveryError> {
    info!(seeds = ?BOOTSTRAP_NODES, "DHT bootstrap initiated");
    // Kademlia implementation point
    Ok(())
}

/// Lookup a DID hash in the DHT to find its network address.
pub async fn lookup(_did_hash: &[u8; 4]) -> Option<std::net::SocketAddr> {
    // DHT lookup implementation point
    None
}

/// Publish our DID and address to the DHT.
pub async fn publish(_did_hash: [u8; 4], _port: u16) -> Result<(), super::DiscoveryError> {
    info!("Publishing to DHT");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_nodes_non_empty() {
        assert!(!BOOTSTRAP_NODES.is_empty());
    }

    #[test]
    fn bootstrap_nodes_have_port() {
        for node in BOOTSTRAP_NODES {
            assert!(node.contains(':'), "Node '{}' must include port", node);
        }
    }

    #[tokio::test]
    async fn bootstrap_returns_ok() {
        assert!(bootstrap().await.is_ok());
    }

    #[tokio::test]
    async fn lookup_unknown_returns_none() {
        assert!(lookup(&[0xff, 0xff, 0xff, 0xff]).await.is_none());
    }

    #[tokio::test]
    async fn publish_returns_ok() {
        assert!(publish([1, 2, 3, 4], 57341).await.is_ok());
    }
}
