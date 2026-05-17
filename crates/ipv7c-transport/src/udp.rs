//! Async UDP socket wrapper for the sovereign mesh.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::TransportError;

pub const DEFAULT_PORT: u16 = 57341;
pub const MAX_PACKET: usize = 65535;

/// A peer connection tracked by the UDP transport layer.
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub addr: SocketAddr,
    pub did_hash: [u8; 4],
    pub last_seen: std::time::Instant,
    pub rtt_ms: Option<f64>,
}

/// The core UDP transport node.
pub struct UdpTransport {
    socket: Arc<UdpSocket>,
    peers: Arc<RwLock<HashMap<[u8; 4], PeerConnection>>>,
    local_addr: SocketAddr,
}

impl UdpTransport {
    /// Bind to a UDP port and create the transport.
    pub async fn bind(port: u16) -> Result<Self, TransportError> {
        let addr = format!("0.0.0.0:{port}");
        let socket = UdpSocket::bind(&addr).await?;
        let local_addr = socket.local_addr()?;
        info!(%local_addr, "UDP transport bound");
        Ok(Self {
            socket: Arc::new(socket),
            peers: Arc::new(RwLock::new(HashMap::new())),
            local_addr,
        })
    }

    /// Send raw bytes to a peer by socket address.
    pub async fn send_to(&self, data: &[u8], addr: SocketAddr) -> Result<usize, TransportError> {
        let n = self.socket.send_to(data, addr).await?;
        Ok(n)
    }

    /// Receive raw bytes. Returns (bytes_read, sender_address).
    pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), TransportError> {
        let (n, addr) = self.socket.recv_from(buf).await?;
        Ok((n, addr))
    }

    /// Register or update a peer connection.
    pub async fn upsert_peer(&self, did_hash: [u8; 4], addr: SocketAddr) {
        let mut peers = self.peers.write().await;
        let entry = peers.entry(did_hash).or_insert(PeerConnection {
            addr,
            did_hash,
            last_seen: std::time::Instant::now(),
            rtt_ms: None,
        });
        entry.addr = addr;
        entry.last_seen = std::time::Instant::now();
        debug!(?did_hash, %addr, "Peer upserted");
    }

    /// Get a peer's address by DID hash.
    pub async fn peer_addr(&self, did_hash: &[u8; 4]) -> Option<SocketAddr> {
        self.peers.read().await.get(did_hash).map(|p| p.addr)
    }

    /// Remove stale peers (not seen in `max_age` seconds).
    pub async fn prune_stale(&self, max_age_secs: u64) -> usize {
        let mut peers = self.peers.write().await;
        let before = peers.len();
        peers.retain(|_, p| p.last_seen.elapsed().as_secs() < max_age_secs);
        let pruned = before - peers.len();
        if pruned > 0 { warn!(pruned, "Pruned stale peers"); }
        pruned
    }

    /// Number of tracked peers.
    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }

    /// Get a snapshot of all tracked peer connections.
    pub async fn tracked_peers(&self) -> Vec<PeerConnection> {
        self.peers.read().await.values().cloned().collect()
    }

    /// Local bound address.
    pub fn local_addr(&self) -> SocketAddr { self.local_addr }

    /// Get a clone of the socket Arc for spawning tasks.
    pub fn socket(&self) -> Arc<UdpSocket> { self.socket.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bind_and_echo() {
        let server = UdpTransport::bind(0).await.unwrap();
        let client = UdpTransport::bind(0).await.unwrap();

        // Use 127.0.0.1 explicitly since 0.0.0.0 is not a valid send target on Windows
        let server_addr: SocketAddr = format!("127.0.0.1:{}", server.local_addr().port()).parse().unwrap();
        let srv = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let (n, from) = server.recv_from(&mut buf).await.unwrap();
            server.send_to(&buf[..n], from).await.unwrap();
        });

        client.send_to(b"ping", server_addr).await.unwrap();
        let mut buf = [0u8; 1024];
        let (n, _) = client.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"ping");
        srv.await.unwrap();
    }

    #[tokio::test]
    async fn peer_tracking() {
        let t = UdpTransport::bind(0).await.unwrap();
        let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();
        t.upsert_peer([1,2,3,4], addr).await;
        assert_eq!(t.peer_count().await, 1);
        assert_eq!(t.peer_addr(&[1,2,3,4]).await, Some(addr));
    }
}
