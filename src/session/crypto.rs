use ed25519_dalek::{SigningKey, VerifyingKey};

use tyche_enclave::envelopes::{
    storage::wrap_envelope,
    transport::{KeyToUse, TransportKeyReceiver},
};

/// Result type for crypto operations.
pub type CryptoResult<T> = Result<T, CryptoError>;

/// Errors that can occur during cryptographic operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Transport key error: {0}")]
    TransportKeyError(String),
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Derivation failed")]
    DerivationFailed,
}

pub const KEY_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct EnclaveTransportKeys {
    pub ephemeral: VerifyingKey,
    pub deterministic: VerifyingKey,
    pub attestation: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct UsersEncryptionKeys {
    pub transport: SigningKey,
    pub storage: [u8; 32],
    pub meta: Option<[u8; 32]>,
}

impl UsersEncryptionKeys {
    pub fn new(transport: SigningKey, storage: [u8; 32], meta: Option<[u8; 32]>) -> Self {
        Self {
            transport,
            storage,
            meta,
        }
    }
}

/// Encrypt arbitrary blobs for transport or storage with user's encryption key.
pub fn seal_for_storage(blob: Vec<u8>, users_keys: &UsersEncryptionKeys) -> CryptoResult<Vec<u8>> {
    wrap_envelope(&users_keys.storage, None, &blob).map_err(|e| CryptoError::TransportKeyError(e.to_string()))
}

/// Seal arbitrary blobs for transport.
pub fn seal_for_transport(
    blob: Vec<u8>,
    transport_key: &TransportKeyReceiver,
    key_to_use: KeyToUse,
) -> CryptoResult<Vec<u8>> {
    transport_key
        .encrypt_envelope(blob.as_slice(), key_to_use)
        .map_err(|e| CryptoError::TransportKeyError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    #[test]
    fn test_encrypt_wallet_key_different_keys() {
        let uek = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        let private_key1 = vec![0xABu8; 32];
        let private_key2 = vec![0xCDu8; 32];

        let blob1 = seal_for_storage(private_key1, &uek).unwrap();
        let blob2 = seal_for_storage(private_key2, &uek).unwrap();

        // Different keys should produce different ciphertexts
        assert_ne!(blob1, blob2);
    }
}
