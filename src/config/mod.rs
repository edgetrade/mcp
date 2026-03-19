//! Configuration management for Edge CLI.
//!
//! Handles loading and saving of configuration from the XDG config directory
//! or a user-specified location via `--config` flag or `EDGE_CONFIG` env var.
//! Configuration includes session storage preferences and keyring availability.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// Default config directory name
pub const CONFIG_DIR_NAME: &str = "edge";
/// Default config file name
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Global override for config file path (set via --config flag or EDGE_CONFIG env var)
static CONFIG_PATH_OVERRIDE: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Set the global config file path override.
///
/// This should be called early in main() before any config operations.
/// Once set, all config operations will use this path instead of the default.
pub fn set_config_path_override(path: Option<PathBuf>) {
    let _ = CONFIG_PATH_OVERRIDE.set(path);
}

/// Edge CLI configuration.
///
/// This struct represents the user-configurable settings for the Edge CLI,
/// stored in `~/.config/edge/config.toml` (XDG config directory).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Session storage configuration
    #[serde(default)]
    pub session: SessionConfig,
    /// ISO 8601 timestamp of last manifest fetch
    #[serde(default)]
    pub manifest_last_fetched: Option<String>,
    /// Enclave security configuration
    #[serde(default)]
    pub enclave: EnclaveConfig,
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
    /// Load configuration from the XDG config directory.
    ///
    /// Attempts to load from `~/.config/edge/config.toml`. If the file
    /// doesn't exist, returns a default configuration.
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
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
        let config_path = Self::config_path()?;
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
    /// First checks for a global override set via `set_config_path_override()`.
    /// If no override is set, returns the platform-specific XDG config path:
    /// - Linux: `~/.config/edge/config.toml`
    /// - macOS: `~/Library/Application Support/edge/config.toml`
    /// - Windows: `%APPDATA%\edge\config.toml`
    ///
    /// # Returns
    /// - `Ok(PathBuf)` - The path to the config file
    /// - `Err(ConfigError)` - If the config directory cannot be determined
    pub fn config_path() -> Result<PathBuf, ConfigError> {
        // Check for global override first
        if let Some(override_path) = CONFIG_PATH_OVERRIDE.get()
            && let Some(path) = override_path
        {
            return Ok(path.clone());
        }

        // Fall back to XDG config directory
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join(CONFIG_DIR_NAME);
        Ok(config_dir.join(CONFIG_FILE_NAME))
    }

    /// Check if the configuration file exists.
    pub fn exists() -> bool {
        Self::config_path().map(|p| p.exists()).unwrap_or(false)
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

/// Determine whether to use keyring based on configuration.
///
/// This function checks the config file first. If `session.use_keyring`
/// is set, it returns that value. If not set (None), it probes the OS
/// keyring to determine availability, saves the result to config, and
/// returns the detected value.
///
/// This avoids probing the OS keyring on every command execution.
pub fn should_use_keyring() -> bool {
    // Try to load existing config
    match Config::load() {
        Ok(config) => {
            // If config has explicit setting, use it
            if let Some(use_keyring) = config.session.use_keyring {
                return use_keyring;
            }

            // Otherwise, detect and save
            let available = crate::session::keyring_available();
            let mut new_config = config;
            new_config.session.use_keyring = Some(available);

            // Save the detected value (ignore errors, we'll just probe next time)
            let _ = new_config.save();

            available
        }
        Err(_) => {
            // If we can't load config, just probe and return
            crate::session::keyring_available()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            session: SessionConfig {
                use_keyring: Some(true),
            },
            manifest_last_fetched: None,
            enclave: EnclaveConfig::default(),
        };

        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.session.use_keyring, Some(true));
        assert_eq!(parsed.enclave.verify_attestation, true);
        assert_eq!(parsed.enclave.transport_key_ttl_minutes, 15);
    }

    #[test]
    fn test_enclave_config_custom_values() {
        let config = Config {
            session: SessionConfig::default(),
            manifest_last_fetched: None,
            enclave: EnclaveConfig {
                verify_attestation: false,
                transport_key_ttl_minutes: 30,
            },
        };

        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.enclave.verify_attestation, false);
        assert_eq!(parsed.enclave.transport_key_ttl_minutes, 30);
    }
}
