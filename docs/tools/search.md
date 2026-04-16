# search

MCP tool name: `intelligence` (this page is named `search.md` historically; the actual tool you invoke is `intelligence`)

Token and swap discovery across 8 chains.

## Actions

| Action | Purpose |
|--------|---------|
| `search_tokens` | Find tokens by name, symbol, or contract address |
| `screen_tokens` | Filter tokens by onchain metrics (Solana only): see [screen](./screen.md) |
| `search_swaps` | Look up recent swap transactions |

## Call format

```json
{"action": "search_tokens", "schema": 1, "data": {
  "search": "PEPE",
  "chainId": "solana",
  "hasGraduated": true,
  "pairTypes": null,
  "weightings": null
}}
```

- `chainId` is `"solana"` (string) or a number for EVM chains
- The parameter is `search`, not `query`
- Optional: `hasGraduated`, `pairTypes`, `weightings`

## Resolving by contract address

```json
{"action": "search_tokens", "schema": 1, "data": {
  "search": "So11111111111111111111111111111111111111112",
  "chainId": "solana"
}}
```

## Looking up a specific swap

```json
{"action": "search_swaps", "schema": 1, "data": {
  "chainId": "solana",
  "txHash": "5UfDuX7hXbAjRMn2z9M3G2bqT7KLhRYmkCFRJubXpam3RqKMD1gzFbGeackCkAFVJ8oCBVxbJFuhEqaVxKsFdNFr",
  "blockTimestamp": 1710000000
}}
```

Both `txHash` and `blockTimestamp` are required to look up a swap.

## Related

- [screen](./screen.md): onchain-metric filtering (Solana only)
- [Agent Patterns → Chain ID format](../agent-patterns.md#pattern-3-chain-id-format)
- [Concepts → Address formats](../concepts.md#address-formats)
