//! Configuration management for Edge CLI.
//!
//! Handles loading and saving of configuration from the XDG config directory
//! or a user-specified location via `--config` flag or `EDGE_CONFIG` env var.
//! Configuration includes session storage preferences and keyring availability.
//!
//! # Loading Configuration
//!
//! Use [`Config::load`] to load configuration from a specific path or the default
//! XDG config location.
//!
//! # Configuration Location
//!
//! The default config file is located at:
//! - Linux: `~/.config/edge/config.toml`
//! - macOS: `~/Library/Application Support/edge/config.toml`
//! - Windows: `%APPDATA%\edge\config.toml`
//!
//! The location can be overridden with the `EDGE_CONFIG` environment variable.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Default config directory name
pub const CONFIG_DIR_NAME: &str = "edge";
/// Default config file name
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Returns the default config file path, checking EDGE_CONFIG env var first.
///
/// This function is used by both the CLI (for default_value) and Config::config_path()
/// to ensure consistent path resolution.
pub fn default_config_path_buf() -> Option<PathBuf> {
    // Check for EDGE_CONFIG env var first
    if let Ok(env_path) = std::env::var("EDGE_CONFIG") {
        return Some(PathBuf::from(env_path));
    }

    // Fall back to XDG config directory
    dirs::config_dir().map(|d| d.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
}

/// Edge CLI configuration.
///
/// This struct represents the user-configurable settings for the Edge CLI,
/// stored in `~/.config/edge/config.toml` (XDG config directory).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Edge API key for authentication
    #[serde(default)]
    pub api_key: Option<String>,
    /// Session storage configuration
    #[serde(default)]
    pub session: SessionConfig,
    /// ISO 8601 timestamp of last manifest fetch
    #[serde(default)]
    pub manifest_last_fetched: Option<String>,
    /// Enclave security configuration
    #[serde(default)]
    pub enclave: EnclaveConfig,
    /// Agent identifier for tracking
    #[serde(default)]
    pub agent_id: Option<Uuid>,
    /// MCP server enabled state (persisted across restarts)
    #[serde(default)]
    pub mcp_server_enabled: bool,
}

/// Enclave security and transport key configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnclaveConfig {
    /// Whether to verify attestation documents from the enclave.
    ///
    /// When `true`, attestation documents are cryptographically verified.
    /// When `false`, verification is skipped (useful for testing/local dev).
    /// Default: `true`.
    #[serde(default = "default_verify_attestation")]
    pub verify_attestation: bool,
    /// TTL for cached transport keys in minutes.
    ///
    /// Transport keys are cached locally to avoid repeated attestation
    /// round-trips. This specifies how long cached keys remain valid.
    /// Default: 15 minutes.
    #[serde(default = "default_transport_key_ttl")]
    pub transport_key_ttl_minutes: u64,
}

impl Default for EnclaveConfig {
    fn default() -> Self {
        Self {
            verify_attestation: default_verify_attestation(),
            transport_key_ttl_minutes: default_transport_key_ttl(),
        }
    }
}

fn default_verify_attestation() -> bool {
    true
}

fn default_transport_key_ttl() -> u64 {
    15
}

/// Session storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    /// Whether to use the OS keyring for session storage.
    ///
    /// If `true`, the keyring will be used. If `false` or not set,
    /// file-based storage will be used as a fallback.
    ///
    /// This is automatically detected on first run and cached here.
    /// Users can manually edit this to force a specific storage backend.
    #[serde(default)]
    pub use_keyring: Option<bool>,
}

/// Error type for configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("Config directory not found")]
    NoConfigDir,
}

impl Config {
    /// Load configuration from the specified path or default location.
    ///
    /// If `path` is `Some`, loads from that path. Otherwise, uses the default
    /// config location (`~/.config/edge/config.toml` or `$EDGE_CONFIG` env var).
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Arguments
    /// - `path` - Optional path to the config file. If `None`, uses default location.
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load(path: Option<PathBuf>) -> Result<Self, ConfigError> {
        match path {
            Some(p) => Self::load_from(p),
            None => Self::load_default(),
        }
    }

