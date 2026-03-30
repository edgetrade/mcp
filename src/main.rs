//! Unified main entry point for Edge CLI.
//!
//! This binary combines both keyring-based and file-based session storage
//! into a single executable. The session backend is selected based on
//! configuration, or auto-detected on first run.

use std::process;

use poseidon::app::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
