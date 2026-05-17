use thiserror::Error;
#[derive(Debug, Error)]
pub enum TunnelError {
    #[error("Failed to create TUN adapter: {0}")]
    AdapterCreation(String),
    #[error("TUN read error: {0}")]
    Read(#[from] std::io::Error),
    #[error("TUN not available on this platform")]
    Unavailable,
}
