# VS Code

Use Edge with GitHub Copilot's agent mode in VS Code 1.102 or later.

## Install

Copy to `.vscode/mcp.json` in your workspace. Note the root key is `servers`, not `mcpServers`:

```json
{
  "servers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "sk-..."]
    }
  }
}
```

For a user-level install, open the Command Palette and run **MCP: Open User Configuration**, then add the same `servers` block.

## Verify

Command Palette to **MCP: List Servers**. `edge` should appear as connected. Then open Copilot Chat, switch to agent mode, and confirm the Edge tools appear in the tools list.

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
