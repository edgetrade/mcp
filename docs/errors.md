# Errors

All error codes returned by the Edge Agent API. Errors are JSON objects with a `code` field and a human-readable `message`.

## Error response format

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Token not found on chain 8453"
  }
}
```

## Authentication errors

| Code | HTTP Status | Meaning | Fix |
|------|-----------|---------|-----|
| `UNAUTHORIZED` | 401 | API key is missing or invalid | Check your API key. Regenerate if needed at edge.trade/settings/api-keys |
| `FORBIDDEN` | 403 | Valid key but insufficient permissions | Register the wallet in Edge before accessing wallet-scoped data |

## Resource errors

| Code | HTTP Status | Meaning | Fix |
|------|-----------|---------|-----|
| `NOT_FOUND` | 404 | Token, pair, order, or wallet does not exist | Verify the address and chain ID are correct. Use `search_tokens` to find valid addresses. |
| `INVALID_ADDRESS` | 400 | Address is malformed or not checksummed | Use checksummed EVM addresses (0x...) or valid base58 for Solana. Use `search_tokens` to get the canonical address. |
| `INVALID_CHAIN` | 400 | Chain ID is not supported or incorrectly formatted | Use string format: `"1"`, `"8453"`, `"42161"`, `"solana"`. Do not use names like `"base"` or `"ethereum"`. |
| `PAIR_NOT_FOUND` | 404 | Pair address does not exist on this chain | Use `token_info_with_pricing` with `useBestPair: true` to find the canonical pair address. |

## Trading errors

| Code | HTTP Status | Meaning | Fix |
|------|-----------|---------|-----|
| `INSUFFICIENT_BALANCE` | 400 | Wallet lacks enough tokens for this trade | Check holdings with `wallet_holdings` or `native_balances` before trading. |
| `SLIPPAGE_EXCEEDED` | 400 | Price moved beyond your slippage tolerance | Increase slippage in your order's `txPreset`, or retry. Check `pair_metrics` for current price. |
| `ORDER_EXPIRED` | 400 | Limit order timed out before filling | Set a longer `expiration`, or check if the target price is realistic using `pair_metrics`. |
| `NOT_IMPLEMENTED` | 501 | This trading feature is not yet live | Some execution features are in development. Check the Edge changelog for status updates. |

## Rate limiting

| Code | HTTP Status | Meaning | Fix |
|------|-----------|---------|-----|
| `RATE_LIMIT` | 429 | Too many requests from this API key | Reduce request frequency. Cache results for 1-5 minutes between calls. Batch queries where possible. |

### Rate limit guidelines

- **Search/screen**: Cache results for 5-10 minutes
- **Pair metrics**: Cache for 1-5 minutes (prices change frequently)
- **Portfolio**: Cache for 1-5 minutes
- **Token data**: Cache for 10+ minutes (changes slowly)
- **Orders**: No caching. Always fetch fresh.

## Subscription errors

These surface in the Edge webapp alerts dashboard for alerts you configured under Settings > Alerts. They are not returned through MCP.

| Code | Meaning | Fix |
|------|---------|-----|
| `WEBHOOK_DELIVERY_FAILED` | Webhook endpoint returned 4xx/5xx or was unreachable | Verify the URL is correct and the endpoint is running. Check for firewall or SSL issues. |
| `INVALID_FILTER` | Filter shape does not match the alert's `inputSchema` | Re-check the per-alert input schema in the `edge://alerts` MCP resource. |

## Data errors

| Code | HTTP Status | Meaning | Fix |
|------|-----------|---------|-----|
| `null` values | 200 | Data not available or token is too new | Some fields (like `volume24h`, `mcap`, `socialScore`) return `null` for very new tokens. Try again after 10+ minutes. |
| Empty array `[]` | 200 | No results match your query | Broaden your search terms or loosen filter criteria. Verify the chain has activity. |

## Retry guidance

| Error type | Should retry? | Strategy |
|-----------|--------------|----------|
| `UNAUTHORIZED` / `FORBIDDEN` | No | Fix credentials or permissions |
| `NOT_FOUND` / `INVALID_ADDRESS` | No | Fix the input data |
| `RATE_LIMIT` | Yes | Wait 5-10 seconds, then retry with exponential backoff |
| `SLIPPAGE_EXCEEDED` | Maybe | Retry with higher slippage or wait for price to stabilize |
| `NOT_IMPLEMENTED` | No | Feature not available yet |
| Network timeouts | Yes | Retry after 2-5 seconds |

## See also

- [Authentication](authentication.md): API key setup
- [Concepts](concepts.md): Chain ID and address formats
- [Tools](tools/): Per-tool error tables in the "Mistakes" section of each tool page
