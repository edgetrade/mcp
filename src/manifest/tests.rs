//! Comprehensive tests for the manifest module
use std::path::PathBuf;

use super::types::McpManifest;
use super::*;

#[test]
fn test_sha256_hashing() {
    let data = b"test data";
    let hash = sha256(data);
    // SHA256 hash is always 64 hex characters
    assert_eq!(hash.len(), 64);
    // Same input produces same hash
    assert_eq!(hash, sha256(b"test data"));
    // Different input produces different hash
    assert_ne!(hash, sha256(b"different data"));
}

fn create_test_manifest() -> McpManifest {
    McpManifest {
        tools: vec![types::ToolDef {
            name: "agent".to_string(),
            description: "Agent tool".to_string(),
            input_schema: serde_json::json!({"type": "object"}),
            kind: "namespace".to_string(),
            actions: vec![],
        }],
        resources: vec![],
        prompts: vec![],
        skills: vec![],
    }
}

fn create_test_manifest_with_actions() -> McpManifest {
    McpManifest {
        tools: vec![types::ToolDef {
            name: "agent".to_string(),
            description: "Agent tool".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["existing"]
                    }
                }
            }),
            kind: "namespace".to_string(),
            actions: vec![types::ActionDef {
                name: "existing".to_string(),
                description: "Existing action".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
                procedure: "agent.existing".to_string(),
                kind: "local".to_string(),
            }],
        }],
        resources: vec![],
        prompts: vec![],
        skills: vec![],
    }
}

mod types_tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let manifest = create_test_manifest();
        let json = serde_json::to_string(&manifest).unwrap();
        let parsed: McpManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.tools.len(), parsed.tools.len());
        assert_eq!(manifest.tools[0].name, parsed.tools[0].name);
    }

    #[test]
    fn test_manifest_deserialization_from_server_format() {
        let server_json = r#"{
                "tools": [{
                    "name": "market",
                    "description": "Market data",
                    "inputSchema": {"type": "object"},
                    "kind": "namespace",
                    "actions": [{
                        "name": "searchTokens",
                        "description": "Search tokens",
                        "inputSchema": {"type": "object"},
                        "procedure": "market.searchTokens",
                        "kind": "query"
                    }]
                }],
                "resources": [],
                "prompts": [],
                "skills": []
            }"#;

        let manifest: McpManifest = serde_json::from_str(server_json).unwrap();
        assert_eq!(manifest.tools.len(), 1);
        assert_eq!(manifest.tools[0].name, "market");
        assert_eq!(manifest.tools[0].actions.len(), 1);
        assert_eq!(manifest.tools[0].actions[0].name, "searchTokens");
    }

    #[test]
    fn test_tool_def_roundtrip() {
        let tool = types::ToolDef {
            name: "test".to_string(),
            description: "Test tool".to_string(),
            input_schema: serde_json::json!({"type": "object"}),
            kind: "namespace".to_string(),
            actions: vec![],
        };

        let json = serde_json::to_string(&tool).unwrap();
        let parsed: types::ToolDef = serde_json::from_str(&json).unwrap();
        assert_eq!(tool.name, parsed.name);
        assert_eq!(tool.description, parsed.description);
    }

    #[test]
    fn test_action_def_roundtrip() {
        let action = types::ActionDef {
            name: "test_action".to_string(),
            description: "Test action".to_string(),
            input_schema: serde_json::json!({"type": "object"}),
            procedure: "tool.test_action".to_string(),
            kind: "query".to_string(),
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: types::ActionDef = serde_json::from_str(&json).unwrap();
        assert_eq!(action.name, parsed.name);
        assert_eq!(action.procedure, parsed.procedure);
        assert_eq!(action.kind, parsed.kind);
    }
}

mod cache_tests {
    use super::*;

    #[test]
    fn test_cache_roundtrip() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let manifest = create_test_manifest();

