# Redis stream delivery

Redis is one of three delivery targets for Edge alerts. Pick it when you already run Redis and want events to arrive as stream entries your backend can consume asynchronously. The other two are [Webhooks](webhooks.md) and Telegram.

Redis delivery is **push**. Edge writes to the stream you configure; your consumers read from it at their own pace.

{% hint style="warning" %}
Redis-stream delivery is **configured in the Edge webapp** (Settings > Alerts > New Alert > Redis), not via MCP actions. This page documents the consumer-side contract: how stream entries are shaped and how to read them.
{% endhint %}

## Configure in the webapp

1. Open [edge.trade](https://edge.trade) and go to **Settings > Alerts**.
2. Click **New alert**, pick the alert type (e.g. `on_wallet_swaps`).
3. Fill in the per-alert filters (e.g. wallet address).
4. Pick **Redis** as the delivery target.
5. Provide a Redis URL (`redis://host:6379` or `rediss://...` for TLS) and a stream name.
6. Save.

Browse the live catalog of available alert names and per-alert input shapes via the `edge://alerts` MCP resource.

## Consuming the stream

Every alert becomes an entry on the Redis stream you named. Consume it like any other Redis stream.

### Node.js (ioredis)

```javascript
import Redis from "ioredis";

const redis = new Redis("redis://host:6379");
let lastId = "$"; // start from new messages only

while (true) {
  const result = await redis.xread("BLOCK", 5000, "STREAMS", "edge.whale.swaps", lastId);
  if (!result) continue;
  for (const [stream, entries] of result) {
    for (const [id, fields] of entries) {
      // fields is a flat array: [key1, value1, key2, value2, ...]
      const event = Object.fromEntries(chunk(fields, 2));
      handle(event);
      lastId = id;
    }
  }
}
```

### Python (redis-py)

```python
import redis
r = redis.Redis.from_url("redis://host:6379")

last_id = "$"
while True:
    resp = r.xread({"edge.whale.swaps": last_id}, count=100, block=5000)
    if not resp:
        continue
    for stream, entries in resp:
        for entry_id, fields in entries:
            handle(fields)
            last_id = entry_id
```

## When to prefer Redis over webhook

| Concern | Redis stream | Webhook |
|---------|--------------|---------|
| **Requires public endpoint** | No (internal Redis URL) | Yes |
| **Durability** | Yes (Redis persists until read or trimmed) | No (delivery retry only) |
| **Back-pressure** | Consumer reads at its own pace | Edge expects 2xx within seconds |
| **Fan-out** | Multiple consumers can read the same stream | One webhook per registration |
| **Good for** | Backends that already run Redis; async pipelines | Third-party tools, Zapier, no-code flows |

## Stop the stream

Delete or pause the alert from the webapp's alerts dashboard. Existing entries on the stream remain until your consumer reads them or the stream is trimmed.

## See also

- [Subscriptions](subscriptions.md): alert types, delivery catalog, configuration overview
- [Webhooks](webhooks.md): HTTP push delivery with HMAC verification
- [alerts tool reference](tools/alerts.md): `edge://alerts` MCP resource
- [Errors: Subscription errors](errors.md#subscription-errors): retry semantics
