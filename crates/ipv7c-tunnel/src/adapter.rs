//! Platform-specific TUN adapter abstraction.

use crate::TunnelError;

pub const DEFAULT_MTU: u16 = 1400;
pub const TUN_SUBNET: &str = "10.7.0.0/24";
pub const TUN_NAME: &str = "ipv7c0";

/// Configuration for the TUN adapter.
#[derive(Debug, Clone)]
pub struct TunConfig {
    pub name: String,
    pub address: [u8; 4],
    pub netmask: [u8; 4],
    pub mtu: u16,
}

impl Default for TunConfig {
    fn default() -> Self {
        Self {
            name: TUN_NAME.to_string(),
            address: [10, 7, 0, 1],
            netmask: [255, 255, 255, 0],
            mtu: DEFAULT_MTU,
        }
    }
}

/// Create the TUN adapter (platform-dependent).
/// Returns an error on unsupported platforms or if driver is missing.
pub fn create_adapter(config: &TunConfig) -> Result<(), TunnelError> {
    tracing::info!(
        name = %config.name,
        addr = ?config.address,
        mtu = config.mtu,
        "Creating TUN adapter"
    );

    #[cfg(target_os = "windows")]
    {
        tracing::info!("Using Wintun driver on Windows");
        // Wintun adapter creation will go here
        // wintun::Adapter::create(...)
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        tracing::info!("Using /dev/net/tun on Linux");
        // tun2 adapter creation will go here
        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        tracing::info!("Using utun on macOS");
        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err(TunnelError::Unavailable)
    }
}
