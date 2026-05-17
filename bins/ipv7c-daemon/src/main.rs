//! IPv7C background daemon.

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .json()
        .init();

    tracing::info!("ipv7c-daemon starting");

    let config = ipv7c_config::NodeConfig::load_or_default();
    let mut node = ipv7c_node::lifecycle::SovereignNode::new(config);
    node.start().await?;

    Ok(())
}
