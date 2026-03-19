//! Command implementations for Edge CLI.
//!
//! Provides interactive commands for key management, wallet operations,
//! and session lifecycle management.
//!
//! # Feature Boundaries
//!
//! This module defines common error types that are agnostic to the
//! desktop/server feature split. Feature-specific code should convert
//! their errors to these generic types using `.map_err()` rather than
//! implementing `From` trait for feature-specific types.

pub mod key;
pub mod wallet;

use thiserror::Error;

use crate::messages;

/// Error type for command operations.
///
/// This is a generic error type that does not depend on feature-specific
/// modules. All feature-specific errors should be converted to this
/// type at the feature boundary using `.map_err()`.
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Session error: {0}")]
    Session(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Configuration already exists")]
    AlreadyExists,
    #[error("Configuration not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Wallet error: {0}")]
    Wallet(String),
}

/// Error codes for consistent error message formatting.
#[derive(Debug, Clone, Copy)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    // Key management errors
    pub const KEY_CREATE_FAILED: Self = ErrorCode("ERR_KEY_CREATE_FAILED");
    pub const KEY_UPDATE_FAILED: Self = ErrorCode("ERR_KEY_UPDATE_FAILED");
    pub const KEY_DELETE_FAILED: Self = ErrorCode("ERR_KEY_DELETE_FAILED");
    pub const KEY_LOCK_FAILED: Self = ErrorCode("ERR_KEY_LOCK_FAILED");
    pub const KEY_UNLOCK_FAILED: Self = ErrorCode("ERR_KEY_UNLOCK_FAILED");
    pub const KEY_SHOW_FAILED: Self = ErrorCode("ERR_KEY_SHOW_FAILED");
    pub const KEY_EXPORT_FAILED: Self = ErrorCode("ERR_KEY_EXPORT_FAILED");

    // Filestore errors
    pub const FILESTORE_CREATE_FAILED: Self = ErrorCode("ERR_FILESTORE_CREATE_FAILED");
    pub const FILESTORE_UPDATE_FAILED: Self = ErrorCode("ERR_FILESTORE_UPDATE_FAILED");
    pub const FILESTORE_DELETE_FAILED: Self = ErrorCode("ERR_FILESTORE_DELETE_FAILED");
    pub const FILESTORE_LOCK_FAILED: Self = ErrorCode("ERR_FILESTORE_LOCK_FAILED");
    pub const FILESTORE_UNLOCK_FAILED: Self = ErrorCode("ERR_FILESTORE_UNLOCK_FAILED");
    pub const FILESTORE_SHOW_FAILED: Self = ErrorCode("ERR_FILESTORE_SHOW_FAILED");

    // Keyring errors
    pub const KEYRING_CREATE_FAILED: Self = ErrorCode("ERR_KEYRING_CREATE_FAILED");
    pub const KEYRING_UPDATE_FAILED: Self = ErrorCode("ERR_KEYRING_UPDATE_FAILED");
    pub const KEYRING_DELETE_FAILED: Self = ErrorCode("ERR_KEYRING_DELETE_FAILED");
    pub const KEYRING_LOCK_FAILED: Self = ErrorCode("ERR_KEYRING_LOCK_FAILED");
    pub const KEYRING_UNLOCK_FAILED: Self = ErrorCode("ERR_KEYRING_UNLOCK_FAILED");
    pub const KEYRING_SHOW_FAILED: Self = ErrorCode("ERR_KEYRING_SHOW_FAILED");

    // Wallet errors
    pub const WALLET_CREATE_FAILED: Self = ErrorCode("ERR_WALLET_CREATE_FAILED");
    pub const WALLET_IMPORT_FAILED: Self = ErrorCode("ERR_WALLET_IMPORT_FAILED");
    pub const WALLET_DELETE_FAILED: Self = ErrorCode("ERR_WALLET_DELETE_FAILED");
    pub const WALLET_SHOW_FAILED: Self = ErrorCode("ERR_WALLET_SHOW_FAILED");
    pub const WALLET_LIST_FAILED: Self = ErrorCode("ERR_WALLET_LIST_FAILED");

    // Session errors
    pub const SESSION_UNLOCK_FAILED: Self = ErrorCode("ERR_SESSION_UNLOCK_FAILED");
    pub const SESSION_LOCK_FAILED: Self = ErrorCode("ERR_SESSION_LOCK_FAILED");

    // Manifest errors
    pub const MANIFEST_FETCH_FAILED: Self = ErrorCode("ERR_MANIFEST_FETCH_FAILED");
    pub const MANIFEST_REFRESH_FAILED: Self = ErrorCode("ERR_MANIFEST_REFRESH_FAILED");
    pub const MANIFEST_SAVE_FAILED: Self = ErrorCode("ERR_MANIFEST_SAVE_FAILED");
    pub const MANIFEST_PARSE_FAILED: Self = ErrorCode("ERR_MANIFEST_PARSE_FAILED");

    // Client API key errors
    pub const API_KEY_REQUIRED: Self = ErrorCode("ERR_API_KEY_REQUIRED");
    pub const API_KEY_INVALID: Self = ErrorCode("ERR_API_KEY_INVALID");
}

impl ErrorCode {
    /// Returns the error code string
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Only implement From for types that are common to both features.
// Feature-specific types should use .map_err() at the boundary.

impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        CommandError::Io(e.to_string())
    }
}

