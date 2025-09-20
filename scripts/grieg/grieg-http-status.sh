#!/usr/bin/env bash
set -euo pipefail
echo "== listener =="
(ss -ltnp 'sport = :8000' || true) | sed 's/^/  /'
echo
echo "== probe =="
curl -sS --max-time 2 -H 'Content-Type: application/json' \
  -d '{"prompt":"A -> B"}' http://127.0.0.1:8000/api/v1/evaluate || echo "  (curl failed)"
echo
echo "== logs (tail) =="
tail -n 30 "$HOME/grieg_http.log" 2>/dev/null || true
