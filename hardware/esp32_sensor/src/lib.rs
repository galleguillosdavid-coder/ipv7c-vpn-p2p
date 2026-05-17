#![no_std]

use core::panic::PanicInfo;

// ============================================================
// IPv7C HARDWARE SENSOR NODE - Protocolo SOS Bare Metal
// Objetivo: microcontroladores (ESP32, RP2040) sin SO
// ============================================================

/// Magic byte del protocolo IPv7C
pub const IPV7C_MAGIC: u8 = 0x07;

/// Frame SOS ultraligero (max 16 bytes < MTU 512B del modo sparse)
/// Layout:
///  [0] = magic IPV7C (0x07)
///  [1] = flags (0x01 = SOS mode)
///  [2] = node_id (low byte)
///  [3] = node_id (high byte)
///  [4..5] = sensor_value (u16 little-endian)
///  [6] = transporte (0=BLE, 1=LoRa, 2=UDP)
///  [7..14] = reservado
///  [15] = checksum XOR simple de bytes 0..14
#[derive(Debug, Copy, Clone)]
pub struct SosFrame {
    pub node_id: u16,
    pub sensor_value: u16,
    pub transport: u8,
}

impl SosFrame {
    /// Serializa el frame en un buffer de bytes listo para transmisión
    pub fn encode(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0] = IPV7C_MAGIC;
        buf[1] = 0x01; // SOS mode flag
        buf[2] = (self.node_id & 0xFF) as u8;
        buf[3] = (self.node_id >> 8) as u8;
        buf[4] = (self.sensor_value & 0xFF) as u8;
        buf[5] = (self.sensor_value >> 8) as u8;
        buf[6] = self.transport;

        // Checksum XOR simple
        let mut chk: u8 = 0;
        for i in 0..15 {
            chk ^= buf[i];
        }
        buf[15] = chk;
        buf
    }

    /// Decodifica un buffer recibido a SosFrame, verifica checksum
    pub fn decode(buf: &[u8; 16]) -> Option<Self> {
        if buf[0] != IPV7C_MAGIC {
            return None;
        }
        // Verificar checksum
        let mut chk: u8 = 0;
        for i in 0..15 {
            chk ^= buf[i];
        }
        if chk != buf[15] {
            return None;
        }
        Some(SosFrame {
            node_id: (buf[3] as u16) << 8 | buf[2] as u16,
            sensor_value: (buf[5] as u16) << 8 | buf[4] as u16,
            transport: buf[6],
        })
    }
}

/// Determina si el payload cumple con el MTU del modo SOS (<= 512 bytes)
/// Ref: ipv7c.py sos_mode_active MTU enforcement
#[inline(always)]
pub fn is_sos_compliant(payload_size: usize) -> bool {
    payload_size <= 512
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // En hardware real aquí se activaría un LED de error o watchdog reset
    loop {}
}
