//! Key management commands for Edge CLI.
//!
//! Provides commands for creating, unlocking, locking, and updating
//! encryption keys. The implementation is split between desktop (keyring-based)
//! and server (password-based) variants.

pub mod filestore;
pub mod keyring;
pub mod session_manager;
