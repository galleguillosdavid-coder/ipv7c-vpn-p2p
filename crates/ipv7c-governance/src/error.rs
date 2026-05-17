use thiserror::Error;
#[derive(Debug, Error)]
pub enum GovernanceError {
    #[error("Peer {0} is banned")]
    PeerBanned(String),
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}
