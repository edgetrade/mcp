use std::process;

use clap::CommandFactory;
use tyche_enclave::types::chain_type::ChainType;

use crate::app::cli::Transport;
use crate::commands;
use crate::commands::serve::mcp::EdgeServer;
use crate::config::Config;
use crate::manifest::McpManifest;
use crate::messages;
use crate::session::Session;
use crate::utils::urls::EDGE_MCP_URL;

use super::cli::{Cli, KeyCommand, ServeArgs, SkillCommand, WalletCommand};
use super::{KeyCreateFn, KeyDeleteFn, KeyLockFn, KeyUnlockFn, KeyUpdateFn};

pub async fn serve(args: &ServeArgs, server: EdgeServer) -> Result<(), i32> {
    match args.transport {
        Transport::Http => server
            .serve_http(&args.host, &args.port, &args.path)
            .await
            .map_err(|e| {
                messages::error::mcp_server_error(&e.to_string());
                1
            }),
        Transport::Stdio => server.serve_stdio().await.map_err(|e| {
            messages::error::mcp_server_error(&e.to_string());
            1
        }),
    }
}

pub struct KeyCommandArgs {
    pub command: Option<KeyCommand>,
    pub config: Config,
    pub client: crate::client::IrisClient,
}

pub async fn handle_key(
    key_create: KeyCreateFn,
    key_unlock: KeyUnlockFn,
    key_lock: KeyLockFn,
    key_update: KeyUpdateFn,
    key_delete: KeyDeleteFn,
    args: KeyCommandArgs,
) -> Result<(), i32> {
    match args.command {
        Some(KeyCommand::Create) => key_create(args.config).map_err(|e| {
            messages::error::key_command_error("create", &e.to_string());
            1
        }),
        Some(KeyCommand::Unlock) => key_unlock(args.config).map_err(|e| {
            messages::error::key_command_error("unlock", &e.to_string());
            1
        }),
        Some(KeyCommand::Lock) => key_lock(args.config).map_err(|e| {
            messages::error::key_command_error("lock", &e.to_string());
            1
        }),
        Some(KeyCommand::Update) => key_update(args.config, &args.client).await.map_err(|e| {
            messages::error::key_command_error("update", &e.to_string());
            1
        }),
        Some(KeyCommand::Delete) => key_delete(args.config).map_err(|e| {
            messages::error::key_command_error("delete", &e.to_string());
            1
        }),
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd.find_subcommand("key").expect("key subcommand exists");
            println!("{}", sub.clone().render_help());
            Ok(())
        }
    }
}

pub async fn handle_wallet(
    command: &Option<WalletCommand>,
    session: &Session,
    client: &crate::client::IrisClient,
) -> Result<(), i32> {
    match command {
        Some(WalletCommand::Create { chain_type, name }) => {
            let chain_type = ChainType::parse(chain_type).map_err(|_| {
                messages::error::invalid_chain_type();
                1
            })?;
            commands::wallet::wallet_create(chain_type, name.clone(), session, client)
                .await
                .map_err(|e| {
                    messages::error::wallet_command_error("create", &e.to_string());
                    1
                })
        }
        Some(WalletCommand::Import {
            chain_type,
            name,
            key_file,
        }) => {
            let chain_type = ChainType::parse(chain_type).map_err(|_| {
                messages::error::invalid_chain_type();
                1
            })?;
            commands::wallet::wallet_import(chain_type, name.clone(), key_file.clone(), session, client)
                .await
                .map_err(|e| {
                    messages::error::wallet_command_error("import", &e.to_string());
                    1
                })
        }
        Some(WalletCommand::List) => commands::wallet::wallet_list(client).await.map_err(|e| {
            messages::error::wallet_command_error("list", &e.to_string());
            1
        }),
        Some(WalletCommand::Delete { address }) => commands::wallet::wallet_delete(address.clone(), client)
            .await
            .map_err(|e| {
                messages::error::wallet_command_error("delete", &e.to_string());
                1
            }),
        Some(WalletCommand::Prove { game, replay }) => commands::wallet::wallet_prove(*game, *replay, session, client)
            .await
            .map_err(|e| {
                messages::error::wallet_command_error("prove", &e.to_string());
                1
            }),
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd
                .find_subcommand("wallet")
                .expect("wallet subcommand exists");
            println!("{}", sub.clone().render_help());
            Ok(())
        }
    }
}

pub fn handle_skill(command: &Option<SkillCommand>, manifest: &McpManifest) -> Result<(), i32> {
    match command {
        Some(SkillCommand::List) => {
            for skill in &manifest.skills {
                messages::success::skill_output(&skill.name, &skill.description);
            }
            Ok(())
        }
        Some(SkillCommand::Install { name, path }) => match manifest.skills.iter().find(|s| &s.name == name) {
            Some(skill) => {
                let dir = std::path::Path::new(path).join(name);
                if let Err(e) = std::fs::create_dir_all(&dir) {
                    messages::error::create_dir_error(&e.to_string());
                    return Err(1);
                }
                if let Err(e) = std::fs::write(dir.join("SKILL.md"), &skill.content) {
                    messages::error::write_skill_error(&e.to_string());
                    return Err(1);
                }
                messages::error::skill_installed(name, &dir.display().to_string());
                Ok(())
            }
            None => {
                messages::error::skill_not_found(name);
                Err(1)
            }
        },
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd
                .find_subcommand("skill")
                .expect("skill subcommand exists");
            println!("{}", sub.clone().render_help());
            Ok(())
        }
    }
}

pub async fn handle_ping(verbose: bool) {
    let iris_url = std::env::var("EDGE_MCP_URL").unwrap_or_else(|_| EDGE_MCP_URL.to_string());
    let ping_url = format!("{}/ping", iris_url);

    if verbose {
        messages::error::pinging(&ping_url);
    }

    match reqwest::Client::new().get(&ping_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if verbose {
                    messages::success::ping_success(&response.status().to_string());
                }
                process::exit(0);
            } else {
                messages::error::ping_failed_status(&response.status().to_string());
                process::exit(1);
            }
        }
        Err(e) => {
            messages::error::ping_failed_error(&e.to_string());
            process::exit(1);
        }
    }
}

pub fn handle_version() {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let sha = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
    let short_sha = &sha[..sha.len().min(7)];
    let describe = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or("");
    if describe.is_empty() || describe.starts_with(short_sha) {
        println!("edge {pkg_version} (commit {short_sha})");
    } else {
        println!("edge {pkg_version} ({describe}, commit {short_sha})");
    }
}
