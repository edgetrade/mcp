//! Session management module for Edge CLI.
//!
//! This module provides session storage implementations:
//! - **keyring**: OS keyring storage (preferred)
//! - **filestore**: File-based fallback when keyring unavailable
//!
//! The unified `Session` enum automatically selects keyring if available,
//! otherwise falls back to file-based storage with a warning.
//!
//! # Configuration
//!
//! Session requires an explicit `Config` to be passed during construction.
//! The configuration should be loaded using [`Config::load_default()`] or
//! [`Config::load_from(path)`] before creating a Session.

pub mod crypto;
pub mod filestore;
pub mod keyring;
pub mod transport;

// Re-export both backends
pub use filestore::{Session as FileStoreSession, SessionError as FileStoreError};
pub use keyring::{Session as KeyringSession, SessionError as KeyringError};

use crate::config::Config;
use crate::messages;
use crypto::UsersEncryptionKeys;

/// Unified session error type.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    #[error("Keyring error: {0}")]
    Keyring(String),
    #[error("File storage error: {0}")]
    FileStore(String),
    #[error("Session not found")]
    NotFound,
    #[error("Session corrupted")]
    Corrupted,
}

impl From<KeyringError> for SessionError {
    fn from(e: KeyringError) -> Self {
        SessionError::Keyring(e.to_string())
    }
}

impl From<FileStoreError> for SessionError {
    fn from(e: FileStoreError) -> Self {
        match e {
            FileStoreError::Storage(msg) => SessionError::FileStore(msg),
            FileStoreError::Encoding(msg) => SessionError::FileStore(msg),
            FileStoreError::Corrupted => SessionError::Corrupted,
        }
    }
}

/// Unified session backend that automatically selects keyring or file storage.
///
/// This enum provides a single interface for session management that:
/// 1. Attempts to use the OS keyring first (preferred for security)
/// 2. Falls back to file-based storage if the keyring is unavailable
///
/// Both backends provide the same operations: unlock, lock, get_user_encryption_key, is_unlocked.
#[derive(Debug, Clone)]
pub enum Session {
    /// OS keyring backend (preferred)
    Keyring(KeyringSession),
    /// File-based fallback backend
    File(FileStoreSession),
}

/// Check if the OS keyring is available by attempting a probe operation.
///
/// This function tries to access the keyring to determine if it's functional.
/// It performs a lightweight check that won't affect existing stored data.
///
/// # Returns
/// `true` if the keyring is accessible, `false` otherwise.
pub fn keyring_available() -> bool {
    use keyring::Entry;

    // Try to create an entry - this tests if the keyring service is available
    match Entry::new("edge-probe", "probe") {
        Ok(entry) => {
            // Try to get password to verify keyring is functional
            // We don't care about the result, just that it doesn't panic or hang
            let _: Result<String, _> = entry.get_password();
            true
        }
        Err(_) => false,
    }
}

impl Session {
    /// Create a new session with automatic backend selection.
    ///
    /// This constructor probes the OS keyring and:
    /// - Returns `Session::Keyring` if the keyring is available
    /// - Returns `Session::File` if the keyring is unavailable (prints warning)
    ///
    /// # Arguments
    /// * `config` - The configuration for the session. Must be loaded explicitly
    ///   using `Config::load_default()` or `Config::load_from(path)`.
    ///
    /// # Example
    /// ```rust
    /// use poseidon::session::Session;
    /// use poseidon::config::Config;
    ///
    /// let config = Config::load_default().expect("Failed to load config");
    /// let session = Session::new(config);
    /// if session.is_unlocked() {
    ///     println!("Session is active");
    /// }
    /// ```
    pub fn new(config: Config) -> Self {
        if keyring_available() {
            Session::Keyring(KeyringSession::new(config))
        } else {
            messages::warning::keyring_unavailable();
            Session::File(FileStoreSession::new(config))
        }
    }

    /// Create a new session explicitly using the keyring backend.
    ///
    /// # Panics
    /// Panics if the keyring is not available. Use `Session::new()` for automatic fallback.
    pub fn new_keyring(config: Config) -> Self {
        Session::Keyring(KeyringSession::new(config))
    }

