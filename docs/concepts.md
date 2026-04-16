# Concepts

Core terminology and data types used across the Edge Agent API.

## Token

A fungible asset deployed on a blockchain. Identified by its **contract address** and **chain ID**.

```
Solana:    base58 string     e.g. "So11111111111111111111111111111111111111112"
EVM:       checksummed hex   e.g. "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
```

Key fields returned by the API:
- `address`: contract address
- `symbol`: ticker symbol (e.g., "USDC")
- `name`: full token name
- `decimals`: decimal places for the token (6 for USDC, 18 for most EVM tokens, 9 for most Solana tokens)
- `chainId`: which blockchain the token lives on

## Pair

A trading pair on a DEX, consisting of two tokens in a liquidity pool. Identified by a **pair address** and **chain ID**.

Pairs are identified by a pair contract address and chain ID. Use `token_info_with_pricing` to find the pair address for any token.

Pairs are the unit of trading. When you buy a token, you're swapping through a pair. Most API calls for price data, candles, and swaps require a `pairAddress`, not a token address.

**Token to Pair resolution:** Use `token_info_with_pricing` with `useBestPair: true` to get the canonical pair address. See [Pattern 2: Token to pair](agent-patterns.md#pattern-2-token-to-pair).

## Order

A limit order to buy or sell a token at a target price. Orders have a lifecycle:

| Status | Meaning |
|--------|---------|
| `Working` | Order is active, waiting for the target price |
| `Placing` | Order is being submitted to the blockchain |
| `Filled` | Order executed successfully |
| `Cancelled` | Order was cancelled by the user |
| `Expired` | Order timed out before the target price was reached |
| `Rejected` | Order was rejected (insufficient balance, invalid parameters) |

Orders are placed via the [trade](tools/trade.md) tool. Use `place_limit_order` for trigger-based orders (set `triggerTokenPriceUsd` or `triggerMarketcapUsd`) or `place_spot_order` for immediate execution. Required inputs in either case: the token (`tokenId`), side (`buy` / `sell`), amount (`{type, value}` object), a pair to route through, a wallet registered to your API key, and a `txPreset` (slippage, fees). `place_spot_order` additionally requires an encrypted `envelope` constructed by your client.

## Strategy

A reusable trading configuration that fires automatic buys or sells at predefined percentage moves.

### Entry strategy

Defines **when and how much to buy**. Contains one or more steps. Each step fires at most once, when the price has dropped by `percentToTrigger` percent.

```json
{
  "name": "DCA Buy",
  "chainId": "solana",
  "steps": [
    {"buyAmountNativeToken": 50000000,  "percentToTrigger": 5},
    {"buyAmountNativeToken": 100000000, "percentToTrigger": 10}
  ]
}
```

- `buyAmountNativeToken` is an integer in lamports (Solana) or wei (EVM). `50000000` lamports = `0.05 SOL`.
- `percentToTrigger` is positive and represents a drop percentage. `5` means "buy when price is 5% lower".
- Entry strategies **require** `chainId`.

### Exit strategy

Defines **when and how much to sell**. Each step has five fields; unused fields are `null`.

```json
{
  "name": "TP + SL",
  "steps": [
    {
      "tpPercentToTrigger": 100,
      "tpPercentOfBagToSell": 50,
      "tpPercentOfCostToSell": null,
      "slPercentToTrigger": null,
      "slPercentToSell": null
    },
    {
      "tpPercentToTrigger": null,
      "tpPercentOfBagToSell": null,
      "tpPercentOfCostToSell": null,
      "slPercentToTrigger": 30,
      "slPercentToSell": 100
    }
  ]
}
```

- `tpPercentToTrigger` is the take-profit trigger in percent up.
- `slPercentToTrigger` is the stop-loss trigger in percent down (positive value).
- Choose `tpPercentOfBagToSell` (percent of current holdings) or `tpPercentOfCostToSell` (percent of original cost basis).
- Exit strategies do **not** take a `chainId`.

This strategy sells 50% of the bag when profit hits 100%, and sells everything if price drops 30% from entry.

## Chain ID

Solana uses the string `"solana"`. All EVM chains use numbers.

| Chain | Chain ID | Native Token |
|-------|----------|-------------|
| Solana | `"solana"` | SOL |
| Ethereum | `1` | ETH |
| Base | `8453` | ETH |
| Arbitrum | `42161` | ETH |
| BSC | `56` | BNB |
| Optimism | `10` | ETH |
| Avalanche | `43114` | AVAX |
| Blast | `81457` | ETH |

{% hint style="warning" %}
Solana uses a string. EVM chains use numbers. Using chain names like `"base"` or `"ethereum"` will return `NOT_FOUND`.
{% endhint %}

See [Chains](chains.md) for the full reference.

## Address formats

### EVM (Ethereum, Base, Arbitrum, BSC, Optimism, Avalanche, Blast)

- 42-character hex string starting with `0x`
- Checksummed (mixed-case) format recommended
- Example: `0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48`

### Solana (SVM)

- Base58-encoded string, typically 32-44 characters
- No `0x` prefix
- Example: `So11111111111111111111111111111111111111112`

{% hint style="info" %}
The `search` tool handles both formats automatically. Pass any address and the API will match it to the correct chain.
{% endhint %}

## Wallet

A blockchain address controlled by the user. The Edge API requires wallets to be registered before use:

1. Generate or import a wallet in Edge
2. The wallet is associated with your API key
3. Use the wallet address in `trade` (place_limit_order / place_spot_order) and `portfolio` calls

Multiple wallets per API key are supported.

## See also

- [Chains](chains.md): Full chain ID reference
- [Authentication](authentication.md): API key setup
- [Agent Patterns](agent-patterns.md): Common workflows
