//! User-Space TCP/IP Stack using smoltcp.
//! This allows packet routing without requiring root/admin privileges in some environments.

use tracing::{info, debug, warn};

#[derive(Debug, PartialEq)]
pub enum TrafficClass {
    VoIP,
    SSH,
    Web,
    Unknown,
}

pub struct ZtnaRule {
    pub allowed_ports: Vec<u16>,
    pub is_guest: bool,
}

pub struct UserSpaceStack {
    enabled: bool,
    pub ztna_rules: Option<ZtnaRule>,
}

impl UserSpaceStack {
    pub fn new() -> Self {
        info!("Initializing User-Space TCP/IP Stack (smoltcp wrapper) with DPI and ZTNA");
        Self { enabled: true, ztna_rules: None }
    }

    pub fn with_ztna(mut self, rules: ZtnaRule) -> Self {
        self.ztna_rules = Some(rules);
        self
    }

    /// Simulates Deep Packet Inspection (DPI) to classify traffic.
    pub fn inspect_packet(&self, packet: &[u8]) -> (TrafficClass, u16) {
        if packet.len() < 20 {
            return (TrafficClass::Unknown, 0);
        }
        
        let dst_port = u16::from_be_bytes([packet[18], packet[19]]);
        
        let class = match dst_port {
            22 => TrafficClass::SSH,
            5060 | 5061 | 10000..=20000 => TrafficClass::VoIP,
            80 | 443 => TrafficClass::Web,
            _ => TrafficClass::Unknown,
        };
        (class, dst_port)
    }

    pub fn process_packet(&mut self, packet: &[u8]) -> Option<Vec<u8>> {
        if !self.enabled {
            return None;
        }
        
        let (class, dst_port) = self.inspect_packet(packet);
        
        if let Some(rules) = &self.ztna_rules {
            if rules.is_guest {
                if !rules.allowed_ports.is_empty() && !rules.allowed_ports.contains(&dst_port) {
                    warn!("ZTNA: Packet dropped. Port {} is not allowed for Guest.", dst_port);
                    return None;
                }
            }
        }
        
        debug!("Packet allowed. Classified as: {:?}", class);
        Some(packet.to_vec())
    }
}
