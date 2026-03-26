//! Shared CLI definitions for Edge Trade binaries.
//!
//! This module provides the common CLI structure used by both

use clap::{Parser, Subcommand, ValueEnum};

use crate::config::DEFAULT_CONFIG_PATH;

pub const DEFAULT_TRANSPORT: &str = "stdio";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_PATH: &str = "mcp";

#[derive(Clone, Copy, ValueEnum)]
pub enum Transport {
    Stdio,
    Http,
}

/// Edge — connecting AI agents and their humans to real-time market data, alert tracking, and trading.
///
/// Edge.Trade's local server is built for both agents and their humans to interact with the Edge on-chain
/// trading platform. The server embraces full decentralization principles while still giving users and
/// their agents a great user experience. Only we do it in a way that brings crypto back to its roots.
///
/// With Edge's tooling you and your agents do not need to always be logged into your browser allowing
/// someone else to control your keys and forcing yourself to sit in front of the monitor for 450 hours
/// a day. Instead you can allow the tools and your agents to do their work and get you the best trades.
///
/// Edge's tooling gives you alerts, non-custodial wallet management, information, and trading all in the
/// same place.
#[derive(Parser)]
#[command(name = "edge")]
#[command(arg_required_else_help = true)]
#[command(max_term_width = 120)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        env = "EDGE_CONFIG",
        default_value_t = DEFAULT_CONFIG_PATH.clone(),
        help = "Path to configuration file"
    )]
    pub config: String,

    #[arg(
        long,
        global = true,
        env = "EDGE_API_KEY",
        help = "Edge API key. Priority: 1) this flag, 2) EDGE_API_KEY env var, 3) api_key in config file. Get one at https://edge.trade"
    )]
    pub api_key: Option<String>,

    #[arg(long, global = true, help = "Print verbose connection and request logs to stderr")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve {
        #[command(flatten)]
        args: ServeArgs,

        #[command(subcommand)]
        command: Option<ServeCommand>,
    },

    Key {
        #[command(subcommand)]
        command: Option<KeyCommand>,
    },

    Wallet {
        #[command(subcommand)]
        command: Option<WalletCommand>,
    },

    Skill {
        #[command(subcommand)]
        command: Option<SkillCommand>,
    },

    /// Print available MCP tools as JSON
    ListTools,
    /// Ping the Edge API and exit on success
    Ping,
    /// Print version information and exit
    Version,
}

/// Start your local Edge server connecting to information, alerts, and trading.
#[derive(Subcommand)]
pub enum ServeCommand {
    /// Start the Edge server (default)
    Start,
}

#[derive(Parser)]
pub struct ServeArgs {
    #[arg(
        long,
        default_value = DEFAULT_TRANSPORT,
        help = "Transport: stdio or http. Use stdio for Cursor/Claude Desktop; use http to serve over a local port."
    )]
    pub transport: Transport,

    #[arg(long, default_value = DEFAULT_HOST, help = "Host address to bind")]
    pub host: String,

    #[arg(long, default_value = DEFAULT_PORT, help = "Port to listen on")]
    pub port: u16,

    #[arg(
        long,
        default_value = DEFAULT_PATH,
        help = "Path prefix for the HTTP endpoint (e.g. mcp → /mcp)"
    )]
    pub path: String,
}

/// Manage Edge keys which are used to encrypt the messages and information sent to our servers.
///
/// These keys are important but they are not your wallet keys. We uses these keys to encrypt
/// messages and information which is sent to our backend servers in such a way that only our
/// deepest vaults are able to decrypt it.
///
/// This allows us to do things like save your limit orders in our database without needing to
/// worry that if someone stole those orders that your information would be in the slightest
/// bit compromised.
#[derive(Subcommand)]
pub enum KeyCommand {
    /// Create new key configuration
    Create,
    /// Unlock the session (only available for keystore sessions)
    Unlock,
    /// Lock the session (only available for keystore sessions)
    Lock,
    /// Update authentication (change password)
    Update,
    /// Delete key configuration
    Delete,
}

/// Manage Edge wallets that your agent will have access to.
///
/// Your wallet will never be in a position where it will be able to do anything without your approval.
#[derive(Subcommand)]
pub enum WalletCommand {
    /// Create a new wallet
    Create {
        /// Blockchain type (evm or svm)
        #[arg(short, long)]
        chain_type: String,
        /// Optional wallet name
        #[arg(long)]
        name: Option<String>,
    },

    /// Import a wallet from private key file or manual input
    Import {
        /// Blockchain type (evm or svm)
        #[arg(short, long)]
        chain_type: String,
        /// Optional wallet name
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        key_file: Option<String>,
    },
    /// List wallets for the agent
    List,

    /// Delete a wallet
    Delete {
        /// Wallet address
        #[arg(long)]
        address: String,
    },

    /// Play the prove game to test wallet security constraints
    Prove {
        /// Game to play: 1 (Blind Oracle), 2 (The Vault), or both
        #[arg(long)]
        game: Option<u8>,
        /// Replay the last game without creating new intents
        #[arg(long)]
        replay: bool,
    },
}

/// Manage agent skills that allow your "team" to interact with Edge in the
/// most efficient and effective way possible.
#[derive(Subcommand)]
pub enum SkillCommand {
    /// List available skills from the manifest
    List,

    /// Install a skill to a local directory
    Install {
        /// Name of the skill to install
        name: String,
        /// Directory to install into (writes <dir>/<name>/SKILL.md)
        #[arg(long)]
        path: String,
    },
}
