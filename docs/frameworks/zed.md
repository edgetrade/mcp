# Zed

Zed's Agent Panel calls MCP servers "context servers".

## Install

Copy to `~/.config/zed/settings.json` (macOS and Linux) or `%APPDATA%\Zed\settings.json` (Windows):

```json
{
  "context_servers": {
    "edge": {
      "source": "custom",
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "sk-..."],
      "env": {}
    }
  }
}
```

The `"source": "custom"` field is required for servers you configure yourself. Servers installed from the Zed extension registry use a different shape and do not need this field.

## Verify

Open the Agent Panel settings. The `edge` server indicator should be green with the text "Server is active".

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
