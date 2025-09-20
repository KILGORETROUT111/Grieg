#!/usr/bin/env bash
set -euo pipefail
echo "== :8000 =="
(ss -ltnp 'sport = :8000' || netstat -ltnp 2>/dev/null | grep ':8000' || true) | sed 's/^/  /'
echo
echo "== probe =="
curl -sS -X POST -H 'Content-Type: application/json' -d '{"prompt":"A -> B"}' http://127.0.0.1:8000/api/v1/evaluate || true
echo
echo "== bot =="
cd "$HOME/code/grieg/grieg_v2/tools/telegram/lee-grieg-telegram-connector"
if [ -f bot.pid ] && ps -p "$(cat bot.pid)" >/dev/null 2>&1; then
  echo "  running (pid $(cat bot.pid))"
else
  echo "  not running"
fi
tail -n 20 bot.log 2>/dev/null || true