        cache::save_manifest(temp_dir.path(), &manifest).expect("Failed to save manifest");
        let (loaded, timestamp) = cache::load_cached_manifest(temp_dir.path()).unwrap();

        assert_eq!(manifest.tools.len(), loaded.tools.len());
        assert_eq!(manifest.tools[0].name, loaded.tools[0].name);
        assert!(cache::is_fresh(timestamp, 10));
    }

    #[test]
    fn test_cache_expired() {
        let old_time = chrono::Utc::now() - chrono::Duration::minutes(15);
        assert!(!cache::is_fresh(old_time, 10));
    }

    #[test]
    fn test_cache_fresh_within_ttl() {
        let recent_time = chrono::Utc::now() - chrono::Duration::minutes(5);
        assert!(cache::is_fresh(recent_time, 10));
    }

    #[test]
    fn test_cache_at_exact_boundary() {
        let exactly_ten_minutes_ago = chrono::Utc::now() - chrono::Duration::minutes(10);
        // At exactly the boundary, it should NOT be fresh (<, not <=)
        assert!(!cache::is_fresh(exactly_ten_minutes_ago, 10));
    }

    #[test]
    fn test_load_nonexistent_cache() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let result = cache::load_cached_manifest(temp_dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_load_with_complex_manifest() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let manifest = McpManifest {
            tools: vec![types::ToolDef {
                name: "market".to_string(),
                description: "Market data access".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "action": {"type": "string"}
                    }
                }),
                kind: "namespace".to_string(),
                actions: vec![types::ActionDef {
                    name: "searchTokens".to_string(),
                    description: "Search for tokens".to_string(),
                    input_schema: serde_json::json!({"type": "object"}),
                    procedure: "market.searchTokens".to_string(),
                    kind: "query".to_string(),
                }],
            }],
            resources: vec![types::ResourceDef {
                uri: "edge://test".to_string(),
                name: "Test Resource".to_string(),
                description: "A test resource".to_string(),
                mime_type: "application/json".to_string(),
                content: serde_json::json!({"test": true}),
            }],
            prompts: vec![types::PromptDef {
                name: "test_prompt".to_string(),
                description: "A test prompt".to_string(),
                arguments: vec![types::PromptArgument {
                    name: "input".to_string(),
                    description: "Input parameter".to_string(),
                    required: true,
                }],
                messages: vec![],
            }],
            skills: vec![types::SkillDef {
                name: "test_skill".to_string(),
                description: "A test skill".to_string(),
                content: "Test skill content".to_string(),
            }],
        };

        cache::save_manifest(temp_dir.path(), &manifest).expect("Failed to save manifest");

        let loaded = cache::load_cached_manifest(temp_dir.path());
        assert!(loaded.is_some());

        let (loaded_manifest, fetched_at) = loaded.unwrap();
        assert_eq!(loaded_manifest.tools.len(), 1);
        assert_eq!(loaded_manifest.tools[0].name, "market");
        assert_eq!(loaded_manifest.resources.len(), 1);
        assert_eq!(loaded_manifest.prompts.len(), 1);
        assert_eq!(loaded_manifest.skills.len(), 1);

        // Verify timestamp is recent
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(fetched_at);
        assert!(elapsed.num_seconds() < 5);
    }
}

mod inject_tests {
    use super::*;

    #[test]
    fn test_inject_local_actions_adds_ping() {
        let mut manifest = create_test_manifest();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        assert!(agent_tool.actions.iter().any(|a| a.name == "ping"));
    }

    #[test]
    fn test_inject_local_actions_adds_ping_subscription() {
        let mut manifest = create_test_manifest();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        assert!(
            agent_tool
                .actions
                .iter()
                .any(|a| a.name == "ping_subscription")
        );
    }

    #[test]
    fn test_inject_local_actions_adds_list_alerts() {
        let mut manifest = create_test_manifest();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        assert!(agent_tool.actions.iter().any(|a| a.name == "list_alerts"));
    }

