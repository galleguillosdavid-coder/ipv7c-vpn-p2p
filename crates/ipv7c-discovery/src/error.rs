use thiserror::Error;
#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("mDNS unavailable: {0}")]
    MdnsUnavailable(String),
    #[error("DHT bootstrap failed: no reachable seed nodes")]
    DhtBootstrapFailed,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
