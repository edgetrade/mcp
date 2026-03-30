//! Core state management for the Edge CLI application.
//!
//! This module provides a thread-safe singleton state that holds:
//! - Configuration loaded from disk
//! - Session for user authentication
//! - Client for API communication
//! - Server configuration for MCP mode
//!
//! The state is designed as a singleton pattern using `OnceLock` for safe
//! global access with thread-safe interior mutability via `Arc<RwLock<_>>`.

pub mod events;
pub mod server_config;

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use tokio::sync::RwLock;

use crate::client::IrisClient;
use crate::config::Config;
use crate::session::Session;

pub use events::{
    ConfigKey, ConfigValue, ServerFeature, StateEvent, StateEventEmitter, StateEventReceiver, StateEventSender,
    create_state_event_channel,
};
pub use server_config::FeatureServerConfig;

/// Global singleton instance of the Edge state.
///
/// This static holds the initialized state once `init()` has been called.
/// Access the state via `EdgeState::get()` after initialization.
static EDGE_STATE: OnceLock<EdgeState> = OnceLock::new();

/// Errors that can occur during state operations.
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    /// Failed to initialize the state.
    #[error("Failed to initialize state: {0}")]
    InitializationError(String),

    /// Failed to acquire a lock on the state.
    #[error("Failed to acquire state lock: {0}")]
    LockError(String),

    /// Configuration-related error.
    #[error("Configuration error: {0}")]
    ConfigError(#[from] crate::config::ConfigError),

    /// Session-related error.
    #[error("Session error: {0}")]
    SessionError(#[from] crate::session::SessionError),

    /// Client connection error.
    #[error("Client error: {0}")]
    ClientError(String),

    /// State already initialized.
    #[error("State has already been initialized")]
    AlreadyInitialized,

    /// State not yet initialized.
    #[error("State has not been initialized")]
    NotInitialized,
}

/// Configuration for the MCP server mode.
///
/// This holds settings for running the Edge CLI as an MCP server
/// via stdio or HTTP transport.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host address for HTTP server mode.
    pub host: String,
    /// Port for HTTP server mode.
    pub port: u16,
    /// Path prefix for HTTP endpoints.
    pub path: String,
    /// Whether to run in stdio mode (vs HTTP).
    pub stdio_mode: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            path: "/mcp".to_string(),
            stdio_mode: true,
        }
    }
}

/// Inner state holding all application state.
///
/// This struct contains the actual data protected by the RwLock.
/// Access should go through `EdgeState` methods.
pub struct StateInner {
    /// Application configuration loaded from disk.
    pub config: Config,
    /// User session for authentication.
    pub session: Session,
    /// Optional API client for communication with the Edge backend.
    ///
    /// This is populated after successful authentication.
    pub client: Option<IrisClient>,
    /// Server configuration when running in MCP mode.
    pub server_config: ServerConfig,
    /// Feature server configuration for feature flags.
    pub feature_server_config: FeatureServerConfig,
    /// Event sender for broadcasting state changes.
    pub event_sender: StateEventSender,
}

impl std::fmt::Debug for StateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateInner")
            .field("config", &self.config)
            .field("session", &self.session)
            .field("client", &self.client.is_some())
            .field("server_config", &self.server_config)
            .field("feature_server_config", &self.feature_server_config)
            .finish()
    }
}

/// Thread-safe handle to the global application state.
///
/// This struct provides a thread-safe wrapper around `StateInner` using
/// `Arc<RwLock<_>>`. Clone this handle to share access across threads.
#[derive(Debug, Clone)]
pub struct EdgeState {
    inner: Arc<RwLock<StateInner>>,
}

impl EdgeState {
    /// Initialize the global state singleton.
    ///
    /// This method must be called once at application startup before
    /// accessing the state via `get()`. Subsequent calls will return
    /// `StateError::AlreadyInitialized`.
    ///
    /// # Arguments
    /// * `config_path` - Optional custom path to the configuration file.
    ///   If `None`, uses the default XDG config directory.
    ///
    /// # Returns
    /// - `Ok(Self)` - A clone of the initialized state handle
    /// - `Err(StateError)` - If initialization fails
    ///
    /// # Example
    /// ```rust,no_run
    /// use poseidon::state::EdgeState;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let state = EdgeState::init(None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn init(config_path: Option<PathBuf>) -> Result<Self, StateError> {
        // Initialize the state with the provided config path
        let state = Self::new(config_path).await?;

        // Store in the global singleton
        EDGE_STATE
            .set(state.clone())
            .map_err(|_| StateError::AlreadyInitialized)?;

