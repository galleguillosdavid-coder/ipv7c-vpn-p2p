use thiserror::Error;
#[derive(Debug, Error)]
pub enum RoutingError {
    #[error("No route to destination {0:?}")]
    NoRoute([u8; 4]),
    #[error("Route expired for {0:?}")]
    RouteExpired([u8; 4]),
    #[error("TTL exceeded")]
    TtlExceeded,
    #[error("Transport error: {0}")]
    Transport(#[from] ipv7c_transport::TransportError),
}
