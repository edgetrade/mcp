# Gemini CLI

Google's CLI agent with native MCP support.

## Install

Copy to `~/.gemini/settings.json` (user scope) or `.gemini/settings.json` in your project:

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

Restart the CLI after editing by hand.

## Verify

Launch `gemini`, then in the session run:

```
/mcp list
```

`edge` should appear as connected.

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
