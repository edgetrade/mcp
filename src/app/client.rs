use std::process;

use crate::messages;
use crate::utils::urls::EDGE_MCP_URL;

use super::cli::Cli;

#[derive(Clone)]
pub struct AppClientCredentials {
    pub api_key: String,
    pub iris_url: String,
    pub verbose: bool,
}

pub async fn parse_api_credentials(cli: &Cli) -> AppClientCredentials {
    let api_key = cli.api_key.clone().unwrap_or_else(|| {
        messages::error::api_key_required();
        messages::error::api_key_docs_url();
        process::exit(1);
    });

    let iris_url = std::env::var("EDGE_MCP_URL").unwrap_or_else(|_| EDGE_MCP_URL.to_string());

    AppClientCredentials {
        api_key,
        iris_url,
        verbose: cli.verbose,
    }
}
