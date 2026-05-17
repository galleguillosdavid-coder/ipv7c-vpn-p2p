//! 3-way handshake state machine: INIT → CHALLENGE → CONFIRM

use crate::ProtocolError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    Idle,
    InitSent,
    ChallengeReceived,
    Confirmed,
    Failed,
}

pub struct HandshakeFsm {
    pub state: HandshakeState,
}

impl HandshakeFsm {
    pub fn new() -> Self { Self { state: HandshakeState::Idle } }

    pub fn send_init(&mut self) -> Result<(), ProtocolError> {
        if self.state != HandshakeState::Idle {
            return Err(ProtocolError::InvalidHandshakeState);
        }
        self.state = HandshakeState::InitSent;
        Ok(())
    }

    pub fn receive_challenge(&mut self) -> Result<(), ProtocolError> {
        if self.state != HandshakeState::InitSent {
            return Err(ProtocolError::InvalidHandshakeState);
        }
        self.state = HandshakeState::ChallengeReceived;
        Ok(())
    }

    pub fn confirm(&mut self) -> Result<(), ProtocolError> {
        if self.state != HandshakeState::ChallengeReceived {
            return Err(ProtocolError::InvalidHandshakeState);
        }
        self.state = HandshakeState::Confirmed;
        Ok(())
    }

    pub fn fail(&mut self) { self.state = HandshakeState::Failed; }
    pub fn is_complete(&self) -> bool { self.state == HandshakeState::Confirmed }
}

impl Default for HandshakeFsm {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let mut fsm = HandshakeFsm::new();
        fsm.send_init().unwrap();
        fsm.receive_challenge().unwrap();
        fsm.confirm().unwrap();
        assert!(fsm.is_complete());
    }

    #[test]
    fn invalid_transition() {
        let mut fsm = HandshakeFsm::new();
        assert!(fsm.confirm().is_err());
    }
}
