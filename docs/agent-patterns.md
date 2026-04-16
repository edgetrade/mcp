# Agent Patterns

Common workflows with the Edge MCP server. All calls use the `{"action": "...", "schema": 1, "data": {...}}` format.

## Pattern 1: Price before order

**Problem**: Placing a limit order requires computing the exact target price. You need current market data first.

**Correct sequence**:

1. `pairs` → `pair_metrics`: fetch current price and 24h range
2. Compute target price (e.g., buy 5% below current)
3. `orders` → `place_limit_order`: submit order with the computed trigger

**Call example**:

```json
// 1. Fetch metrics (access data["24h"].priceUsd for the 24h interval)
{"action": "pair_metrics", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "HWy1jotHpo6U...",
  "interval": "24h"
}}

// 2. Submit a limit buy at the computed price
{"action": "place_limit_order", "schema": 1, "data": {
  "order": {
    "tokenId": {"tokenChainId": "solana", "tokenContractAddress": "EPjFWdd5Au..."},
    "side": "buy",
    "amount": {"type": "native", "value": 100000000},
    "pairAddress": "HWy1jotHpo6U...",
    "wallets": [{"address": "7xKXtg2CW8..."}],
    "txPreset": {"method": "normal", "slippage": 15, "maxBaseGas": 0, "priorityGas": 0, "bribe": 0, "key": "a"},
    "expiration": 3600,
    "triggerTokenPriceUsd": "<computed>",
    "triggerMarketcapUsd": null,
    "entryStrategyId": null,
    "exitStrategyId": null,
    "counterTokenAddress": null
  }
}}
```

`place_limit_order` wraps every field under `order`. `amount.value` is in native-token smallest units (lamports for Solana). For immediate execution use `place_spot_order` instead.

---

## Pattern 2: Token to pair

**Problem**: You have a token address but need pair-scoped data (candles, swaps, metrics).

**Correct sequence**:

1. `tokens` → `token_info_with_pricing`: returns `pair.pairContractAddress` for the canonical pair
2. Use that pair address in subsequent `pairs` calls

**Call example**:

```json
// 1. Resolve token to pair
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "EPjFWdd5Au...",
  "useBestPair": true,
  "pairContractAddress": null
}}
// Save response.pair.pairContractAddress

// 2. Use it in pair_* calls. Note the per-action field naming.
{"action": "pair_metrics", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "<saved>",
  "interval": "24h"
}}

{"action": "pair_ohlcv", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "<saved>",
  "interval": "1hr",
  "countBack": 24
}}

{"action": "pair_swaps", "schema": 1, "data": {
  "chainId": "solana",
  "pairAddress": "<saved>",
  "tokenAddress": null,
  "makerAddress": null,
  "fromBlock": null,
  "toBlock": null,
  "limit": 50
}}
```

Field naming is not uniform across `pairs.*`:

| Action | Chain field | Pair field |
|--------|-------------|------------|
| `pair_metrics` | `pairChainId` | `pairContractAddress` |
| `pair_ohlcv` | `pairChainId` | `pairContractAddress` |
| `pair_info` | `pairChainId` | `pairContractAddress` |
| `pair_swaps` | `chainId` | `pairAddress` |

---

## Pattern 3: Chain ID format

**Problem**: Solana uses a string, EVM chains use numbers, and chain names don't work.

**Correct format**:

- Solana: `"solana"` (string)
- Ethereum: `1` (number)
- Base: `8453` (number)
- Arbitrum: `42161` (number)

**Common mistake**: Using `"base"` or `"ethereum"` will return `NOT_FOUND`. Using a string for an EVM chain may type-error on some endpoints.

**Call example**:

```json
// Correct
{"action": "search_tokens", "data": {"chainId": 8453, "search": "DOGE"}}
{"action": "search_tokens", "data": {"chainId": "solana", "search": "BONK"}}

// Wrong
{"action": "search_tokens", "data": {"chainId": "base"}}     // NOT_FOUND
{"action": "search_tokens", "data": {"chainId": "8453"}}     // Type error on some endpoints
```

---

## Pattern 4: Subscriptions vs polling

**Problem**: You want to react to new events (swaps, fills, holdings changes) without missing any.

**Correct approach**: Edge offers two paths.

- **Push delivery** (configured in the Edge webapp at Settings > Alerts): Edge POSTs to a webhook, writes to a Redis stream, or sends to Telegram. No agent-side polling. Best for production where you control a receiver.
- **Polling** (from inside an agent): call the relevant action on a cadence and diff against the prior result. Good for prototypes and short-running agents that do not want to run a webhook receiver.

| Need | Poll this action |
|---|---|
| Wallet swaps | `wallet.wallet_swaps` |
| Pair swaps | `pairs.pair_swaps` |
| Pair metrics (price/volume/liquidity) | `pairs.pair_metrics` |
| Order status | `orders.list_orders` |
| Holdings change | `wallet.wallet_holdings` |

See [Subscriptions](subscriptions.md) for the push-delivery overview and [Webhooks](webhooks.md) / [Redis stream delivery](redis-streams.md) for receiver-side details.

---

## Pattern 5: Wallet history

**Problem**: Wallet swap history has depth limits depending on which backend you hit.

**Correct usage**:

