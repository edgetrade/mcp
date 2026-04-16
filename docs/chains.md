# Chains

Technical reference for chain IDs, native tokens, and chain-specific behavior in the Edge Agent API.

## Chain ID reference

Solana uses a **string** (`"solana"`). All EVM chains use **numbers** (unquoted).

| Chain | Chain ID | Native Token | Type |
|-------|----------|-------------|------|
| Solana | `"solana"` | SOL | SVM |
| Ethereum | `1` | ETH | EVM |
| Base | `8453` | ETH | EVM |
| Arbitrum | `42161` | ETH | EVM |
| BSC | `56` | BNB | EVM |
| Optimism | `10` | ETH | EVM |
| Avalanche | `43114` | AVAX | EVM |
| Blast | `81457` | ETH | EVM |

{% hint style="warning" %}
Solana is the only chain that uses a string. Pass `"solana"` not `solana`. All EVM chains use numeric IDs: pass `8453` not `"8453"`.
{% endhint %}

## Chain-specific behavior

### Solana

- **Chain ID**: `"solana"` (string)
- **Address format**: Base58-encoded (32–44 characters, no `0x` prefix)
- **Token screening**: `screen_tokens` is **Solana-only**. The `chainId` filter is silently ignored on EVM chains.
- **Transaction speed**: Sub-second confirmation
- **Gas costs**: Fractions of a cent
- **MEV protection**: Via Jito bundles

### EVM chains

- **Chain ID**: Numbers (e.g. `1`, `8453`, `42161`)
- **Address format**: Checksummed hex (42 characters, starts with `0x`)
- **MEV protection**: Via Flashbots (`txPreset.method: "flashbot"`)
- **Gas costs**: Vary significantly. Ethereum mainnet is most expensive; L2s (Base, Arbitrum, Optimism) are much cheaper.

## Tool availability by chain

| Tool | Solana | EVM chains |
|------|--------|-----------|
| `search_tokens` | Yes | Yes |
| `screen_tokens` | **Yes** | No (silently returns empty) |
| `search_swaps` | Yes | Yes |
| `token_info_with_pricing` | Yes | Yes |
| `token_top_holders` | Yes | Yes |
| `token_top_traders` | Yes | Yes |
| `token_dev_tokens` | Yes | Yes |
| `pair_metrics` | Yes | Yes |
| `pair_ohlcv` | Yes | Yes |
| `pair_swaps` | Yes | Yes |
| `pair_info` | Yes | Yes |
| `wallet_holdings` | Yes | Yes |
| `wallet_summary` | Yes | Yes |
| `wallet_swaps` | Yes | Yes |
| `wallet_history` | Yes | Yes |
| `native_balances` | Yes | Yes |
| `place_limit_order` / `place_spot_order` | Yes | Yes (Solana, ETH, Base, Arbitrum verified) |
| `list_orders` / `order` / `cancel_order` / `cancel_all_orders` | Yes | Yes |
| Strategy actions (entry / exit, 10 total) | Yes | Yes |

## Call examples

MCP calls use `{"action": "ACTION_NAME", "schema": 1, "data": {...}}`. The tool name equals the namespace.

```json
// Solana: string chain ID
{"action": "search_tokens", "schema": 1, "data": {"chainId": "solana", "search": "BONK"}}

// Base: number chain ID
{"action": "search_tokens", "schema": 1, "data": {"chainId": 8453, "search": "USDC"}}

// Ethereum mainnet: number chain ID
{"action": "search_tokens", "schema": 1, "data": {"chainId": 1, "search": "USDC"}}
```

### Common mistakes

```json
// WRONG: chain name instead of ID
{"action": "search_tokens", "data": {"chainId": "base"}}       // NOT_FOUND
{"action": "search_tokens", "data": {"chainId": "ethereum"}}   // NOT_FOUND

// WRONG: string for EVM chain
{"action": "search_tokens", "data": {"chainId": "8453"}}       // Type error on some endpoints

// CORRECT
{"action": "search_tokens", "data": {"chainId": 8453}}         // EVM uses number
{"action": "search_tokens", "data": {"chainId": "solana"}}     // Solana uses string
```

## See also

- [Supported Chains and DEXes](../supported-chains-and-dexes.md): Full DEX list per chain
- [Concepts](concepts.md): Chain ID format and address types
- [Agent Patterns](agent-patterns.md): Common patterns