    #[test]
    fn test_inject_local_actions_adds_register_alert() {
        let mut manifest = create_test_manifest();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        let register_alert = agent_tool
            .actions
            .iter()
            .find(|a| a.name == "register_alert");
        assert!(register_alert.is_some());

        let action = register_alert.unwrap();
        assert_eq!(action.procedure, "agent.register_alert");
        assert_eq!(action.kind, "local");
    }

    #[test]
    fn test_inject_local_actions_adds_unregister_alert() {
        let mut manifest = create_test_manifest();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        assert!(
            agent_tool
                .actions
                .iter()
                .any(|a| a.name == "unregister_alert")
        );
    }

    #[test]
    fn test_inject_does_not_duplicate() {
        let mut manifest = create_test_manifest();
        inject::inject_local_agent_actions(&mut manifest);
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        let ping_count = agent_tool
            .actions
            .iter()
            .filter(|a| a.name == "ping")
            .count();
        assert_eq!(ping_count, 1);
    }

    #[test]
    fn test_inject_preserves_existing_actions() {
        let mut manifest = create_test_manifest_with_actions();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        assert!(agent_tool.actions.iter().any(|a| a.name == "existing"));
        assert!(agent_tool.actions.iter().any(|a| a.name == "ping"));
    }

    #[test]
    fn test_inject_adds_to_enum_array() {
        let mut manifest = create_test_manifest_with_actions();
        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        let enum_arr = agent_tool
            .input_schema
            .pointer("/properties/action/enum")
            .and_then(|v| v.as_array())
            .expect("Enum array should exist");

        assert!(enum_arr.iter().any(|v| v.as_str() == Some("existing")));
        assert!(enum_arr.iter().any(|v| v.as_str() == Some("ping")));
        assert!(
            enum_arr
                .iter()
                .any(|v| v.as_str() == Some("register_alert"))
        );
    }

    #[test]
    fn test_inject_extends_description() {
        let mut manifest = create_test_manifest();
        let original_desc = manifest.tools[0].description.clone();

        inject::inject_local_agent_actions(&mut manifest);

        let agent_tool = manifest.tools.iter().find(|t| t.name == "agent").unwrap();
        assert!(agent_tool.description.contains("ping:"));
        assert!(agent_tool.description.contains("register_alert:"));
        assert!(agent_tool.description.starts_with(&original_desc));
    }

    #[test]
    fn test_inject_local_resources_adds_alert_delivery() {
        let mut manifest = create_test_manifest();
        inject::inject_local_resources(&mut manifest);

        let resource = manifest
            .resources
            .iter()
            .find(|r| r.uri == "edge://alert-delivery");
        assert!(resource.is_some());

        let res = resource.unwrap();
        assert_eq!(res.name, "Alert Delivery Methods");
        assert_eq!(res.mime_type, "application/json");
    }

    #[test]
    fn test_inject_resources_does_not_duplicate() {
        let mut manifest = create_test_manifest();
        inject::inject_local_resources(&mut manifest);
        inject::inject_local_resources(&mut manifest);

        let count = manifest
            .resources
            .iter()
            .filter(|r| r.uri == "edge://alert-delivery")
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_inject_no_op_when_no_agent_tool() {
        let mut manifest = McpManifest {
            tools: vec![types::ToolDef {
                name: "market".to_string(),
                description: "Market tool".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
                kind: "namespace".to_string(),
                actions: vec![],
            }],
            resources: vec![],
            prompts: vec![],
            skills: vec![],
        };

        inject::inject_local_agent_actions(&mut manifest);
        // Should not panic, should be no-op
        assert_eq!(manifest.tools[0].actions.len(), 0);
    }
}

mod manager_tests {
    use super::*;

    #[test]
    fn test_manifest_path_construction() {
        let config_dir = std::path::Path::new("/tmp/test-config");
        let path = cache::manifest_path(config_dir);
        assert_eq!(path, PathBuf::from("/tmp/test-config/manifest.json"));
    }
}
