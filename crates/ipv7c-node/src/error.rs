use thiserror::Error;
#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Config error: {0}")]
    Config(#[from] ipv7c_config::ConfigError),
    #[error("Transport error: {0}")]
    Transport(#[from] ipv7c_transport::TransportError),
    #[error("Tunnel error: {0}")]
    Tunnel(#[from] ipv7c_tunnel::TunnelError),
    #[error("Identity error: {0}")]
    Identity(#[from] ipv7c_identity::IdentityError),
    #[error("Discovery error: {0}")]
    Discovery(#[from] ipv7c_discovery::DiscoveryError),
    #[error("Node is shutting down")]
    ShuttingDown,
}
