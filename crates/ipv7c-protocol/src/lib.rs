//! # ipv7c-protocol
//!
//! Binary wire protocol, frame types, and handshake state machine.

pub mod frame;
pub mod handshake;

mod error;
pub use error::ProtocolError;
