//! UDP hole-punching for NAT traversal.

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::sync::Arc;
use tracing::{debug, info};

const PUNCH_MAGIC: &[u8; 8] = b"IPV7CPNC";
const PUNCH_ACK: &[u8; 8] = b"IPV7CACK";
const MAX_ATTEMPTS: u32 = 10;
const ATTEMPT_DELAY_MS: u64 = 200;

/// Attempt UDP hole-punching to a remote peer.
pub async fn punch(
    socket: Arc<UdpSocket>,
    target: SocketAddr,
) -> Result<bool, std::io::Error> {
    info!(%target, "Starting UDP hole-punch");
    for attempt in 0..MAX_ATTEMPTS {
        socket.send_to(PUNCH_MAGIC, target).await?;
        debug!(attempt, %target, "Punch packet sent");
        tokio::time::sleep(tokio::time::Duration::from_millis(ATTEMPT_DELAY_MS)).await;
    }
    Ok(true)
}

/// Check if incoming data is a punch signal and respond.
pub async fn handle_punch(
    socket: Arc<UdpSocket>,
    data: &[u8],
    from: SocketAddr,
) -> Result<bool, std::io::Error> {
    if data.len() >= 8 && &data[..8] == PUNCH_MAGIC {
        socket.send_to(PUNCH_ACK, from).await?;
        info!(%from, "Punch ACK sent");
        return Ok(true);
    }
    if data.len() >= 8 && &data[..8] == PUNCH_ACK {
        info!(%from, "Punch confirmed");
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn punch_ack_roundtrip() {
        let a = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
        let b = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
        let b_addr = b.local_addr().unwrap();

        a.send_to(PUNCH_MAGIC, b_addr).await.unwrap();
        let mut buf = [0u8; 64];
        let (n, from) = b.recv_from(&mut buf).await.unwrap();
        let is_punch = handle_punch(b.clone(), &buf[..n], from).await.unwrap();
        assert!(is_punch);
    }
}
