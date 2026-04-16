# Codex CLI

OpenAI's CLI agent with native MCP support.

## Install

Codex uses TOML, not JSON. Copy to `~/.codex/config.toml` (Windows: `%USERPROFILE%\.codex\config.toml`):

```toml
[mcp_servers.edge]
command = "npx"
args = ["-y", "@edgedottrade/edge", "--api-key", "sk-..."]
startup_timeout_sec = 30
```

Use a recent Codex release. Earlier versions had a bug that ignored `config.toml` MCP entries.

## Verify

```bash
codex mcp list
```

`edge` should appear as running. Or launch `codex` and check the status line at the bottom of the UI.

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
