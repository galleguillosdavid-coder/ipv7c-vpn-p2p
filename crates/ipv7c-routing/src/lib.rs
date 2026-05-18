//! # ipv7c-routing — Dynamic route table, gossip, and multi-hop relay.

pub mod table;
pub mod gossip;
pub mod score;
pub mod metrics;
pub mod ai;

mod error;
pub use error::RoutingError;