        Ok(state)
    }

    /// Get the global state singleton.
    ///
    /// Returns a reference to the initialized state. Must call `init()`
    /// before accessing this method.
    ///
    /// # Returns
    /// - `Ok(&EdgeState)` - Reference to the global state
    /// - `Err(StateError::NotInitialized)` - If `init()` hasn't been called
    pub fn get() -> Result<&'static EdgeState, StateError> {
        EDGE_STATE.get().ok_or(StateError::NotInitialized)
    }

    /// Check if the state has been initialized.
    ///
    /// Returns `true` if `init()` has been called successfully.
    pub fn is_initialized() -> bool {
        EDGE_STATE.get().is_some()
    }

    /// Create a new state instance (internal constructor).
    ///
    /// This loads the configuration and initializes the session.
    /// For most use cases, use `init()` instead.
    ///
    /// # Returns
    /// - `Ok(EdgeState)` - New state instance
    /// - `Err(StateError)` - If loading config or session fails
    async fn new(config_path: Option<PathBuf>) -> Result<Self, StateError> {
        // Load configuration from disk
        let config = Config::load(config_path)?;

        // Initialize session with automatic backend selection
        let session = Session::new(config.clone());

        // Create the event channel
        let (event_sender, _) = create_state_event_channel(128);

        // Create the inner state
        // Load feature configuration from config file if present
        let feature_server_config = load_feature_config(&config).unwrap_or_default();

        let inner = StateInner {
            config,
            session,
            client: None,
            server_config: ServerConfig::default(),
            feature_server_config,
            event_sender,
        };

        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
        })
    }

    /// Get a read lock on the inner state.
    ///
    /// # Returns
    /// - `RwLockReadGuard<StateInner>` - Read access to the state
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, StateInner> {
        self.inner.read().await
    }

    /// Get a write lock on the inner state.
    ///
    /// # Returns
    /// - `RwLockWriteGuard<StateInner>` - Write access to the state
    pub async fn write(&self) -> tokio::sync::RwLockWriteGuard<'_, StateInner> {
        self.inner.write().await
    }

    /// Check if the session is unlocked (user is authenticated).
    ///
    /// This is a convenience method that acquires a read lock internally.
    ///
    /// # Returns
    /// `true` if the session has an active user encryption key.
    pub async fn is_authenticated(&self) -> bool {
        let inner = self.read().await;
        inner.session.is_unlocked()
    }

    /// Get a clone of the application configuration.
    ///
    /// # Returns
    /// - `Ok(Config)` - A clone of the configuration
    /// - `Err(StateError::LockError)` - If the lock cannot be acquired
    pub async fn get_config(&self) -> Result<Config, StateError> {
        let inner = self.read().await;
        Ok(inner.config.clone())
    }

    /// Get a clone of the user session.
    ///
    /// # Returns
    /// - `Ok(Session)` - A clone of the session
    /// - `Err(StateError::LockError)` - If the lock cannot be acquired
    pub async fn get_session(&self) -> Result<Session, StateError> {
        let inner = self.read().await;
        Ok(inner.session.clone())
    }

    /// Get an optional clone of the API client.
    ///
    /// Similar to `get_client()` but returns `None` instead of an error
    /// when the client is not initialized.
    ///
    /// # Returns
    /// - `Ok(Some(IrisClient))` - The client if initialized
    /// - `Ok(None)` - If the client is not yet initialized
    /// - `Err(StateError::LockError)` - If the lock cannot be acquired
    pub async fn get_client_opt(&self) -> Result<Option<IrisClient>, StateError> {
        let inner = self.read().await;
        Ok(inner.client.clone())
    }

    /// Check if a specific server feature is enabled.
    ///
    /// This checks the `FeatureServerConfig` which is stored separately
    /// from the basic `ServerConfig`. The feature flags control which
    /// functionality is available when running as an MCP server.
    ///
    /// # Arguments
    /// * `feature` - The feature to check
    ///
    /// # Returns
    /// - `Ok(true)` - If the feature is enabled in `FeatureServerConfig`
    /// - `Ok(false)` - If the feature is disabled or not present
    /// - `Err(StateError::LockError)` - If the lock cannot be acquired
    pub async fn is_server_feature_enabled(&self, feature: ServerFeature) -> Result<bool, StateError> {
        let inner = self.read().await;
        Ok(inner.feature_server_config.is_feature_enabled(feature))
    }

    /// Check if the session is unlocked.
    ///
    /// Similar to `is_authenticated()` but returns a `Result` for
    /// consistency with other state read operations.
    ///
    /// # Returns
    /// - `Ok(true)` - If the session has an active user encryption key
    /// - `Ok(false)` - If the session is locked
    /// - `Err(StateError::LockError)` - If the lock cannot be acquired
    pub async fn is_session_unlocked(&self) -> Result<bool, StateError> {
        let inner = self.read().await;
        Ok(inner.session.is_unlocked())
    }

    /// Set the API client after successful authentication.
    ///
    /// # Arguments
    /// * `client` - The initialized IrisClient
    pub async fn set_client(&self, client: IrisClient) {
        let mut inner = self.write().await;
        inner.client = Some(client);
    }

    /// Get a clone of the API client if available.
    ///
    /// # Returns
    /// - `Ok(IrisClient)` - The client instance
    /// - `Err(StateError::InitializationError)` - If client not set
    pub async fn get_client(&self) -> Result<IrisClient, StateError> {
        let inner = self.read().await;
        inner
            .client
            .clone()
            .ok_or_else(|| StateError::InitializationError("Client not initialized".to_string()))
    }

    /// Configure the server settings for MCP mode.
    ///
    /// # Arguments
    /// * `config` - The server configuration
    pub async fn set_server_config(&self, config: ServerConfig) {
        let mut inner = self.write().await;
        inner.server_config = config;
    }

    /// Subscribe to state events.
    ///
    /// Returns a receiver that can be used to listen for state changes.
    /// Each subscriber receives a clone of events sent to the channel.
    ///
    /// # Returns
    /// A `StateEventReceiver` for receiving state events.
    pub fn subscribe(&self) -> StateEventReceiver {
        self.inner.blocking_read().event_sender.subscribe()
    }

    /// Update the configuration with a callback function.
    ///
    /// The callback receives a mutable reference to the config, allowing
    /// modifications. After the callback completes, the config is persisted
    /// to disk.
    ///
    /// # Arguments
    /// * `f` - A callback function that receives `&mut Config`
    ///
    /// # Returns
    /// - `Ok(())` - On success
    /// - `Err(StateError)` - If the config cannot be saved
    pub async fn update_config<F>(&self, f: F) -> Result<(), StateError>
    where
        F: FnOnce(&mut Config),
    {
        let mut inner = self.write().await;
        f(&mut inner.config);
        inner.config.save()?;
        // TODO: Emit ConfigChanged event for changed fields
        Ok(())
    }

    /// Toggle a server feature on or off.
    ///
    /// This updates the feature state in `FeatureServerConfig` and persists
    /// the change to disk via the config file.
    ///
    /// # Arguments
    /// * `feature` - The feature to toggle
    /// * `enabled` - Whether to enable or disable the feature
    ///
    /// # Returns
    /// - `Ok(())` - On success
    /// - `Err(StateError)` - If the lock cannot be acquired or save fails
    pub async fn toggle_server_feature(&self, feature: ServerFeature, enabled: bool) -> Result<(), StateError> {
        let mut inner = self.write().await;

        // Update the feature state
        inner.feature_server_config.set_feature(feature, enabled);

        // Update the config's mcp_server_enabled field based on McpServer feature state
        inner.config.mcp_server_enabled = inner
            .feature_server_config
            .is_feature_enabled(ServerFeature::McpServer);

        // Save the config to disk
        inner.config.save()?;

        // Emit ServerConfigChanged event
        let _ = inner
            .event_sender
            .send(StateEvent::ServerConfigChanged { feature, enabled });

        Ok(())
    }

    /// Unlock the session with a password.
    ///
    /// # Arguments
    /// * `password` - The user's password to derive the encryption key from
    ///
    /// # Returns
    /// - `Ok(())` - On successful unlock
    /// - `Err(StateError)` - If unlocking fails
    pub async fn unlock_session(&self, password: &str) -> Result<(), StateError> {
        let inner = self.write().await;
        inner.session.unlock_with_password(password)?;
        // Emit SessionUnlocked event
        let _ = inner.event_sender.send(StateEvent::SessionUnlocked);
        Ok(())
    }

    /// Lock the session.
    ///
    /// This clears the client and locks the session.
    ///
    /// # Returns
    /// - `Ok(())` - On successful lock
    /// - `Err(StateError)` - If locking fails
    pub async fn lock_session(&self) -> Result<(), StateError> {
        let mut inner = self.write().await;
        inner.session.lock()?;
        inner.client = None;
        // Emit SessionLocked event
        let _ = inner.event_sender.send(StateEvent::SessionLocked);
        Ok(())
    }

    /// Initialize the API client.
    ///
    /// Creates a new IrisClient with the provided credentials and stores it
    /// in the state.
    ///
    /// # Arguments
    /// * `api_key` - The API key for authentication
    /// * `url` - The server URL
    ///
    /// # Returns
    /// - `Ok(())` - On successful initialization
    /// - `Err(StateError)` - If client creation fails
    pub async fn initialize_client(&self, api_key: String, url: String) -> Result<(), StateError> {
        let client = IrisClient::connect(&url, &api_key, false)
            .await
            .map_err(|e| StateError::ClientError(e.to_string()))?;
        let mut inner = self.write().await;
        inner.client = Some(client);
        Ok(())
    }

    /// Get a clone of the feature server configuration.
    ///
    /// # Returns
    /// - `Ok(FeatureServerConfig)` - A clone of the feature server configuration
    /// - `Err(StateError::LockError)` - If the lock cannot be acquired
    pub async fn get_feature_server_config(&self) -> Result<FeatureServerConfig, StateError> {
        let inner = self.read().await;
        Ok(inner.feature_server_config.clone())
    }

    /// Set the feature server configuration.
    ///
    /// # Arguments
    /// * `config` - The feature server configuration to set
    pub async fn set_feature_server_config(&self, config: FeatureServerConfig) {
        let mut inner = self.write().await;
        inner.feature_server_config = config;
    }
}

