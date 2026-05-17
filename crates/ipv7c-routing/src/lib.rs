//! # ipv7c-routing — Dynamic route table, gossip, and multi-hop relay.

pub mod table;
pub mod gossip;

mod error;
pub use error::RoutingError;
