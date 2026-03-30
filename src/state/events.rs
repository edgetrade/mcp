//! State events module for event-driven state notifications.
//!
//! This module provides:
//! - Event types for state changes (config, session, server config)
//! - Broadcast channel for distributing events to multiple subscribers
//!
//! Events are sent whenever state changes occur, allowing components
//! to react to state transitions without polling.

use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::broadcast;

/// Type alias for configuration keys.
pub type ConfigKey = String;

/// Type alias for configuration values.
pub type ConfigValue = serde_json::Value;

/// Features that can be enabled/disabled in server mode.
///
/// These features control which functionality is available when
/// running as an MCP server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServerFeature {
    /// MCP server functionality (stdio or HTTP transport).
    McpServer,
    /// Real-time subscription handling.
    Subscriptions,
    /// Alert delivery and management.
    Alerts,
}

impl fmt::Display for ServerFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerFeature::McpServer => write!(f, "MCP Server"),
            ServerFeature::Subscriptions => write!(f, "Subscriptions"),
            ServerFeature::Alerts => write!(f, "Alerts"),
        }
    }
}

/// Events emitted when application state changes.
///
/// These events allow subscribers to react to state changes without
/// polling. All events are Clone + Send + Sync for safe use across
/// async boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateEvent {
    /// Configuration value changed.
    ConfigChanged {
        /// The configuration key that changed.
        key: ConfigKey,
        /// The new configuration value.
        value: ConfigValue,
    },
    /// Session was unlocked (user authenticated).
    SessionUnlocked,
    /// Session was locked (user logged out).
    SessionLocked,
    /// Server feature configuration changed.
    ServerConfigChanged {
        /// The feature that was toggled.
        feature: ServerFeature,
        /// Whether the feature is now enabled.
        enabled: bool,
    },
}

impl StateEvent {
    /// Returns a short display name for the event type.
    pub fn event_name(&self) -> &'static str {
        match self {
            StateEvent::ConfigChanged { .. } => "config_changed",
            StateEvent::SessionUnlocked => "session_unlocked",
            StateEvent::SessionLocked => "session_locked",
            StateEvent::ServerConfigChanged { .. } => "server_config_changed",
        }
    }
}

/// Sender handle for broadcasting state events.
///
/// Clone this handle to distribute events to multiple subscribers.
/// The channel has a bounded capacity; old events are dropped when
/// receivers lag behind.
pub type StateEventSender = broadcast::Sender<StateEvent>;

/// Receiver for state events.
///
/// Each receiver gets a clone of events sent to the channel.
/// Receivers that lag behind may miss events.
pub type StateEventReceiver = broadcast::Receiver<StateEvent>;

/// Create a new broadcast channel for state events.
///
/// # Arguments
/// * `capacity` - The buffer size for the channel. When full, oldest
///   events are dropped for lagging receivers.
///
/// # Returns
/// A tuple of (sender, receiver) for the channel.
///
/// # Example
/// ```rust,no_run
/// use poseidon::state::events::create_state_event_channel;
///
/// # async fn example() {
/// let (tx, mut rx) = create_state_event_channel(100);
/// # }
/// ```
pub fn create_state_event_channel(capacity: usize) -> (StateEventSender, StateEventReceiver) {
    broadcast::channel(capacity)
}

/// Trait for types that can emit state events.
///
/// Implement this trait on components that need to publish state changes.
pub trait StateEventEmitter {
    /// Emit a state event to all subscribers.
    ///
    /// # Arguments
    /// * `event` - The event to emit
    ///
    /// # Returns
    /// - `Ok(usize)` - The number of receivers that got the event
    /// - `Err(_)` - If all receivers have been dropped
    fn emit_event(&self, event: StateEvent) -> Result<usize, broadcast::error::SendError<StateEvent>>;
}

impl StateEventEmitter for StateEventSender {
    fn emit_event(&self, event: StateEvent) -> Result<usize, broadcast::error::SendError<StateEvent>> {
        self.send(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_server_feature_variants() {
        let features = vec![
            ServerFeature::McpServer,
            ServerFeature::Subscriptions,
            ServerFeature::Alerts,
        ];

        // Verify all variants are Clone
        let cloned: Vec<ServerFeature> = features.iter().cloned().collect();
        assert_eq!(cloned.len(), 3);
    }

    #[test]
    fn test_state_event_variants() {
        let events = vec![
            StateEvent::ConfigChanged {
                key: "test.key".to_string(),
                value: json!("test_value"),
            },
            StateEvent::SessionUnlocked,
            StateEvent::SessionLocked,
            StateEvent::ServerConfigChanged {
                feature: ServerFeature::McpServer,
                enabled: true,
            },
        ];

        // Verify all variants are Clone
        let cloned: Vec<StateEvent> = events.iter().cloned().collect();
        assert_eq!(cloned.len(), 4);
    }

    #[test]
    fn test_state_event_names() {
        assert_eq!(
            StateEvent::ConfigChanged {
                key: "x".to_string(),
                value: json!(1),
            }
            .event_name(),
            "config_changed"
        );
        assert_eq!(StateEvent::SessionUnlocked.event_name(), "session_unlocked");
        assert_eq!(StateEvent::SessionLocked.event_name(), "session_locked");
        assert_eq!(
            StateEvent::ServerConfigChanged {
                feature: ServerFeature::McpServer,
                enabled: true,
            }
            .event_name(),
            "server_config_changed"
        );
    }

    #[test]
    fn test_server_feature_serialization() {
        let feature = ServerFeature::Subscriptions;
        let json = serde_json::to_string(&feature).unwrap();
        assert_eq!(json, "\"Subscriptions\"");

        let deserialized: ServerFeature = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ServerFeature::Subscriptions);
    }

    #[test]
    fn test_state_event_serialization() {
        let event = StateEvent::ServerConfigChanged {
            feature: ServerFeature::Alerts,
            enabled: false,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Alerts"));
        assert!(json.contains("false"));

        let deserialized: StateEvent = serde_json::from_str(&json).unwrap();
        match deserialized {
            StateEvent::ServerConfigChanged { feature, enabled } => {
                assert_eq!(feature, ServerFeature::Alerts);
                assert!(!enabled);
            }
            _ => panic!("Deserialization produced wrong variant"),
        }
    }

    #[tokio::test]
    async fn test_broadcast_channel() {
        let (tx, mut rx) = create_state_event_channel(10);

        // Send an event
        let event = StateEvent::SessionUnlocked;
        let sent_count = tx.send(event.clone()).unwrap();
        assert_eq!(sent_count, 1);

        // Receive the event
        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_name(), "session_unlocked");
    }

    #[tokio::test]
    async fn test_multiple_receivers() {
        let (tx, mut rx1) = create_state_event_channel(10);
        let mut rx2 = tx.subscribe();

        // Send event
        let event = StateEvent::SessionLocked;
        let sent_count = tx.send(event.clone()).unwrap();
        assert_eq!(sent_count, 2);

        // Both receivers should get the event
        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();
        assert_eq!(received1.event_name(), received2.event_name());
    }

    #[test]
    fn test_state_event_emitter_trait() {
        let (tx, _rx) = create_state_event_channel(10);

        let event = StateEvent::ConfigChanged {
            key: "api.url".to_string(),
            value: json!("https://example.com"),
        };

        let result = tx.emit_event(event);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    // Verify Send + Sync bounds
    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_send_sync_bounds() {
        assert_send_sync::<StateEvent>();
        assert_send_sync::<ServerFeature>();
        assert_send_sync::<ConfigKey>();
        assert_send_sync::<ConfigValue>();
    }
}
