//! Hybrid key exchange: X25519 (classical) + ML-KEM-768 (post-quantum).

use ml_kem::{EncodedSizeUser, KemCore, MlKem768};
use rand::rngs::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey, SharedSecret};
use zeroize::Zeroize;


/// Ephemeral X25519 keypair for classical DH.
pub struct X25519Keypair {
    pub secret: EphemeralSecret,
    pub public: X25519PublicKey,
}

impl X25519Keypair {
    pub fn generate() -> Self {
        let secret = EphemeralSecret::random_from_rng(OsRng);
        let public = X25519PublicKey::from(&secret);
        Self { secret, public }
    }

    pub fn diffie_hellman(self, peer_public: &X25519PublicKey) -> SharedSecret {
        self.secret.diffie_hellman(peer_public)
    }
}

/// ML-KEM-768 keypair generation returning raw byte vectors for portability.
pub fn mlkem_generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let (dk, ek) = MlKem768::generate(&mut OsRng);
    let dk_bytes: Vec<u8> = dk.as_bytes().to_vec();
    let ek_bytes: Vec<u8> = ek.as_bytes().to_vec();
    (dk_bytes, ek_bytes)
}

/// Result of a full hybrid key exchange — combined shared secret.
pub struct HybridSharedSecret {
    pub combined: Vec<u8>,
}

impl Drop for HybridSharedSecret {
    fn drop(&mut self) {
        self.combined.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x25519_shared_secret_matches() {
        let alice = X25519Keypair::generate();
        let bob = X25519Keypair::generate();
        let alice_pub = alice.public;
        let bob_pub = bob.public;
        let s1 = alice.diffie_hellman(&bob_pub);
        let s2 = bob.diffie_hellman(&alice_pub);
        assert_eq!(s1.as_bytes(), s2.as_bytes());
    }

    #[test]
    fn mlkem_keypair_generates() {
        let (dk, ek) = mlkem_generate_keypair();
        assert!(!dk.is_empty());
        assert!(!ek.is_empty());
    }
}
