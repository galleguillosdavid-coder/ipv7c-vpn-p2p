use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
    #[error("Connection timeout to {0}")]
    Timeout(String),
    #[error("Send failed: buffer full")]
    BufferFull,
}
