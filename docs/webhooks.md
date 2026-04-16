# Webhooks

Webhook is one of three delivery targets for Edge alerts. Pick it when you want events to arrive at an HTTP endpoint outside your agent. The other two are [Redis stream](redis-streams.md) and Telegram.

Webhook delivery is **push**. Edge POSTs to your URL as events happen. You do not poll.

{% hint style="warning" %}
Webhooks are **configured in the Edge webapp** (Settings > Alerts > New Alert > Webhook), not via MCP actions. This page documents the receiver-side contract: payload shape, signature verification, and local-testing workflow.
{% endhint %}

## Configure in the webapp

1. Open [edge.trade](https://edge.trade) and go to **Settings > Alerts**.
2. Click **New alert**, pick the alert type (e.g. `on_portfolio_updates`).
3. Fill in the per-alert filters (e.g. wallet address, pair address).
4. Pick **Webhook** as the delivery target.
5. Provide your HTTPS endpoint URL and a long-random shared secret.
6. Save.

Browse the live catalog of available alert names and per-alert input shapes via the `edge://alerts` MCP resource.

## Payload shape

Each delivery is a POST with JSON body:

```json
{
  "event": "on_portfolio_updates",
  "data": {"...": "..."},
  "timestamp": 1702000720,
  "signature": "sha256=abc123..."
}
```

`signature` is present only when you registered with a `secret`.

## Verify the signature

Edge signs the raw request body with your `secret` using HMAC-SHA256. Verify before trusting the event.

### Node.js

```javascript
import { createHmac, timingSafeEqual } from "node:crypto";

function verifyEdgeWebhook(rawBody, signatureHeader, secret) {
  const expected = "sha256=" + createHmac("sha256", secret)
    .update(rawBody)
    .digest("hex");
  const a = Buffer.from(signatureHeader);
  const b = Buffer.from(expected);
  return a.length === b.length && timingSafeEqual(a, b);
}
```

### Python

```python
import hmac, hashlib

def verify_edge_webhook(raw_body: bytes, signature_header: str, secret: str) -> bool:
    expected = "sha256=" + hmac.new(
        secret.encode(), raw_body, hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(expected, signature_header)
```

### Rules that keep verification safe

- Compute the HMAC over the **raw request body**, not a re-serialized JSON. If your framework parses the body before you see it, configure it to expose the raw bytes for webhook routes.
- Use constant-time comparison (`timingSafeEqual` / `hmac.compare_digest`). Plain string equality leaks information.
- Rotate `secret` on a schedule: create a new webhook subscription with a new secret, verify it works, then delete the old one.

## Responding to deliveries

- Return any `2xx` status within a few seconds to acknowledge.
- A `4xx`/`5xx` or unreachable endpoint surfaces as `WEBHOOK_DELIVERY_FAILED` in the webapp alerts dashboard (see [Errors: Subscription errors](errors.md#subscription-errors)).
- Offload heavy work to a queue. Return `200` fast.

## Testing locally

Expose your local endpoint over HTTPS:

- **ngrok**: `ngrok http 3000` gives you a temporary `https://*.ngrok.io` URL
- **Cloudflare Tunnel**: `cloudflared tunnel --url http://localhost:3000`

Paste the tunnel URL into the webapp alerts form. Delete the alert when you finish testing.

## Using from Zapier, n8n, and Make

Any no-code platform with a "Webhook trigger" step can consume Edge events:

1. Create a webhook trigger in your platform. It gives you a public URL.
2. In the Edge webapp, create an alert with that URL as the delivery target and a long random secret.
3. In your platform, add a verification step that computes HMAC-SHA256 of the raw body and compares to the `signature` field.

## Comparing webhook to other delivery targets

| Concern | Webhook | Redis stream | Telegram |
|---------|---------|--------------|----------|
| **Requires public endpoint** | Yes | No (internal) | No |
| **Signed delivery** | Yes (HMAC optional) | No | No |
| **Good for** | Server-based automation | Backends on Redis | Personal notifications |
| **Response needed** | `2xx` within seconds | N/A | N/A |

## See also

- [Subscriptions](subscriptions.md): alert types, delivery catalog, configuration overview
- [Redis stream delivery](redis-streams.md): push events onto a Redis stream
- [alerts tool reference](tools/alerts.md): `edge://alerts` MCP resource
- [Errors: Subscription errors](errors.md#subscription-errors): retry semantics
