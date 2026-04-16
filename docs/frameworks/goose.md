# Goose

Block's open-source CLI agent with native MCP support.

## Install

The easiest path is the interactive wizard:

```bash
goose configure
```

Choose **Add Extension** then **Command-line Extension**. Name it `edge`, command `npx`, args `-y @edgedottrade/edge --api-key sk-...`.

Or edit `~/.config/goose/config.yaml` (Windows: `%APPDATA%\Block\goose\config\config.yaml`) directly:

```yaml
extensions:
  edge:
    type: stdio
    enabled: true
    cmd: npx
    args: ["-y", "@edgedottrade/edge", "--api-key", "sk-..."]
    timeout: 300
    envs: {}
```

Goose uses `cmd` (not `command`) and `envs` (not `env`). For production, put the API key in an environment variable and reference it through `env_keys` rather than hardcoding it.

## Verify

```bash
goose info -v
```

`edge` should appear under enabled extensions along with the config path Goose is using.

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
