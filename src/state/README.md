# Poseidon State Module

Centralized state management for the Edge CLI application.

## Overview

The state module provides:

1. **`EdgeState`**: Thread-safe singleton for global application state
2. **`StateEvent`**: Event-driven architecture with broadcast channels
3. **`FeatureServerConfig`**: Runtime feature flag management

## Architecture

```
┌─────────────────────────────────────────────┐
│              EdgeState (Singleton)           │
│  ┌──────────────────────────────────────┐  │
│  │  OnceLock<EdgeState>                   │  │
│  │                                       │  │
│  │  ┌────────────────────────────────┐   │  │
│  │  │ Arc<RwLock<StateInner>>         │   │  │
│  │  │                                 │   │  │
│  │  │  ┌────────┐ ┌────────┐         │   │  │
│  │  │  │ Config │ │ Session│         │   │  │
│  │  │  └────────┘ └────────┘         │   │  │
│  │  │                                 │   │  │
│  │  │  ┌────────┐ ┌────────┐         │   │  │
│  │  │  │ Client │ │ Server │         │   │  │
│  │  │  └────────┘ └────────┘         │   │  │
│  │  │                                 │   │  │
│  │  │  ┌────────────────────────┐     │   │  │
│  │  │  │ StateEventSender       │     │   │  │
│  │  │  └────────────────────────┘     │   │  │
│  │  └────────────────────────────────┘   │  │
│  └──────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

## Quick Start

```rust
use poseidon::state::EdgeState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize once at startup
    let state = EdgeState::init(None).await?;
    
    // Access from anywhere
    let state = EdgeState::get()?;
    
    // Check authentication
    if !state.is_authenticated().await {
        println!("Please unlock your session");
    }
    
    // Subscribe to events
    let mut rx = state.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            println!("Event: {:?}", event);
        }
    });
    
    Ok(())
}
```

## Module Structure

| File | Purpose |
|------|---------|
| `mod.rs` | `EdgeState` singleton and `StateInner` struct |
| `events.rs` | `StateEvent` enum and broadcast channel utilities |
| `server_config.rs` | `FeatureServerConfig` for MCP server features |

## Key Types

### EdgeState

Thread-safe handle to global application state using `Arc<RwLock<_>>`.

**Initialization:**
```rust
// Call once at application startup
let state = EdgeState::init(None).await?; // Use default config path
// OR
let state = EdgeState::init(Some(PathBuf::from("/custom/path"))).await?;
```

**Access:**
```rust
// Get reference to initialized state (must call init first)
let state = EdgeState::get()?;

// Check if initialized
if EdgeState::is_initialized() {
    let state = EdgeState::get()?;
}
```

**State Access:**
```rust
// Read lock (multiple concurrent readers)
let inner = state.read().await;
let config = inner.config.clone();

// Write lock (exclusive access)
let mut inner = state.write().await;
inner.client = Some(new_client);
```

**Convenience Methods:**
```rust
// Get cloned config
let config = state.get_config().await?;

// Get cloned session
let session = state.get_session().await?;

// Check if authenticated
let is_auth = state.is_authenticated().await;

// Set API client
state.set_client(client).await;

// Update and persist config
state.update_config(|config| {
    config.api_key = Some("new-key".to_string());
}).await?;

// Unlock/lock session
state.unlock_session("password").await?;
state.lock_session().await?;
```

### StateEvent

Events emitted when state changes.

```rust
pub enum StateEvent {
    ConfigChanged { key: String, value: Value },
    SessionUnlocked,
    SessionLocked,
    ServerConfigChanged { feature: ServerFeature, enabled: bool },
}
```

**Features:**
```rust
pub enum ServerFeature {
    McpServer,
    Subscriptions,
    Alerts,
}
```

**Subscription:**
```rust
// Subscribe to events
let mut rx = state.subscribe();

while let Ok(event) = rx.recv().await {
    match event {
        StateEvent::SessionUnlocked => {
            println!("Session unlocked!");
        }
        StateEvent::ConfigChanged { key, value } => {
            println!("Config {} changed to {}", key, value);
        }
        _ => {}
    }
}
```

### StateInner

The actual state data protected by the RwLock.

```rust
pub struct StateInner {
    pub config: Config,           // Application configuration
    pub session: Session,         // User session
    pub client: Option<IrisClient>, // API client
    pub server_config: ServerConfig, // MCP server config
    pub event_sender: StateEventSender, // Event broadcast channel
}
```

## Error Handling

All state operations return `Result<T, StateError>`:

```rust
pub enum StateError {
    InitializationError(String),
    LockError(String),
    ConfigError(ConfigError),
    SessionError(SessionError),
    ClientError(String),
    AlreadyInitialized,
    NotInitialized,
}
```

Automatic conversions via `From` traits allow using `?`:

```rust
let config = Config::load_default()?; // Converts ConfigError to StateError
let session = Session::new(config)?;   // Converts SessionError to StateError
```

## Thread Safety

The state is designed for concurrent access:

- **`Arc<RwLock<StateInner>>`**: Multiple readers OR single writer
- **`Clone`**: `EdgeState` is cheap to clone (just clones the Arc)
- **`Send + Sync`**: Safe to share across threads
- **Broadcast channels**: Multiple subscribers can receive events

```rust
// Spawn multiple tasks that access state
let state1 = state.clone();
tokio::spawn(async move {
    let config = state1.get_config().await.unwrap();
});

let state2 = state.clone();
tokio::spawn(async move {
    let session = state2.get_session().await.unwrap();
});
```

## Testing

Tests should avoid affecting the global singleton:

```rust
#[test]
fn test_state_error_display() {
    // Test error variants without initializing state
    let err = StateError::NotInitialized;
    assert_eq!(err.to_string(), "State has not been initialized");
}

#[tokio::test]
async fn test_server_config_default() {
    // Test individual components
    let config = ServerConfig::default();
    assert_eq!(config.host, "127.0.0.1");
}
```

**Note**: Tests that use `EdgeState::init()` should run in isolated processes
or clean up after themselves to avoid affecting other tests.

## Future Enhancements

1. **Feature Flags**: Complete integration of `FeatureServerConfig` into `StateInner`
2. **Event Persistence**: Persist config changes to disk and emit events
3. **State Persistence**: Save/restore full state for crash recovery
4. **Metrics**: Add state metrics (lock contention, event queue depth)

## See Also

- [State Refactor Documentation](../../docs/trade.edge/trade.edge.engineering/systems/poseidon/state-refactor.md) - Complete documentation
- [Architecture Overview](../../docs/trade.edge/trade.edge.engineering/systems/poseidon/architecture.md) - System architecture
- [Session Module](../session/mod.rs) - Session management
- [Error Module](../error.rs) - Error types
