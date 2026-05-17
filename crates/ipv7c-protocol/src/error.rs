use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Frame too short: got {got} bytes, minimum is {minimum}")]
    FrameTooShort { got: usize, minimum: usize },
    #[error("Unknown frame type: 0x{0:02x}")]
    UnknownFrameType(u8),
    #[error("Protocol version mismatch: expected {expected}, got {got}")]
    VersionMismatch { expected: u8, got: u8 },
    #[error("Payload exceeds maximum size")]
    PayloadTooLarge,
    #[error("Invalid handshake state transition")]
    InvalidHandshakeState,
    #[error("Compression error: {0}")]
    Compression(String),
}
