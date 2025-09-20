# LEE ↔ Telegram Connector (Scaffold)

Minimal, production-lean scaffold to ingest Telegram messages into LEE:
- FastAPI **gateway** (webhook receiver, normalization, enqueue)
- **Redis** queue
- **Worker** (hashing, simple claim extraction, persistence)
- **Postgres** storage

> This is only **one** use of LEE Inc as engine (Telegram ingress). LEE supports other pipes (email, files, voice) separately.

## Quick start
1. `cp .env.example .env` and set `TG_BOT_TOKEN` and `PUBLIC_WEBHOOK_BASE`.
2. `docker compose up --build`
3. Set Telegram webhook (replace TOKEN & URL):
   ```bash
   curl -X POST "https://api.telegram.org/bot$TG_BOT_TOKEN/setWebhook"          -H "Content-Type: application/json"          -d '{"url":"'$PUBLIC_WEBHOOK_BASE'/ingest/tg"}'
   ```
4. DM your bot or add to a group, then try:
   - Type “We’ll pay Friday.” and “No funds until next month.”
   - Later, call `/map` or `/witness` (stubbed behaviors return JSON).

## Services
- `gateway`: receives Telegram updates at `POST /ingest/tg` and enqueues normalized Events
- `worker`: consumes queue, hashes evidence, extracts simple claims, persists rows
- `postgres`: tables `events`, `claims` (auto-migrated on worker start)

## Caveats
- This scaffold ignores Telegram Secret Chats (use TDLib mode for that, later).
- Security & consent features are rudimentary here; extend before production.
- NLP is rule-based placeholder; swap with your LEE NLP when ready.
