//! Ed25519 signing keypairs for node identity.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use zeroize::Zeroize;

use crate::CryptoError;

/// Generate a new Ed25519 signing keypair.
pub fn generate_signing_keypair() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

/// Sign a message with an Ed25519 signing key.
pub fn sign(signing_key: &SigningKey, message: &[u8]) -> Signature {
    signing_key.sign(message)
}

/// Verify a signature against a verifying (public) key.
pub fn verify(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Result<(), CryptoError> {
    verifying_key
        .verify(message, signature)
        .map_err(|_| CryptoError::SignatureInvalid)
}

/// Export a signing key as raw bytes (32 bytes).
pub fn export_signing_key(key: &SigningKey) -> [u8; 32] {
    key.to_bytes()
}

/// Import a signing key from raw bytes.
pub fn import_signing_key(bytes: &[u8; 32]) -> SigningKey {
    SigningKey::from_bytes(bytes)
}

/// Securely erase a key from memory.
pub fn zeroize_key(bytes: &mut [u8]) {
    bytes.zeroize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify() {
        let sk = generate_signing_keypair();
        let vk = sk.verifying_key();
        let msg = b"sovereign message";

        let sig = sign(&sk, msg);
        assert!(verify(&vk, msg, &sig).is_ok());
    }

    #[test]
    fn verify_wrong_message_fails() {
        let sk = generate_signing_keypair();
        let vk = sk.verifying_key();

        let sig = sign(&sk, b"original");
        assert!(verify(&vk, b"tampered", &sig).is_err());
    }

    #[test]
    fn export_import_roundtrip() {
        let sk = generate_signing_keypair();
        let bytes = export_signing_key(&sk);
        let restored = import_signing_key(&bytes);
        assert_eq!(sk.to_bytes(), restored.to_bytes());
    }
}
