//! Server configuration module for feature flags and persistence.
//!
//! This module provides:
//! - Feature flag management for server mode
//! - Configuration persistence via serde serialization
//! - Methods for querying and modifying feature states
//!
//! Features can be toggled at runtime to enable/disable specific
//! server functionality without restart.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::state::events::ServerFeature;

/// Server feature configuration with feature flags.
///
/// This struct holds the feature flags for MCP server mode,
/// including which features are enabled/disabled. All features
/// are enabled by default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureServerConfig {
    /// Feature flags map - each feature can be enabled or disabled.
    #[serde(default = "default_feature_flags")]
    pub feature_flags: HashMap<ServerFeature, bool>,
}

impl Default for FeatureServerConfig {
    /// Creates a new FeatureServerConfig with all features enabled.
    ///
    /// # Example
    /// ```
    /// use poseidon::state::FeatureServerConfig;
    /// use poseidon::state::ServerFeature;
    ///
    /// let config = FeatureServerConfig::default();
    /// assert!(config.is_feature_enabled(ServerFeature::McpServer));
    /// ```
    fn default() -> Self {
        Self {
            feature_flags: default_feature_flags(),
        }
    }
}

impl FeatureServerConfig {
    /// Creates a new FeatureServerConfig with all features enabled.
    ///
    /// This is equivalent to `Default::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if a specific feature is enabled.
    ///
    /// # Arguments
    /// * `feature` - The feature to check
    ///
    /// # Returns
    /// `true` if the feature is enabled, `false` otherwise.
    /// If the feature is not in the map, returns `false`.
    ///
    /// # Example
    /// ```
    /// use poseidon::state::{FeatureServerConfig, ServerFeature};
    ///
    /// let config = FeatureServerConfig::default();
    /// assert!(config.is_feature_enabled(ServerFeature::McpServer));
    /// ```
    pub fn is_feature_enabled(&self, feature: ServerFeature) -> bool {
        self.feature_flags.get(&feature).copied().unwrap_or(false)
    }

    /// Toggles a feature's enabled state.
    ///
    /// If the feature exists, its state is flipped and the new
    /// state is returned. If the feature doesn't exist, it is
    /// created with `true`.
    ///
    /// # Arguments
    /// * `feature` - The feature to toggle
    ///
    /// # Returns
    /// The new enabled state of the feature after toggling.
    ///
    /// # Example
    /// ```
    /// use poseidon::state::{FeatureServerConfig, ServerFeature};
    ///
    /// let mut config = FeatureServerConfig::default();
    /// let new_state = config.toggle_feature(ServerFeature::Alerts);
    /// assert!(!new_state); // Was true, now false
    /// ```
    pub fn toggle_feature(&mut self, feature: ServerFeature) -> bool {
        let current = self.feature_flags.get(&feature).copied().unwrap_or(false);
        let new_state = !current;
        self.feature_flags.insert(feature, new_state);
        new_state
    }

    /// Sets a feature to a specific enabled state.
    ///
    /// # Arguments
    /// * `feature` - The feature to modify
    /// * `enabled` - Whether to enable or disable the feature
    ///
    /// # Example
    /// ```
    /// use poseidon::state::{FeatureServerConfig, ServerFeature};
    ///
    /// let mut config = FeatureServerConfig::default();
    /// config.set_feature(ServerFeature::Subscriptions, false);
    /// assert!(!config.is_feature_enabled(ServerFeature::Subscriptions));
    /// ```
    pub fn set_feature(&mut self, feature: ServerFeature, enabled: bool) {
        self.feature_flags.insert(feature, enabled);
    }

    /// Returns a list of all enabled features.
    ///
    /// # Returns
    /// A vector of features that are currently enabled.
    pub fn enabled_features(&self) -> Vec<ServerFeature> {
        self.feature_flags
            .iter()
            .filter(|(_, enabled)| **enabled)
            .map(|(feature, _)| *feature)
            .collect()
    }

    /// Returns a list of all disabled features.
    ///
    /// # Returns
    /// A vector of features that are currently disabled.
    pub fn disabled_features(&self) -> Vec<ServerFeature> {
        self.feature_flags
            .iter()
            .filter(|(_, enabled)| !**enabled)
            .map(|(feature, _)| *feature)
            .collect()
    }
}

