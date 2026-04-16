# REST / HTTP access

Edge exposes one service: the **MCP server**. There is no separate plain-REST API. Every action lives behind the MCP protocol (JSON-RPC over stdio, or MCP Streamable HTTP).

## Canonical usage

Install and run the official MCP binary:

```bash
npx -y @edgedottrade/edge --api-key $EDGE_API_KEY
```

Your AI client (Claude Code, Gemini CLI, Cursor, VS Code, Zed, etc.) then talks to it over stdio using standard MCP. See [Frameworks](frameworks/README.md) for per-client setup.

## Can I call Edge with just `curl`?

**Only through a real MCP client.** The Edge backend is behind Cloudflare with bot-signature filtering; arbitrary HTTP clients (plain `curl`, `requests`, `axios` with default headers) get `426 Upgrade Required` or `403 browser_signature_banned`. A compliant MCP Streamable-HTTP client library works, but configuring one by hand is more work than just running the `@edgedottrade/edge` binary.

If you need Edge from a tool that can only make raw HTTP calls (Zapier, n8n, Make, a Custom GPT action), **the supported path is**:

1. Stand up the `@edgedottrade/edge` binary on a small always-on host (VPS, container, lambda).
2. Expose an internal HTTP endpoint of your own on that host that wraps the MCP server.
3. Have the no-code tool call your wrapper endpoint. Your wrapper forwards requests into MCP and returns responses.

## Authentication

The MCP binary reads `--api-key sk-...` at startup or `EDGE_API_KEY` from the environment. Get a key at [edge.trade/settings/api-keys](https://edge.trade/settings/api-keys). See [Authentication](authentication.md) for details.

## Call format (inside MCP)

Every MCP tool call on Edge uses the same envelope in its `arguments`:

```json
{
  "action": "<action_name>",
  "schema": 1,
  "data": { "...action parameters..." }
}
```

Three fields:

| Field | Required | Purpose |
|-------|----------|---------|
| `action` | yes | Action name (see [Tools](tools/README.md) for the full list of 39) |
| `schema` | yes | Always `1` for the current API version |
| `data` | yes | Action parameters (varies per action) |

## Example MCP call over stdio

```bash
npx -y @edgedottrade/edge --api-key sk-your-key-here <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"cli","version":"1.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized"}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"intelligence","arguments":{"action":"search_tokens","schema":1,"data":{"search":"PEPE","chainId":"solana"}}}}
EOF
```

Wait ~8-15 seconds; the response arrives on the line with `"id":2`. Parse `result.content[0].text` as JSON to get the action's response body.

## Errors

Errors are returned inside the MCP response. Each has a `code` field:

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Token not found on chain 8453"
  }
}
```

See [Errors](errors.md) for the full code list and retry guidance.

## See also

- [Tools](tools/README.md): the full list of 39 actions and their parameters
- [Authentication](authentication.md): key creation and management
- [Frameworks](frameworks/README.md): install for Claude Code, Cursor, VS Code, Zed, Gemini CLI, Codex CLI, and more
- [Cookbook](cookbook.md): end-to-end recipes that combine multiple actions
- [Subscriptions](subscriptions.md): alerts, webhooks, Redis stream, Telegram
