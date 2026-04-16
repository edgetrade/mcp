# Claude Code

Anthropic's CLI agent. Installs the Edge MCP server in one command.

## Install

Register Edge at user scope (works from any directory):

```bash
claude mcp add edge --scope user -- npx -y @edgedottrade/edge --api-key sk-...
```

Or edit the config file directly at `~/.claude.json`:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "sk-..."]
    }
  }
}
```

Project-scoped installs use `.mcp.json` at the repo root.

## Verify

```bash
claude mcp list
```

You should see `edge` with a green status. In a new Claude Code session, ask: "Search for PEPE on Solana" and confirm the tool call runs.

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