- `wallet` → `wallet_swaps`: per-swap history (most recent)
- `wallet` → `wallet_history`: time series of PnL (field is `walletDetails`, not `details`)
- `wallet` → `wallet_holdings`: current positions (unwrap with `data.items`)
- `wallet` → `wallet_summary`: has no `totalPnlUsd`. Calculate as `totalSoldUsd + remainingUsd - totalCostUsd`.

**Call example**:

```json
// wallet_holdings takes wallets[] (array, max 5), not a single address
{"action": "wallet_holdings", "schema": 1, "data": {
  "wallets": ["0xUSER..."],
  "filters": {"isInTrade": true, "chainId": 8453},
  "limit": 100,
  "sortByColumn": "pnlUsd",
  "sortDirection": "desc",
  "cursor": null
}}

// wallet_swaps takes makerAddresses[] (array), not address
{"action": "wallet_swaps", "schema": 1, "data": {
  "limit": 100,
  "makerAddresses": ["0xUSER..."],
  "chainId": 8453,
  "tokenContractAddress": null,
  "pairContractAddress": null,
  "previousPairContractAddress": null
}}

// wallet_summary takes wallets[] with a filters object
{"action": "wallet_summary", "schema": 1, "data": {
  "wallets": ["0xUSER..."],
  "filters": {"isInTrade": true, "chainId": 8453}
}}
```

---

## Pattern 6: Screening (Solana only)

**Problem**: `screen_tokens` filters tokens by dozens of onchain metrics, but it only works on Solana.

**Correct usage**:

- `intelligence` → `screen_tokens`: Solana only. `chainId` filter is silently ignored on EVM.
- Arguments pass as an **array** in `data`: `data: [{...filters}]` not `data: {...filters}`.
- Loop results through `token_info_with_pricing` to enrich with live pricing.

**Call example**:

```json
// Screen Solana tokens with mcap > $100k and low sniper %
{"action": "screen_tokens", "schema": 1, "data": [{
  "chainId": "solana",
  "marketCapUsdMin": 100000,
  "sniperHoldingPctMax": 5
}]}
```

---

## Pattern 7: Chain and pair field naming

**Problem**: Pair-scoped actions use different field names than the rest. `pair_swaps` is the odd one out: it uses `chainId` and `pairAddress` while the other three pair actions use `pairChainId` and `pairContractAddress`.

**The rule per action** (verified against the live `tools/list` manifest):

| Action | Chain field | Pair/token field |
|--------|-------------|------------------|
| `pair_metrics` | `pairChainId` | `pairContractAddress` |
| `pair_ohlcv` | `pairChainId` | `pairContractAddress` |
| `pair_info` | `pairChainId` | `pairContractAddress` |
| `pair_swaps` | `chainId` | `pairAddress` (also `tokenAddress`, `makerAddress`) |
| `tokens.*` | `chainId` | `tokenContractAddress` |
| `apply_entry_strategy` / `apply_exit_strategy` | `tokenChainId` | `tokenContractAddress` |
| `place_limit_order` / `place_spot_order` | inside `order.tokenId.tokenChainId` | inside `order.tokenId.tokenContractAddress` + `order.pairAddress` |
| Everything else | `chainId` | n/a |

**Call examples**:

```json
// pair_ohlcv uses pairChainId + pairContractAddress
{"action": "pair_ohlcv", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "HWy1jotHpo6U...",
  "interval": "1hr",
  "countBack": 24
}}

// pair_swaps uses chainId + pairAddress (different from others in pairs)
{"action": "pair_swaps", "schema": 1, "data": {
  "chainId": "solana",
  "pairAddress": "HWy1jotHpo6U...",
  "tokenAddress": null,
  "makerAddress": null,
  "fromBlock": null,
  "toBlock": null,
  "limit": 50
}}

// tokens uses chainId + tokenContractAddress
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "EPjFWdd5Au...",
  "useBestPair": true
}}
```

Solana uses the string `"solana"` in every variant. EVM chains use numbers.

---

## Pattern 8: Determining swap direction

**Problem**: Swap-returning actions (`wallet_swaps`, `pair_swaps`, `search_swaps`) do not include a `side` field. You have to infer the direction from the filled amounts.

**Rule**:

- `tokensBought > "0"` means the wallet **bought** (acquired tokens)
- `tokensSold > "0"` means the wallet **sold** (spent tokens)

Amounts are returned as strings to preserve full decimal precision. Compare as strings or parse to `BigInt` / `Decimal` before numeric comparison.

**Call example**:

```json
// Pull recent swaps. wallet_swaps takes makerAddresses (plural, array).
{"action": "wallet_swaps", "schema": 1, "data": {
  "limit": 50,
  "makerAddresses": ["7xKXtg2CW8..."],
  "chainId": "solana",
  "tokenContractAddress": null,
  "pairContractAddress": null,
  "previousPairContractAddress": null
}}
```

Then in your code:

```javascript
for (const swap of swaps) {
  const side = swap.tokensBought !== "0" ? "buy" : "sell";
  // swap.fromAddress = maker, swap.transactionHash = tx id
}
```

Field names on each swap are `tokensBought`, `tokensSold`, `fromAddress` (not `makerAddress`), and `transactionHash` (not `txHash`).

See the [portfolio tool reference](tools/portfolio.md) for the full response shape.
