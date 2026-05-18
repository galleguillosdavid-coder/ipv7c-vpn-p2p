//! Kademlia DHT for WAN peer discovery.
use tracing::info;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Kademlia Protocol Messages
#[derive(Debug, Clone)]
pub enum KademliaMessage {
    Ping([u8; 4]),
    Store([u8; 4], SocketAddr),
    FindNode([u8; 4]),
    FindValue([u8; 4]),
}

/// Simulated response for Kademlia
pub async fn handle_kademlia_message(msg: KademliaMessage) {
    match msg {
        KademliaMessage::Ping(id) => info!("Received PING from {:?}", id),
        KademliaMessage::Store(id, addr) => {
            let _ = publish(id, addr.port()).await;
        }
        KademliaMessage::FindNode(id) => info!("Received FIND_NODE for {:?}", id),
        KademliaMessage::FindValue(id) => {
            let _ = lookup(&id).await;
        }
    }
}


pub const BOOTSTRAP_NODES: &[&str] = &[
    "seed1.ipv7c.net:57341",
    "seed2.ipv7c.net:57341",
];

use std::sync::OnceLock;

static KADEMLIA_TABLE: OnceLock<Arc<RwLock<HashMap<[u8; 4], SocketAddr>>>> = OnceLock::new();

fn get_table() -> Arc<RwLock<HashMap<[u8; 4], SocketAddr>>> {
    KADEMLIA_TABLE.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))).clone()
}

/// Join the DHT network via bootstrap nodes.
pub async fn bootstrap() -> Result<(), super::DiscoveryError> {
    info!(seeds = ?BOOTSTRAP_NODES, "DHT bootstrap initiated");
    let table_lock = get_table();
    let mut table = table_lock.write().await;
    // Mock bootstrap
    for node in BOOTSTRAP_NODES {
        if let Ok(addr) = node.parse() {
            table.insert([0, 0, 0, 0], addr); // Placeholder for seed node ID
        }
    }
    Ok(())
}

/// Lookup a DID hash in the DHT to find its network address.
pub async fn lookup(did_hash: &[u8; 4]) -> Option<SocketAddr> {
    let table_lock = get_table();
    let table = table_lock.read().await;
    table.get(did_hash).copied()
}

/// Publish our DID and address to the DHT.
pub async fn publish(did_hash: [u8; 4], port: u16) -> Result<(), super::DiscoveryError> {
    info!("Publishing to DHT");
    let table_lock = get_table();
    let mut table = table_lock.write().await;
    // In a real DHT, this would send STORE requests to closest nodes.
    // For now, we store locally in the mock table.
    table.insert(did_hash, format!("127.0.0.1:{}", port).parse().unwrap());
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
