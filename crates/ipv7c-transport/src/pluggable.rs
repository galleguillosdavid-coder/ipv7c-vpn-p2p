//! Pluggable transports for out-of-band communication (BLE, LoRa, Serial)

use tracing::{info, warn, error};
use std::io::{Error, ErrorKind};

pub trait PluggableTransport {
    fn name(&self) -> &'static str;
    fn send(&self, data: &[u8]) -> Result<(), super::TransportError>;
    fn receive(&self) -> Result<Vec<u8>, super::TransportError>;
}

/// Simulated Bluetooth Mesh Transport using btleplug patterns
pub struct BluetoothTransport {
    enabled: bool,
}

impl BluetoothTransport {
    pub fn new() -> Self {
        info!("Initializing Bluetooth Mesh transport (btleplug backend)");
        Self { enabled: true }
    }
}

impl PluggableTransport for BluetoothTransport {
    fn name(&self) -> &'static str {
        "BluetoothMesh"
    }

    fn send(&self, data: &[u8]) -> Result<(), super::TransportError> {
        if !self.enabled {
            return Err(super::TransportError::Io(Error::new(ErrorKind::NotConnected, "BLE disabled")));
        }
        // Simulated: send via BLE
        info!("BLE: Broadcasting {} bytes", data.len());
        Ok(())
    }

    fn receive(&self) -> Result<Vec<u8>, super::TransportError> {
        if !self.enabled {
            return Err(super::TransportError::Io(Error::new(ErrorKind::NotConnected, "BLE disabled")));
        }
        // Simulated receive
        Ok(vec![])
    }
}

/// Simulated LoRa Hardware Transport using tokio-serial patterns
pub struct LoRaTransport {
    enabled: bool,
    serial_port: Option<String>,
}

impl LoRaTransport {
    pub fn new() -> Self {
        info!("Initializing LoRa hardware transport (tokio-serial backend)");
        Self { enabled: true, serial_port: None }
    }
    
    pub fn with_port(port: String) -> Self {
        Self { enabled: true, serial_port: Some(port) }
    }
}

impl PluggableTransport for LoRaTransport {
    fn name(&self) -> &'static str {
        "LoRa"
    }

    fn send(&self, data: &[u8]) -> Result<(), super::TransportError> {
        if !self.enabled {
            return Err(super::TransportError::Io(Error::new(ErrorKind::NotConnected, "LoRa disabled")));
        }
        if self.serial_port.is_none() {
            warn!("LoRa: No serial port configured. Packet dropped.");
            return Ok(());
        }
        // Send via LoRa serial
        info!("LoRa [{}]: Transmitting {} bytes", self.serial_port.as_ref().unwrap(), data.len());
        Ok(())
    }

    fn receive(&self) -> Result<Vec<u8>, super::TransportError> {
        if !self.enabled {
            return Err(super::TransportError::Io(Error::new(ErrorKind::NotConnected, "LoRa disabled")));
        }
        // Simulated receive
        Ok(vec![])
    }
}
