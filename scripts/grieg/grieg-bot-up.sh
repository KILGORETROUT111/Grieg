#!/usr/bin/env bash
set -euo pipefail
cd "$HOME/code/grieg/grieg_v2/tools/telegram/lee-grieg-telegram-connector"
perl -0777 -pe 's|^LEE_ENDPOINT=.*|LEE_ENDPOINT=http://127.0.0.1:8000/api/v1/evaluate|m' -i .env
set -a; . ./.env; set +a
source "$HOME/code/grieg/grieg_v2/.venv/bin/activate"
kill $(cat bot.pid 2>/dev/null) 2>/dev/null || true
rm -f bot.pid bot.log
nohup env TELEGRAM_BOT_TOKEN="$TELEGRAM_BOT_TOKEN" LEE_ENDPOINT="$LEE_ENDPOINT" JSONL_MODE="${JSONL_MODE:-engine}" \
  python3 bot.py > bot.log 2>&1 & echo $! > bot.pid
sleep 0.3
