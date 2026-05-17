//! Platform defaults and paths.
use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    { PathBuf::from(std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into())).join(".ipv7c") }
    #[cfg(not(target_os = "windows"))]
    { PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".ipv7c") }
}

pub fn config_path() -> PathBuf { data_dir().join("config.toml") }
pub fn wallet_path() -> PathBuf { data_dir().join("wallet.db") }
pub fn log_dir() -> PathBuf { data_dir().join("logs") }
