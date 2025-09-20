# Grieg ↔ Telegram Connector — Automated Fix Package (in-place)
What this adds:
- One script to init deps, validate :8000 endpoint, start/stop bot
- .env example, no interactive editing
- Optional smart stub for testing

Quick use:
  bash connector_fix/grieg_telegram_fix.sh init
  bash connector_fix/grieg_telegram_fix.sh status
  bash connector_fix/grieg_telegram_fix.sh start
Optional:
  bash connector_fix/grieg_telegram_fix.sh stub-up
  bash connector_fix/grieg_telegram_fix.sh stub-down
  bash connector_fix/grieg_telegram_fix.sh stop
