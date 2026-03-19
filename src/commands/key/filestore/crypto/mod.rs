//! Cryptographic operations for the Edge CLI.
//!
//! Provides key derivation (PBKDF2, HKDF), encryption (AES-256-GCM),
//! and secure key types with automatic memory zeroization.

pub mod encryption;
pub mod types;
