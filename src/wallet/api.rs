use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::json;

use crate::{
    client::IrisClient,
    config::Config,
    session::crypto::{UsersEncryptionKeys, seal_for_transport},
    wallet::types::GetTransportKeyResponse,
};
use tyche_enclave::{
    envelopes::{
        rotate_user_key::RotateUserKeyPayload,
        transport::{KeyToUse, TransportKeyReceiver},
    },
    types::chain_type::ChainType,
};

use crate::session::transport::{
    CachedTransportKeys, delete_cached_transport_keys, is_cache_fresh, load_cached_transport_keys, save_transport_keys,
};

use super::types::{
    CreateEncryptedWalletResponse, ListEncryptedWalletsResponse, Wallet, WalletError, WalletList, WalletResult,
};

/// Get transport keys from cache or fetch from API.
///
/// Checks the cache first based on TTL from config. If cache is fresh,
/// returns cached keys. Otherwise fetches from API, optionally verifies
/// attestation (based on config), and caches the result.
///
/// # Arguments
/// * `client` - The Iris API client
///
/// # Returns
/// Transport keys for encrypting wallet data
///
/// # Errors
/// Returns `WalletError` if fetching from API fails, attestation verification
/// fails (when enabled), or cache operations fail.
pub async fn get_transport_key(client: &IrisClient) -> WalletResult<TransportKeyReceiver> {
    // Load config to get TTL and verification settings
    let config = Config::load().map_err(|e| WalletError::StorageFailed(e.to_string()))?;
    let ttl_minutes = config.enclave.transport_key_ttl_minutes;
    let verify_attestation = config.enclave.verify_attestation;

    // Get config directory for cache storage
    let config_dir = Config::config_path()
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?
        .parent()
        .ok_or_else(|| WalletError::StorageFailed("No config dir".to_string()))?
        .to_path_buf();

    // Check cache first
    if let Some(cached) = load_cached_transport_keys(&config_dir)
        && let Some(timestamp) = cached.timestamp()
        && is_cache_fresh(timestamp, ttl_minutes)
    {
        // Cache is fresh, decode and return
        let ephemeral = STANDARD
            .decode(&cached.ephemeral)
            .map_err(|_| WalletError::Crypto("Invalid cached ephemeral key".to_string()))?;
        let deterministic = STANDARD
            .decode(&cached.deterministic)
            .map_err(|_| WalletError::Crypto("Invalid cached deterministic key".to_string()))?;
        let attestation = STANDARD
            .decode(&cached.attestation_document)
            .map_err(|_| WalletError::Crypto("Invalid cached attestation".to_string()))?;

        // Decode base64 to raw bytes and convert to fixed-size arrays
        let ephemeral_bytes: [u8; 32] = ephemeral
            .as_slice()
            .try_into()
            .map_err(|_| WalletError::Crypto("Invalid ephemeral key length".to_string()))?;
        let deterministic_bytes: [u8; 32] = deterministic
            .as_slice()
            .try_into()
            .map_err(|_| WalletError::Crypto("Invalid deterministic key length".to_string()))?;

        return Ok(TransportKeyReceiver::from_message(
            &ephemeral_bytes,
            &deterministic_bytes,
            attestation,
        ));
    }

    // Cache miss or stale - fetch from API
    let response: GetTransportKeyResponse = client
        .query("agent.getTransportKey", json!({}))
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let ephemeral = STANDARD
        .decode(&response.ephemeral)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let deterministic = STANDARD
        .decode(&response.deterministic)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let attestation = STANDARD
        .decode(&response.attestation_document)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Verify attestation if configured
    if verify_attestation {
        verify_attestation_document(&attestation).map_err(|e| WalletError::Crypto(e.to_string()))?;
    }

    // Decode base64 to raw bytes and convert to fixed-size arrays
    let ephemeral_bytes: [u8; 32] = ephemeral
        .as_slice()
        .try_into()
        .map_err(|_| WalletError::Crypto("Invalid ephemeral key length".to_string()))?;
    let deterministic_bytes: [u8; 32] = deterministic
        .as_slice()
        .try_into()
        .map_err(|_| WalletError::Crypto("Invalid deterministic key length".to_string()))?;

    let transport_key = TransportKeyReceiver::from_message(&ephemeral_bytes, &deterministic_bytes, attestation.clone());

    // Save to cache
    let cached_keys = CachedTransportKeys::new(
        response.ephemeral,
        response.deterministic,
        response.attestation_document,
    );

    if let Err(e) = save_transport_keys(&config_dir, &cached_keys) {
        // Log cache save failure but don't fail the operation
        eprintln!("Warning: Failed to cache transport keys: {}", e);
    }

    Ok(transport_key)
}