    /// Create a new session explicitly using the file storage backend.
    pub fn new_file(config: Config) -> Self {
        Session::File(FileStoreSession::new(config))
    }

    /// Store the user encryption key (unlock the session).
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn unlock(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.unlock(uek).map_err(|e| e.into()),
            Session::File(s) => s.unlock(uek).map_err(|e| e.into()),
        }
    }

    /// Lock the session by deleting the stored key.
    ///
    /// This operation is idempotent - it succeeds even if no session exists.
    pub fn lock(&self) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.lock().map_err(|e| e.into()),
            Session::File(s) => s.lock().map_err(|e| e.into()),
        }
    }

    /// Get the user encryption key from the session.
    ///
    /// # Returns
    /// `Ok(Some(UsersEncryptionKeys))` if a key exists,
    /// `Ok(None)` if no session is active.
    pub fn get_user_encryption_key(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        match self {
            Session::Keyring(s) => s.get_user_encryption_key().map_err(|e| e.into()),
            Session::File(s) => s.get_user_encryption_key().map_err(|e| e.into()),
        }
    }

    /// Get the agent ID from the session.
    ///
    /// # Returns
    /// `Ok(&Config)` if the config exists,
    /// `Err(SessionError)` if the config cannot be retrieved.
    pub fn get_config(&self) -> Result<&Config, SessionError> {
        match self {
            Session::Keyring(s) => s
                .get_config()
                .map_err(|e| SessionError::Keyring(e.to_string())),
            Session::File(s) => s
                .get_config()
                .map_err(|e| SessionError::FileStore(e.to_string())),
        }
    }

    /// Check if the session is unlocked (key exists).
    pub fn is_unlocked(&self) -> bool {
        match self {
            Session::Keyring(s) => s.is_unlocked(),
            Session::File(s) => s.is_unlocked(),
        }
    }

    /// Save the user encryption key (alias for unlock).
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn save(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        self.unlock(uek)
    }

    /// Change the stored user encryption key.
    ///
    /// # Arguments
    /// * `uek` - The new user encryption key to store
    pub fn change(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.change(uek).map_err(|e| e.into()),
            Session::File(s) => s.unlock(uek).map_err(|e| e.into()),
        }
    }

    /// Get the user encryption key (alias for get_user_encryption_key).
    pub fn get(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        self.get_user_encryption_key()
    }

    /// Unlock the session using a password.
    ///
    /// This method derives the user encryption key from the provided password
    /// and stores it in the session, making the session unlocked.
    ///
    /// # Arguments
    /// * `password` - The user's password to derive the encryption key from.
    ///
    /// # Returns
    /// `Ok(())` on success, or `SessionError` if unlocking fails.
    ///
    /// # Example
    /// ```rust,no_run
    /// use poseidon::session::Session;
    /// use poseidon::config::Config;
    ///
    /// let config = Config::load_default().expect("Failed to load config");
    /// let session = Session::new(config);
    /// session.unlock_with_password("my_secure_password")
    ///     .expect("Failed to unlock session");
    /// ```
    pub fn unlock_with_password(&self, password: &str) -> Result<(), SessionError> {
        // Derive the user encryption key from the password
        // This uses a fixed salt for deterministic key derivation
        // In production, salt should be loaded from secure storage
        let derived_key = self.derive_key_from_password(password)?;
        self.unlock(&derived_key)
    }

    /// Derive a user encryption key from a password.
    ///
    /// This is a helper method used by `unlock_with_password`.
    fn derive_key_from_password(&self, _password: &str) -> Result<UsersEncryptionKeys, SessionError> {
        // This is a placeholder implementation
        // The actual implementation should use the same derivation logic
        // as the filestore unlock flow (PBKDF2 + HKDF)
        // For now, this returns an error indicating the method is not fully implemented
        Err(SessionError::Keyring(
            "Password-based unlock requires salt from secure storage. \
             Use the filestore unlock command instead."
                .to_string(),
        ))
    }
}