/// Creates default feature flags with all features enabled.
fn default_feature_flags() -> HashMap<ServerFeature, bool> {
    let mut flags = HashMap::new();
    flags.insert(ServerFeature::McpServer, true);
    flags.insert(ServerFeature::Subscriptions, true);
    flags.insert(ServerFeature::Alerts, true);
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_server_config() {
        let config = FeatureServerConfig::default();

        // All features should be enabled by default
        assert!(config.is_feature_enabled(ServerFeature::McpServer));
        assert!(config.is_feature_enabled(ServerFeature::Subscriptions));
        assert!(config.is_feature_enabled(ServerFeature::Alerts));
    }

    #[test]
    fn test_new_creates_default_config() {
        let new_config = FeatureServerConfig::new();
        let default_config = FeatureServerConfig::default();

        assert_eq!(
            new_config.is_feature_enabled(ServerFeature::McpServer),
            default_config.is_feature_enabled(ServerFeature::McpServer)
        );
    }

    #[test]
    fn test_is_feature_enabled_returns_false_for_unknown() {
        let mut config = FeatureServerConfig::default();
        // Remove all features
        config.feature_flags.clear();

        assert!(!config.is_feature_enabled(ServerFeature::McpServer));
    }

    #[test]
    fn test_set_feature() {
        let mut config = FeatureServerConfig::default();

        // Disable a feature
        config.set_feature(ServerFeature::Alerts, false);
        assert!(!config.is_feature_enabled(ServerFeature::Alerts));

        // Re-enable it
        config.set_feature(ServerFeature::Alerts, true);
        assert!(config.is_feature_enabled(ServerFeature::Alerts));
    }

    #[test]
    fn test_toggle_feature() {
        let mut config = FeatureServerConfig::default();

        // Toggle from true to false
        let result = config.toggle_feature(ServerFeature::McpServer);
        assert!(!result);
        assert!(!config.is_feature_enabled(ServerFeature::McpServer));

        // Toggle back from false to true
        let result = config.toggle_feature(ServerFeature::McpServer);
        assert!(result);
        assert!(config.is_feature_enabled(ServerFeature::McpServer));
    }

    #[test]
    fn test_toggle_feature_creates_if_missing() {
        let mut config = FeatureServerConfig::default();
        config.feature_flags.clear();

        // Feature doesn't exist, toggle creates it with true
        let result = config.toggle_feature(ServerFeature::Subscriptions);
        assert!(result);
    }

    #[test]
    fn test_enabled_features() {
        let mut config = FeatureServerConfig::default();
        config.set_feature(ServerFeature::Alerts, false);

        let enabled = config.enabled_features();
        assert!(enabled.contains(&ServerFeature::McpServer));
        assert!(enabled.contains(&ServerFeature::Subscriptions));
        assert!(!enabled.contains(&ServerFeature::Alerts));
    }

    #[test]
    fn test_disabled_features() {
        let mut config = FeatureServerConfig::default();
        config.set_feature(ServerFeature::Subscriptions, false);
        config.set_feature(ServerFeature::Alerts, false);

        let disabled = config.disabled_features();
        assert_eq!(disabled.len(), 2);
        assert!(disabled.contains(&ServerFeature::Subscriptions));
        assert!(disabled.contains(&ServerFeature::Alerts));
    }

    #[test]
    fn test_server_feature_display() {
        assert_eq!(format!("{}", ServerFeature::McpServer), "MCP Server");
        assert_eq!(format!("{}", ServerFeature::Subscriptions), "Subscriptions");
        assert_eq!(format!("{}", ServerFeature::Alerts), "Alerts");
    }

    #[test]
    fn test_serde_serialization() {
        let config = FeatureServerConfig::default();
        let json = serde_json::to_string(&config).unwrap();

        // Should contain feature_flags field
        assert!(json.contains("feature_flags"));

        // Deserialize and verify
        let deserialized: FeatureServerConfig = serde_json::from_str(&json).unwrap();
        assert!(deserialized.is_feature_enabled(ServerFeature::McpServer));
        assert!(deserialized.is_feature_enabled(ServerFeature::Subscriptions));
        assert!(deserialized.is_feature_enabled(ServerFeature::Alerts));
    }

    #[test]
    fn test_serde_with_partial_features() {
        // Simulate deserializing with missing features
        let json = r#"{"feature_flags": {"McpServer": true}}"#;
        let config: FeatureServerConfig = serde_json::from_str(json).unwrap();

        // Missing features should not be present
        assert!(config.is_feature_enabled(ServerFeature::McpServer));
        assert!(!config.is_feature_enabled(ServerFeature::Alerts)); // Missing = false
    }

    #[test]
    fn test_server_feature_hashmap_key() {
        // Verify ServerFeature can be used as HashMap key
        let mut map = HashMap::new();
        map.insert(ServerFeature::McpServer, "enabled");
        map.insert(ServerFeature::Alerts, "disabled");

        assert_eq!(map.get(&ServerFeature::McpServer), Some(&"enabled"));
        assert_eq!(map.get(&ServerFeature::Alerts), Some(&"disabled"));
    }

    #[test]
    fn test_server_config_clone() {
        let config = FeatureServerConfig::default();
        let cloned = config.clone();

        assert_eq!(
            config.is_feature_enabled(ServerFeature::McpServer),
            cloned.is_feature_enabled(ServerFeature::McpServer)
        );
    }
}
