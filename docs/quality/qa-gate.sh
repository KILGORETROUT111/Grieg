#!/usr/bin/env bash
set -euo pipefail
REPO="${REPO:-$HOME/code/grieg/grieg_v2}"
BOT="$REPO/tools/telegram/lee-grieg-telegram-connector"
ENV="$BOT/.env"
errors=0
warn(){ echo "[WARN] $*"; }
fail(){ echo "[FAIL] $*"; errors=$((errors+1)); }

# QA docs present
[ -f "$REPO/docs/quality/qa-checklist.md" ]    || fail "missing docs/quality/qa-checklist.md"
[ -f "$REPO/docs/quality/qa-positioning.md" ]  || fail "missing docs/quality/qa-positioning.md"

# Bot env
[ -f "$ENV" ] || fail "missing $ENV (needs TELEGRAM_BOT_TOKEN, LEE_ENDPOINT)"
if [ -f "$ENV" ]; then
  set -a; . "$ENV"; set +a
  [ -n "${TELEGRAM_BOT_TOKEN:-}" ] || fail "TELEGRAM_BOT_TOKEN missing"
  [ -n "${LEE_ENDPOINT:-}" ]       || fail "LEE_ENDPOINT missing"
fi

HOST=$(echo "${LEE_ENDPOINT:-}" | awk -F[/:] '{print $4}')
PORT=$(echo "${LEE_ENDPOINT:-}" | awk -F[/:] '{print $5}')
[ -z "${PORT:-}" ] && PORT=8000

# Engine listener?
if ! ss -ltn "( sport = :$PORT )" 2>/dev/null | grep -q LISTEN; then
  fail "no listener on :$PORT (start grieg-http)"
fi

# Probe engine (2s)
RESP=$(curl -sS --max-time 2 -H 'Content-Type: application/json' \
  --data-binary '{"prompt":"A -> B"}' "${LEE_ENDPOINT:-http://127.0.0.1:8000/api/v1/evaluate}" || true)
echo "$RESP" | grep -q '"phase"' || fail "probe lacks \"phase\" key"
echo "$RESP" | grep -Eoq '"phase"\s*:\s*"(VAC|JAM|MEM|ALIVE)"' || warn "phase not in {VAC,JAM,MEM,ALIVE}"

# Spot checks (robust JSON payload builder)
spot_check() {
  local expr="$1"
  local payload
  payload=$(python3 -c 'import json,sys; print(json.dumps({"prompt": sys.argv[1]}))' "$expr")
  curl -sS --max-time 3 -H 'Content-Type: application/json' --data-binary "$payload" "${LEE_ENDPOINT}" || true
}

for expr in '@mem(true -> false)' 'A -> B'; do
  r="$(spot_check "$expr")"
  echo "$r" | grep -q '"phase"' || fail "no phase for: $expr"
done

# Bot state
if [ -f "$BOT/bot.pid" ] && ps -p "$(cat "$BOT/bot.pid")" >/dev/null 2>&1; then
  echo "[ok] bot running (pid $(cat "$BOT/bot.pid"))"
else
  warn "bot not running; restart with your bot script if needed"
fi

# Result
if [ $errors -eq 0 ]; then
  echo "[PASS] QA gate passed"
  exit 0
else
  echo "[FAIL] $errors issue(s)"
  exit 1
fi
