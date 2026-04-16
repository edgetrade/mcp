# inspect

MCP tool name: `tokens` (this page is named `inspect.md` historically; the actual tool you invoke is `tokens`)

Token-scoped analytics. Paired with the `pairs` tool for pair-scoped data.

## Actions (4)

| Action | Purpose |
|--------|---------|
| `token_info_with_pricing` | Token details + live pricing + canonical pair address |
| `token_top_holders` | Top holders of the token, with sniper/insider flags |
| `token_top_traders` | Highest-volume traders on the token |
| `token_dev_tokens` | All other tokens deployed by this token's creator |

For pair-scoped data (`pair_metrics`, `pair_ohlcv`, `pair_swaps`, `pair_info`) use the `pairs` tool.

## Call format

```json
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "useBestPair": true,
  "pairContractAddress": null
}}
```

Field naming within the `tokens` namespace:

- All four actions use `chainId` (string `"solana"` or numeric EVM ID)
- `token_dev_tokens` uses `deployerAddress` instead of `tokenContractAddress`

Solana uses `"solana"` (string); EVM chains use numbers.

## Resolving a token to its pair

Many pair-scoped calls need a pair address. Call `token_info_with_pricing` first and keep `pair.pairContractAddress` from the response.

```json
// Step 1: resolve token to pair
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "EPjFWdd5Au...",
  "useBestPair": true
}}
// Save response.pair.pairContractAddress

// Step 2: use it in the pairs tool (note pairChainId, pairContractAddress)
{"action": "pair_ohlcv", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "<saved>",
  "interval": "1hr",
  "countBack": 24
}}
```

## Related

- [Agent Patterns: Token to pair](../agent-patterns.md#pattern-2-token-to-pair)
- [Concepts: Token](../concepts.md#token)