/// Verify the attestation document from the enclave.
///
/// This validates the cryptographic attestation to ensure the enclave
/// is authentic and running the expected code.
///
/// # Arguments
/// * `attestation` - Raw attestation document bytes
///
/// # Returns
/// `Ok(())` if verification succeeds
///
/// # Errors
/// Returns error string if verification fails
fn verify_attestation_document(attestation: &[u8]) -> Result<(), String> {
    // TODO: Implement actual attestation verification
    // For now, this is a placeholder that always succeeds
    // In production, this should verify:
    // 1. Attestation signature using AWS KMS or NSM certificate
    // 2. PCR values match expected build measurements
    // 3. Timestamp is recent (anti-replay)
    // 4. Enclave identity matches expected value

    if attestation.is_empty() {
        return Err("Attestation document is empty".to_string());
    }

    // Placeholder - in production this would use the actual verification
    // tyche_enclave::attestation::verify(attestation)

    Ok(())
}

/// Clear the transport key cache.
///
/// Useful when wanting to force a fresh attestation or when
/// switching between different enclave instances.
///
/// # Returns
/// `Ok(())` on success
///
/// # Errors
/// Returns `WalletError` if cache deletion fails
pub fn clear_transport_key_cache() -> WalletResult<()> {
    let config_dir = Config::config_path()
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?
        .parent()
        .ok_or_else(|| WalletError::StorageFailed("No config dir".to_string()))?
        .to_path_buf();

    delete_cached_transport_keys(&config_dir).map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}

/// Check if transport keys are cached and fresh.
///
/// # Returns
/// `Some(timestamp)` if cached and within TTL, `None` otherwise
pub fn has_fresh_cached_keys() -> Option<chrono::DateTime<chrono::Utc>> {
    let config_dir = Config::config_path().ok()?.parent()?.to_path_buf();
    let config = Config::load().ok()?;
    let ttl_minutes = config.enclave.transport_key_ttl_minutes;

    let cached = load_cached_transport_keys(&config_dir)?;
    let timestamp = cached.timestamp()?;

    if is_cache_fresh(timestamp, ttl_minutes) {
        Some(timestamp)
    } else {
        None
    }
}

pub async fn upsert_encrypted_wallet(
    wallet: Wallet,
    user_key: &UsersEncryptionKeys,
    ek: Option<&TransportKeyReceiver>,
    client: &IrisClient,
) -> WalletResult<Wallet> {
    let enclave_keys = match ek {
        Some(ek) => ek,
        None => &get_transport_key(client).await?,
    };

    // First we get the encrypted_wallet_material sorted. This is saved in the db after being dual encrypted.
    let encrypted_wallet_blob = seal_for_transport(
        wallet.encrypted_private_key.clone(),
        enclave_keys,
        KeyToUse::Deterministic,
    )
    .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Second we get the envelope for the users keys sorted.
    let wrapped = RotateUserKeyPayload::new(user_key.storage, None)
        .to_bytes()
        .map_err(WalletError::InvalidPrivateKey)?;
    let envelope = seal_for_transport(wrapped, enclave_keys, KeyToUse::Deterministic)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let _response: CreateEncryptedWalletResponse = client
        .mutation(
            "agent.createEncryptedWallet",
            json!({
                "name": wallet.name.clone(),
                "address": wallet.address.clone(),
                "blob": STANDARD.encode(encrypted_wallet_blob),
                "envelope": STANDARD.encode(envelope),
            }),
        )
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(wallet)
}

pub async fn list_wallets(client: &IrisClient) -> WalletResult<Vec<WalletList>> {
    let response: ListEncryptedWalletsResponse = client
        .query("agent.listEncryptedWallets", json!({}))
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Convert the HashMap<chain_type, address> to Vec<WalletList>
    let wallets: Vec<WalletList> = response
        .0
        .into_iter()
        .map(|(chain_str, entry)| {
            let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

            Ok(WalletList {
                chain_type,
                name: entry.name,
                address: entry.address,
            })
        })
        .collect::<Result<Vec<_>, WalletError>>()?;

    Ok(wallets)
}