/// Load feature configuration from the config file.
///
/// The feature configuration is stored as a JSON-serialized field
/// in the config file.
///
/// # Arguments
/// * `config` - The application configuration
///
/// # Returns
/// - `Ok(Some(FeatureServerConfig))` - If feature configuration was found and parsed
/// - `Ok(None)` - If no feature configuration was found
/// - `Err(ConfigError)` - If there was an error loading the config
fn load_feature_config(config: &Config) -> Option<FeatureServerConfig> {
    // For now, we use the `mcp_server_enabled` field in Config as the source of truth
    // for the McpServer feature. Other features default to enabled.
    let mut feature_flags = std::collections::HashMap::new();

    // McpServer feature state comes from Config.mcp_server_enabled
    feature_flags.insert(ServerFeature::McpServer, config.mcp_server_enabled);

    // Other features default to enabled
    feature_flags.insert(ServerFeature::Subscriptions, true);
    feature_flags.insert(ServerFeature::Alerts, true);

    Some(FeatureServerConfig { feature_flags })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_state_error_display() {
        let err = StateError::NotInitialized;
        assert_eq!(err.to_string(), "State has not been initialized");

        let err = StateError::AlreadyInitialized;
        assert_eq!(err.to_string(), "State has already been initialized");
    }

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.path, "/mcp");
        assert!(config.stdio_mode);
    }

    #[test]
    fn test_is_initialized_before_init() {
        // Note: This test assumes tests run in isolated processes
        // or that no other test has called init()
        assert!(!EdgeState::is_initialized());
    }

    #[test]
    fn test_load_feature_config_from_mcp_server_enabled() {
        // Test when mcp_server_enabled is false (default)
        let config = Config::default();
        let feature_config = load_feature_config(&config).unwrap_or_else(|| FeatureServerConfig {
            feature_flags: HashMap::new(),
        });
        assert!(!feature_config.is_feature_enabled(ServerFeature::McpServer));
        assert!(feature_config.is_feature_enabled(ServerFeature::Subscriptions));
        assert!(feature_config.is_feature_enabled(ServerFeature::Alerts));

        // Test when mcp_server_enabled is true
        let mut config = Config::default();
        config.mcp_server_enabled = true;
        let feature_config = load_feature_config(&config).unwrap_or_else(|| FeatureServerConfig {
            feature_flags: HashMap::new(),
        });
        assert!(feature_config.is_feature_enabled(ServerFeature::McpServer));
    }

    #[test]
    fn test_state_inner_debug_includes_feature_config() {
        let config = Config::default();
        let session = Session::new(config.clone());
        let (event_sender, _) = create_state_event_channel(128);

        // Create feature config with McpServer enabled
        let mut feature_flags = HashMap::new();
        feature_flags.insert(ServerFeature::McpServer, true);
        feature_flags.insert(ServerFeature::Subscriptions, true);
        feature_flags.insert(ServerFeature::Alerts, true);
        let feature_server_config = FeatureServerConfig { feature_flags };

        let inner = StateInner {
            config,
            session,
            client: None,
            server_config: ServerConfig::default(),
            feature_server_config,
            event_sender,
        };

        // Ensure Debug output includes feature_server_config
        let debug_str = format!("{:?}", inner);
        assert!(debug_str.contains("feature_server_config"));
    }
}