    /// Load configuration from a specific file path.
    ///
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Arguments
    /// - `path` - Path to the config file
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load_from(path: PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration from the default location.
    ///
    /// Uses the XDG config directory (`~/.config/edge/config.toml`)
    /// or `$EDGE_CONFIG` env var if set.
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load_default() -> Result<Self, ConfigError> {
        let config_path = default_config_path_buf().ok_or(ConfigError::NoConfigDir)?;
        Self::load_from(config_path)
    }

    /// Save configuration to the XDG config directory.
    ///
    /// Writes the configuration to `~/.config/edge/config.toml`,
    /// creating the directory if it doesn't exist.
    ///
    /// # Returns
    /// - `Ok(())` - On successful save
    /// - `Err(ConfigError)` - If there was an error writing the config file
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = default_config_path_buf().ok_or(ConfigError::NoConfigDir)?;
        let config_dir = config_path.parent().ok_or(ConfigError::NoConfigDir)?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }

        let contents = toml::to_string_pretty(self)?;
        let mut file = fs::File::create(&config_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }

    /// Get the path to the config file.
    ///
    /// Returns the default config path (from XDG config directory or `$EDGE_CONFIG` env var).
    ///
    /// # Returns
    /// - `Ok(PathBuf)` - The config file path
    /// - `Err(ConfigError)` - If the config directory could not be determined
    pub fn config_path() -> Result<PathBuf, ConfigError> {
        default_config_path_buf().ok_or(ConfigError::NoConfigDir)
    }

    /// Update the manifest timestamp to now and save
    pub fn update_manifest_timestamp(&mut self) -> Result<(), ConfigError> {
        self.manifest_last_fetched = Some(chrono::Utc::now().to_rfc3339());
        self.save()
    }

    /// Parse the stored timestamp into DateTime<Utc>
    pub fn get_manifest_timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        use chrono::DateTime;
        self.manifest_last_fetched
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.session.use_keyring.is_none());
        assert_eq!(config.enclave.verify_attestation, true);
        assert_eq!(config.enclave.transport_key_ttl_minutes, 15);
    }

    #[test]
    fn test_config_roundtrip() {
        let config = Config {
            api_key: None,
            session: SessionConfig {
                use_keyring: Some(true),
            },
            manifest_last_fetched: None,
            enclave: EnclaveConfig::default(),
            agent_id: Some(Uuid::new_v4()),
            mcp_server_enabled: false,
        };

        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.session.use_keyring, Some(true));
        assert_eq!(parsed.enclave.verify_attestation, true);
        assert_eq!(parsed.enclave.transport_key_ttl_minutes, 15);
        assert_ne!(parsed.agent_id, Some(Uuid::new_v4()));
    }

    #[test]
    fn test_enclave_config_custom_values() {
        let config = Config {
            api_key: None,
            session: SessionConfig::default(),
            manifest_last_fetched: None,
            enclave: EnclaveConfig {
                verify_attestation: false,
                transport_key_ttl_minutes: 30,
            },
            agent_id: None,
            mcp_server_enabled: false,
        };

        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.enclave.verify_attestation, false);
        assert_eq!(parsed.enclave.transport_key_ttl_minutes, 30);
        assert_eq!(parsed.agent_id, None);
    }

    #[test]
    fn test_backward_compatibility_no_agent_id() {
        // Config TOML without agent_id (simulating existing config file)
        let toml_str = r#"
api_key = "some-api-key"

[session]
use_keyring = false

[enclave]
verify_attestation = true
transport_key_ttl_minutes = 15
"#;

        let parsed: Config = toml::from_str(toml_str).unwrap();

        // agent_id should default to None when not present in the config file
        assert_eq!(parsed.agent_id, None);
        assert_eq!(parsed.api_key, Some("some-api-key".to_string()));
        assert_eq!(parsed.session.use_keyring, Some(false));
        assert_eq!(parsed.enclave.verify_attestation, true);
        assert_eq!(parsed.enclave.transport_key_ttl_minutes, 15);
    }

    #[test]
    fn test_config_load_with_explicit_path() {
        // Create a temp config file
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
[session]
use_keyring = true
"#;
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        // Load with explicit path
        let config = Config::load(Some(temp_file.path().to_path_buf())).unwrap();
        assert_eq!(config.session.use_keyring, Some(true));
    }

    #[test]
    fn test_config_load_returns_default_when_file_doesnt_exist() {
        // Try to load from a non-existent path
        let config = Config::load(Some(PathBuf::from("/nonexistent/path/config.toml"))).unwrap();

        // Should return default config
        assert!(config.session.use_keyring.is_none());
        assert_eq!(config.enclave.verify_attestation, true);
    }
}
