//! # ipv7c-config — Zero-config and TOML configuration.
pub mod defaults;
mod error;
pub use error::ConfigError;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub listen_port: u16,
    pub max_peers: usize,
    pub log_level: String,
    pub tunnel: TunnelConfig,
    pub crypto: CryptoConfig,
    pub discovery: DiscoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    pub mtu: u16,
    pub dns: String,
    pub split_tunnel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    pub key_rotation_secs: u64,
    pub pqc_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub mdns_enabled: bool,
    pub dht_enabled: bool,
    pub bootstrap_nodes: Vec<String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            listen_port: 57341, max_peers: 64, log_level: "info".into(),
            tunnel: TunnelConfig { mtu: 1400, dns: "10.7.0.53".into(), split_tunnel: false },
            crypto: CryptoConfig { key_rotation_secs: 600, pqc_enabled: true },
            discovery: DiscoveryConfig {
                mdns_enabled: true, dht_enabled: true,
                bootstrap_nodes: vec!["seed1.ipv7c.net:57341".into(), "seed2.ipv7c.net:57341".into()],
            },
        }
    }
}

impl NodeConfig {
    pub fn load_or_default() -> Self {
        let path = defaults::config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = defaults::config_path();
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).ok(); }
        let content = toml::to_string_pretty(self).map_err(|e| ConfigError::Serialization(e.to_string()))?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
