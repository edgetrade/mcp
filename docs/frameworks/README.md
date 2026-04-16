# Frameworks

Edge's MCP server works with any AI agent or editor that speaks the Model Context Protocol. Pick your client below for setup instructions. All of them run the same `@edgedottrade/edge` package under the hood, so the capabilities are identical.

## CLIs

| Client | Guide |
|--------|-------|
| Claude Code | [claude-code.md](claude-code.md) |
| Gemini CLI | [gemini-cli.md](gemini-cli.md) |
| Codex CLI | [codex-cli.md](codex-cli.md) |
| Goose | [goose.md](goose.md) |

## Editors and IDEs

| Client | Guide |
|--------|-------|
| Cursor | [cursor.md](cursor.md) |
| VS Code (GitHub Copilot) | [vs-code.md](vs-code.md) |
| Zed | [zed.md](zed.md) |
| Cline (VS Code extension) | [cline.md](cline.md) |
| Continue | [continue.md](continue.md) |
| Windsurf | [windsurf.md](windsurf.md) |

## Agent SDKs

| Framework | Guide |
|-----------|-------|
| OpenAI Agents SDK | [openai-agents-sdk.md](openai-agents-sdk.md) |
| Google ADK | [google-adk.md](google-adk.md) |
| LangChain | [langchain.md](langchain.md) |
| CrewAI | [crewai.md](crewai.md) |
| Mastra | [mastra.md](mastra.md) |
| Vercel AI SDK | [vercel-ai-sdk.md](vercel-ai-sdk.md) |

## Don't see your client?

If your AI tool supports MCP, it will work with Edge. Use one of the configs above as a template and point it at `npx @edgedottrade/edge --api-key $EDGE_API_KEY`.

If your tool cannot speak MCP at all, see [REST / HTTP access](../rest-api.md) for the (constrained) ways to call Edge over plain HTTP.

## See also

- [Quick Start](../quick-start.md): install Edge MCP once, use it from any client
- [Plugins](../plugins/): Claude Desktop and OpenClaw skill bundles
- [Tools](../tools/): the 6 MCP tools and their 39 actions
