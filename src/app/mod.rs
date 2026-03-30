//! Application runtime module for the Edge CLI.
//!
//! This module provides the core application logic for the Edge Trade CLI,
//! including command-line parsing, command handling, and the main application runner.
//!
//! # Module Structure
//!
//! - `cli`: Command-line interface definitions using clap
//! - `client`: API client credential parsing and management
//! - `handler`: Command handlers for keys, wallets, skills, and server operations
//! - `runner`: Main application runner with session and manifest management
//!
//! # Usage
//!
//! For library consumers, the main entry point is the [`run`] function:
//!
//! ```rust,no_run
//! use poseidon::app::{run, App};
//!
//! // Run the CLI application
//! // run(key_create, key_unlock, key_lock, key_update, key_delete).await;
//! ```
//!
//! For programmatic use, the [`App`] struct provides session and manifest management.

pub mod cli;
pub mod client;
pub mod handler;
pub mod runner;

pub use runner::{App, run};
