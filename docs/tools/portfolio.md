# portfolio

MCP tool name: `wallet` (this page is named `portfolio.md` historically; the actual tool you invoke is `wallet`)

Wallet holdings, PnL, trade history, and native balances.

## Actions

| Action | Purpose |
|--------|---------|
| `wallet_holdings` | Current positions. Unwrap with `data.items`. |
| `wallet_summary` | Aggregate PnL overview. Has no `totalPnlUsd`; calculate: `totalSoldUsd + remainingUsd - totalCostUsd`. |
| `wallet_swaps` | Per-swap history. |
| `wallet_history` | Time series of PnL. Response field is `walletDetails`, not `details`. |
| `native_balances` | Native gas token balances across chains. Nested as `balances.solana.ADDR` = lamports string; divide by 1e9 for SOL. |

For token-scoped top traders (the highest-volume traders on a specific token), use the `tokens` tool's `token_top_traders` action: that's a token analytic, not a wallet one.

## Call format

```json
{"action": "wallet_holdings", "schema": 1, "data": {
  "wallets": ["7xKXtg2CW8..."],
  "filters": {
    "isInTrade": true,
    "chainId": "solana"
  },
  "limit": 100,
  "sortByColumn": "pnlUsd",
  "sortDirection": "desc",
  "cursor": null
}}
```

- `wallets` is an array. Max **5 wallets per call**.
- `chainId` uses numbers for EVM, `"solana"` for Solana.
- `sortByColumn` options include `pnlUsd`, `balanceUsd`, `mostRecentTx`.

## Common gotchas

- `wallet_holdings` wraps results in `{items: [], next}`. Unwrap with `data.items`.
- Max 5 wallets per call for `wallet_holdings` and `wallet_summary`.
- `wallet_summary` has no `totalPnlUsd` or `winRate` field. Compute manually: `totalSoldUsd + remainingUsd - totalCostUsd`.
- `wallet_history` uses `walletDetails` (not `details`). Silent empty return if you use the wrong field.
- `wallet_swaps` uses `makerAddresses` (plural array), not `address`.
- `native_balances` is deeply nested: `balances.<chain>.<address>` is a lamports/wei string. Divide by 1e9 for SOL.
- No `side` field in swap rows. Check `tokensBought > "0"` for buy, `tokensSold > "0"` for sell.
- Swap rows use `fromAddress` (maker) and `transactionHash` (tx id).

## Related

- [Agent Patterns → Wallet history](../agent-patterns.md#pattern-5-wallet-history)
- [trade](./trade.md): check holdings before selling
