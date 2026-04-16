# OpenClaw

Install Edge as an OpenClaw skill for use with any OpenClaw-compatible agent.

## Installation

```bash
claw skill install edge
```

Or add to your `clawhub.yaml`:

```yaml
skills:
  - name: edge
    version: 0.1.0
    config:
      API_KEY: ${EDGE_API_KEY}
```

## Configuration

Set your API key:

```bash
export EDGE_API_KEY=sk-your-key-here
```

## Skill details

| Field | Value |
|-------|-------|
| **Name** | edge |
| **Author** | edgedottrade |
| **Version** | 0.1.0 |
| **License** | MIT |
| **Repository** | [github.com/edgetrade/edge](https://github.com/edgetrade/edge) |
| **ClawHub** | [clawhub.ai/lexomis/edge](https://clawhub.ai/lexomis/edge) |

## Available tools

Same 6 tools and 39 actions as all other integrations. See [Tools](../tools/README.md).

See [Quick Start](../quick-start.md) for full setup instructions.
