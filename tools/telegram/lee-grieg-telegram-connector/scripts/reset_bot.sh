#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
set -a; . "$ROOT/.env"; set +a
pkill -f "$ROOT/bot.py" 2>/dev/null || true
curl -sS "https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/deleteWebhook?drop_pending_updates=true" >/dev/null || true
source "$ROOT/.venv/bin/activate"
exec python "$ROOT/bot.py"
