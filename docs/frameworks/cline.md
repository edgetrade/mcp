# Cline

Cline is a VS Code extension with MCP support.

## Install

Open the Cline sidebar, click the **MCP Servers** icon, then **Configure**. Or edit the config directly.

Config path:

- macOS: `~/Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json`
- Windows: `%APPDATA%\Code\User\globalStorage\saoudrizwan.claude-dev\settings\cline_mcp_settings.json`
- Linux: `~/.config/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json`

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "sk-..."],
      "disabled": false,
      "alwaysAllow": []
    }
  }
}
```

Use the `Code - Insiders` folder instead of `Code` if you're on VS Code Insiders.

## Verify

Open the Cline sidebar, click the **MCP Servers** icon, and confirm `edge` shows a green status dot.

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
