//! Unified main entry point for Edge CLI.
//!
//! This binary combines both keyring-based and file-based session storage
//! into a single executable. The session backend is selected based on
//! configuration, or auto-detected on first run.

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use clap::Parser;
use poseidon::app::cli::Cli;
use poseidon::app::runner::run;
use poseidon::app::{KeyCreateFn, KeyDeleteFn, KeyLockFn, KeyUnlockFn, KeyUpdateFn};
use poseidon::client::IrisClient;
use poseidon::config::{Config, set_config_path, should_use_keyring};
use poseidon::messages;
use poseidon::messages::success::CommandResult;

/// Keyring-based key command implementations (preferred when available).
mod keyring {
    pub use poseidon::commands::key::keyring::{
        keyring_create as key_create, keyring_delete as key_delete, keyring_lock as key_lock,
        keyring_unlock as key_unlock, keyring_update as key_update,
    };
}

/// File-based key command implementations (fallback when keyring unavailable).
mod filestore {
    pub use poseidon::commands::key::filestore::{key_create, key_delete, key_lock, key_unlock, key_update};
}

fn keyring_create_wrapper(config: Config) -> CommandResult<()> {
    keyring::key_create(config)
}

fn filestore_create_wrapper(_config: Config) -> CommandResult<()> {
    filestore::key_create()
}

fn keyring_unlock_wrapper(_config: Config) -> CommandResult<()> {
    keyring::key_unlock()
}

fn filestore_unlock_wrapper(config: Config) -> CommandResult<()> {
    filestore::key_unlock(config)
}

fn keyring_lock_wrapper(_config: Config) -> CommandResult<()> {
    keyring::key_lock()
}

fn filestore_lock_wrapper(config: Config) -> CommandResult<()> {
    filestore::key_lock(config)
}

fn keyring_update_wrapper(
    config: Config,
    client: &IrisClient,
) -> Pin<Box<dyn Future<Output = CommandResult<()>> + '_>> {
    Box::pin(keyring::key_update(config, client))
}

fn filestore_update_wrapper(
    config: Config,
    client: &IrisClient,
) -> Pin<Box<dyn Future<Output = CommandResult<()>> + '_>> {
    Box::pin(filestore::key_update(config, client))
}

fn keyring_delete_wrapper(_config: Config) -> CommandResult<()> {
    keyring::key_delete()
}

fn filestore_delete_wrapper(_config: Config) -> CommandResult<()> {
    filestore::key_delete()
}

/// Select key command implementations based on configuration.
///
/// Checks the config file for the session.use_keyring setting.
/// If not set, probes the OS keyring once and saves the result to config.
/// Users can override by editing the config file directly.
fn select_key_commands() -> (KeyCreateFn, KeyUnlockFn, KeyLockFn, KeyUpdateFn, KeyDeleteFn) {
    if should_use_keyring() {
        (
            keyring_create_wrapper,
            keyring_unlock_wrapper,
            keyring_lock_wrapper,
            Box::new(keyring_update_wrapper),
            keyring_delete_wrapper,
        )
    } else {
        messages::warning::file_storage_fallback();
        (
            filestore_create_wrapper,
            filestore_unlock_wrapper,
            filestore_lock_wrapper,
            Box::new(filestore_update_wrapper),
            filestore_delete_wrapper,
        )
    }
}

#[tokio::main]
async fn main() {
    // Parse CLI arguments early to get config path override
    let cli = Cli::parse();

    // Set config path override from the --config flag (always has a value via default)
    set_config_path(Some(PathBuf::from(cli.config)));

    // Select the appropriate key command implementations based on config
    let (key_create, key_unlock, key_lock, key_update, key_delete): (
        KeyCreateFn,
        KeyUnlockFn,
        KeyLockFn,
        KeyUpdateFn,
        KeyDeleteFn,
    ) = select_key_commands();

    // Run the application with the selected key commands
    run(key_create, key_unlock, key_lock, key_update, key_delete).await;
}