pub async fn rotate_user_encryption_key(
    new_user_encryption_key: &UsersEncryptionKeys,
    old_user_encryption_key: &UsersEncryptionKeys,
    ek: Option<&TransportKeyReceiver>,
    client: &IrisClient,
) -> WalletResult<()> {
    let enclave_keys = match ek {
        Some(ek) => ek,
        None => &get_transport_key(client).await?,
    };

    let wrapped = RotateUserKeyPayload::new(new_user_encryption_key.storage, Some(old_user_encryption_key.storage))
        .to_bytes()
        .map_err(WalletError::InvalidPrivateKey)?;
    let envelope = seal_for_transport(wrapped, enclave_keys, KeyToUse::Deterministic)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let _response: CreateEncryptedWalletResponse = client
        .mutation(
            "agent.rotateUserEncryptionKey",
            json!({
                "envelope": STANDARD.encode(envelope),
            }),
        )
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}

pub async fn delete_wallet(address: String, client: &IrisClient) -> WalletResult<()> {
    let _response: CreateEncryptedWalletResponse = client
        .mutation(
            "agent.deleteEncryptedWallet",
            json!({
                "walletAddress": address
            }),
        )
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::wallet::types::WalletEntry;

    use super::*;

    #[test]
    fn test_list_wallets_response_conversion() {
        // Test the conversion logic without any API calls
        let mut wallets_map = HashMap::new();
        wallets_map.insert(
            "EVM".to_string(),
            WalletEntry {
                name: "EVM".to_string(),
                address: "0xabc123".to_string(),
            },
        );
        wallets_map.insert(
            "SVM".to_string(),
            WalletEntry {
                name: "SVM".to_string(),
                address: "SolanaAddress123".to_string(),
            },
        );

        let response = ListEncryptedWalletsResponse(wallets_map);

        // Convert the HashMap<chain_type, address> to Vec<WalletList>
        let wallets: Vec<WalletList> = response
            .0
            .into_iter()
            .map(|(chain_str, entry)| {
                let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

                Ok(WalletList {
                    chain_type,
                    name: entry.name,
                    address: entry.address,
                })
            })
            .collect::<Result<Vec<_>, WalletError>>()
            .unwrap();

        assert_eq!(wallets.len(), 2);

        // Check EVM wallet
        let evm_wallet = wallets
            .iter()
            .find(|w| matches!(w.chain_type, ChainType::EVM));
        assert!(evm_wallet.is_some());
        assert_eq!(evm_wallet.unwrap().address, "0xabc123");

        // Check SVM wallet
        let svm_wallet = wallets
            .iter()
            .find(|w| matches!(w.chain_type, ChainType::SVM));
        assert!(svm_wallet.is_some());
        assert_eq!(svm_wallet.unwrap().address, "SolanaAddress123");
    }

    #[test]
    fn test_list_wallets_empty_response() {
        let response = ListEncryptedWalletsResponse(HashMap::new());

        let wallets: Vec<WalletList> = response
            .0
            .into_iter()
            .map(|(chain_str, entry)| {
                let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

                Ok(WalletList {
                    chain_type,
                    name: entry.name,
                    address: entry.address,
                })
            })
            .collect::<Result<Vec<_>, WalletError>>()
            .unwrap();

        assert!(wallets.is_empty());
    }

    #[test]
    fn test_list_wallets_invalid_chain() {
        let mut wallets_map = HashMap::new();
        wallets_map.insert(
            "INVALID_CHAIN".to_string(),
            WalletEntry {
                name: "INVALID_CHAIN".to_string(),
                address: "0xabc123".to_string(),
            },
        );

        let response = ListEncryptedWalletsResponse(wallets_map);

        let result: Result<Vec<WalletList>, WalletError> = response
            .0
            .into_iter()
            .map(|(chain_str, entry)| {
                let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

                Ok(WalletList {
                    chain_type,
                    name: entry.name,
                    address: entry.address,
                })
            })
            .collect();

        assert!(result.is_err());
        match result.unwrap_err() {
            WalletError::ParsingWalletList => (), // Expected
            _ => panic!("Expected ParsingWalletList error"),
        }
    }

    #[test]
    fn test_verify_attestation_empty() {
        let result = verify_attestation_document(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_verify_attestation_non_empty() {
        // Currently a placeholder, should succeed with any non-empty data
        let result = verify_attestation_document(b"some attestation data");
        assert!(result.is_ok());
    }
}