impl From<crate::commands::key::filestore::crypto::types::CryptoError> for CommandError {
    fn from(e: crate::commands::key::filestore::crypto::types::CryptoError) -> Self {
        CommandError::Crypto(e.to_string())
    }
}

impl From<crate::wallet::types::WalletError> for CommandError {
    fn from(e: crate::wallet::types::WalletError) -> Self {
        CommandError::Wallet(e.to_string())
    }
}

impl From<crate::commands::key::filestore::storage::StorageError> for CommandError {
    fn from(e: crate::commands::key::filestore::storage::StorageError) -> Self {
        CommandError::Storage(e.to_string())
    }
}

/// Result type for command operations.
pub type CommandResult<T> = Result<T, CommandError>;

// ============== Context-Aware Constructor Functions ==============
// These functions delegate console output to the messages module

// --- Key Command Errors ---

/// Key creation failed
pub fn key_create_failed(chain: &str) {
    messages::error::key_command_error("create", chain);
}

/// Key update failed
pub fn key_update_failed(chain: &str) {
    messages::error::key_command_error("update", chain);
}

/// Key deletion failed
pub fn key_delete_failed(chain: &str) {
    messages::error::key_command_error("delete", chain);
}

/// Key lock failed
pub fn key_lock_failed(chain: &str) {
    messages::error::key_command_error("lock", chain);
}

/// Key unlock failed
pub fn key_unlock_failed(chain: &str) {
    messages::error::key_command_error("unlock", chain);
}

/// Key show failed
pub fn key_show_failed(chain: &str) {
    messages::error::key_command_error("show", chain);
}

/// Key export failed
pub fn key_export_failed(chain: &str) {
    messages::error::key_command_error("export", chain);
}

// --- Filestore Command Errors ---

/// Filestore creation failed
pub fn filestore_create_failed(name: &str) {
    messages::error::key_command_error("create_filestore", name);
}

/// Filestore update failed
pub fn filestore_update_failed(name: &str) {
    messages::error::key_command_error("update_filestore", name);
}

/// Filestore deletion failed
pub fn filestore_delete_failed(name: &str) {
    messages::error::key_command_error("delete_filestore", name);
}

/// Filestore lock failed
pub fn filestore_lock_failed(name: &str) {
    messages::error::key_command_error("lock_filestore", name);
}

/// Filestore unlock failed
pub fn filestore_unlock_failed(name: &str) {
    messages::error::key_command_error("unlock_filestore", name);
}

/// Filestore show failed
pub fn filestore_show_failed(name: &str) {
    messages::error::key_command_error("show_filestore", name);
}

// --- Keyring Command Errors ---

/// Keyring creation failed
pub fn keyring_create_failed(chain: &str) {
    messages::error::key_command_error("create_keyring", chain);
}

/// Keyring update failed
pub fn keyring_update_failed(chain: &str) {
    messages::error::key_command_error("update_keyring", chain);
}

/// Keyring deletion failed
pub fn keyring_delete_failed(chain: &str) {
    messages::error::key_command_error("delete_keyring", chain);
}

/// Keyring lock failed
pub fn keyring_lock_failed(chain: &str) {
    messages::error::key_command_error("lock_keyring", chain);
}

/// Keyring unlock failed
pub fn keyring_unlock_failed(chain: &str) {
    messages::error::key_command_error("unlock_keyring", chain);
}

/// Keyring show failed
pub fn keyring_show_failed(chain: &str) {
    messages::error::key_command_error("show_keyring", chain);
}

// --- Wallet Command Errors ---

/// Wallet creation failed
pub fn wallet_create_failed(chain: &str) {
    messages::error::wallet_command_error("create", chain);
}

/// Wallet import failed
pub fn wallet_import_failed(chain: &str) {
    messages::error::wallet_command_error("import", chain);
}

/// Wallet deletion failed
pub fn wallet_delete_failed(chain: &str) {
    messages::error::wallet_command_error("delete", chain);
}

/// Wallet show failed
pub fn wallet_show_failed(chain: &str) {
    messages::error::wallet_command_error("show", chain);
}

/// Wallet list failed
pub fn wallet_list_failed(err: &str) {
    messages::error::wallet_command_error("list", err);
}

// --- Session Command Errors ---

/// Session unlock failed
pub fn session_unlock_failed(err: &str) {
    messages::error::session_error(err);
}

/// Session lock failed
pub fn session_lock_failed(err: &str) {
    messages::error::session_error(err);
}

// --- Manifest Errors ---

/// Manifest fetch failed
pub fn manifest_fetch_failed(url: &str) {
    messages::error::fetch_error(url);
}

/// Manifest refresh failed
pub fn manifest_refresh_failed(err: &str) {
    messages::error::manifest_parse_error(err);
}

/// Manifest save error
pub fn manifest_save_error(err: &str) {
    messages::error::manifest_save_error(err);
}

/// Manifest parse error
pub fn manifest_parse_error(err: &str) {
    messages::error::manifest_parse_error(err);
}

// --- Client API Key Errors ---

/// API key required
pub fn api_key_required() {
    messages::error::api_key_required();
}

/// API key invalid
pub fn api_key_invalid() {
    messages::error::api_key_invalid();
}

// --- Utility Functions ---

/// Display an error code and message
pub fn display_error(error_code: ErrorCode, message: &str) {
    eprintln!("[{}] {}", error_code.as_str(), message);
}

/// Display an error with context
pub fn display_error_with_context(error_code: ErrorCode, context: &str, message: &str) {
    eprintln!("[{}] {}: {}", error_code.as_str(), context, message);
}
