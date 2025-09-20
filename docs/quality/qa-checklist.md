# QA Checklist (daily)

- [ ] **Engine listener** is up
  - `ss -ltnp 'sport = :8000'` ✓ (or your chosen port)
  - `curl -sS -H 'Content-Type: application/json' -d '{"prompt":"A -> B"}' http://127.0.0.1:8000/api/v1/evaluate`
- [ ] **Bot** is running
  - `tail -n 40 tools/telegram/lee-grieg-telegram-connector/bot.log`
- [ ] **.env** correct
  - `LEE_ENDPOINT=http://127.0.0.1:8000/api/v1/evaluate`
  - `JSONL_MODE=engine`
- [ ] **Smoke** from Telegram
  - `/expr A -> B` returns a phase
  - `/jsonl` on a known file returns `jsonl run complete: ok=…`
