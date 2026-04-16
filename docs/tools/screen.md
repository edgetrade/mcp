# screen

Namespace: `intelligence` · Action: `screen_tokens`

Filter Solana tokens by onchain metrics. Solana-only.

{% hint style="warning" %}
`screen_tokens` is **Solana-only**. The `chainId` filter is silently ignored on EVM chains; the action always returns Solana tokens.
{% endhint %}

## Call format

`data` is an **array** containing a single object with `sortColumn`, `limit`, and `filters`:

```json
{"action": "screen_tokens", "schema": 1, "data": [{
  "sortColumn": "creation_timestamp",
  "limit": 20,
  "filters": {
    "isGraduated": true,
    "minLiquidityUsd": 5000,
    "minHolderCount": 100,
    "maxSnipersHoldingPercentage": 20
  }
}]}
```

Critical:

- `data` is `[{...}]`, not `{...}`.
- Filters live inside the nested `filters` object, not at the top level.
- Filter names use the `min*`/`max*` prefix convention (e.g., `minLiquidityUsd`, `maxSnipersHoldingPercentage`).

## Common filter categories

The schema supports 30+ filters. Typical categories:

- **Size**: `minMarketCapUsd` / `maxMarketCapUsd`, `minLiquidityUsd`, `minVolumeUsd24h`
- **Risk**: `maxSnipersHoldingPercentage`, `maxInsidersHoldingPercentage`, `maxBundlersHoldingPercentage`, `maxTop10HoldingPercentage`
- **Age**: `minAgeMinutes`, `maxAgeMinutes`
- **Holders**: `minHolderCount`
- **Graduation**: `isGraduated`
- **Safety flags**: `mintable`, `freezable`, `dexPaid`
- **Socials**: `hasSocials`

The full list is in `intelligence_screen_tokens_request` in the [openapi schema](../../openapi.json) reference.

## Pairing with live pricing

Screen results are a snapshot. Loop each result through `token_info_with_pricing` (see [inspect](./inspect.md)) to enrich with live pair and pricing data before acting.

## Related

- [search](./search.md): name/address-based search
- [Agent Patterns: Screening](../agent-patterns.md#pattern-6-screening-solana-only)
