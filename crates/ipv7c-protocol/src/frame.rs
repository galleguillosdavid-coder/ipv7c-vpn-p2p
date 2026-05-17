//! Binary wire protocol frame format.

use crate::ProtocolError;

pub const PROTOCOL_VERSION: u8 = 1;
pub const HEADER_SIZE: usize = 32;
pub const MAX_PAYLOAD: usize = 65535;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    Data = 0x01,
    Control = 0x02,
    Gossip = 0x03,
    Punch = 0x04,
    Heartbeat = 0x05,
    Handshake = 0x06,
    Governance = 0x07,
}

impl TryFrom<u8> for FrameType {
    type Error = ProtocolError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x01 => Ok(Self::Data),
            0x02 => Ok(Self::Control),
            0x03 => Ok(Self::Gossip),
            0x04 => Ok(Self::Punch),
            0x05 => Ok(Self::Heartbeat),
            0x06 => Ok(Self::Handshake),
            0x07 => Ok(Self::Governance),
            other => Err(ProtocolError::UnknownFrameType(other)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrameHeader {
    pub version: u8,
    pub frame_type: FrameType,
    pub payload_len: u16,
    pub sequence: u32,
    pub ttl: u8,
    pub flags: u8,
    pub fragment_info: u16,
    pub src_hash: [u8; 4],
    pub dst_hash: [u8; 4],
    pub nonce: [u8; 12],
}

impl FrameHeader {
    pub fn serialize(&self) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        buf[0] = self.version;
        buf[1] = self.frame_type as u8;
        buf[2..4].copy_from_slice(&self.payload_len.to_be_bytes());
        buf[4..8].copy_from_slice(&self.sequence.to_be_bytes());
        buf[8] = self.ttl;
        buf[9] = self.flags;
        buf[10..12].copy_from_slice(&self.fragment_info.to_be_bytes());
        buf[12..16].copy_from_slice(&self.src_hash);
        buf[16..20].copy_from_slice(&self.dst_hash);
        buf[20..32].copy_from_slice(&self.nonce);
        buf
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() < HEADER_SIZE {
            return Err(ProtocolError::FrameTooShort { got: buf.len(), minimum: HEADER_SIZE });
        }
        let version = buf[0];
        if version != PROTOCOL_VERSION {
            return Err(ProtocolError::VersionMismatch { expected: PROTOCOL_VERSION, got: version });
        }
        let frame_type = FrameType::try_from(buf[1])?;
        let payload_len = u16::from_be_bytes([buf[2], buf[3]]);
        let sequence = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let ttl = buf[8];
        let flags = buf[9];
        let fragment_info = u16::from_be_bytes([buf[10], buf[11]]);
        let mut src_hash = [0u8; 4];
        src_hash.copy_from_slice(&buf[12..16]);
        let mut dst_hash = [0u8; 4];
        dst_hash.copy_from_slice(&buf[16..20]);
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&buf[20..32]);

        Ok(Self { version, frame_type, payload_len, sequence, ttl, flags, fragment_info, src_hash, dst_hash, nonce })
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub header: FrameHeader,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(header: FrameHeader, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let hdr = self.header.serialize();
        let mut buf = Vec::with_capacity(HEADER_SIZE + self.payload.len());
        buf.extend_from_slice(&hdr);
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, ProtocolError> {
        let header = FrameHeader::deserialize(buf)?;
        let payload_start = HEADER_SIZE;
        let payload = buf.get(payload_start..).unwrap_or_default().to_vec();
        Ok(Self { header, payload })
    }

    pub fn heartbeat(src: [u8; 4], dst: [u8; 4], seq: u32) -> Self {
        Self {
            header: FrameHeader {
                version: PROTOCOL_VERSION,
                frame_type: FrameType::Heartbeat,
                payload_len: 1,
                sequence: seq,
                ttl: 1,
                flags: 0,
                fragment_info: 0,
                src_hash: src,
                dst_hash: dst,
                nonce: [0u8; 12],
            },
            payload: vec![0xFF],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_roundtrip() {
        let h = FrameHeader {
            version: PROTOCOL_VERSION, frame_type: FrameType::Data,
            payload_len: 1234, sequence: 42, ttl: 5, flags: 0,
            fragment_info: 0, src_hash: [1,2,3,4], dst_hash: [5,6,7,8],
            nonce: [9u8; 12],
        };
        let buf = h.serialize();
        let h2 = FrameHeader::deserialize(&buf).unwrap();
        assert_eq!(h2.payload_len, 1234);
        assert_eq!(h2.sequence, 42);
        assert_eq!(h2.frame_type, FrameType::Data);
    }

    #[test]
    fn frame_roundtrip() {
        let f = Frame::heartbeat([1,2,3,4], [5,6,7,8], 99);
        let buf = f.serialize();
        let f2 = Frame::deserialize(&buf).unwrap();
        assert_eq!(f2.header.frame_type, FrameType::Heartbeat);
        assert_eq!(f2.payload, vec![0xFF]);
    }
}
