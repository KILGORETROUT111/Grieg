# Incident Playbook

**Symptom:** Bot is silent
1) `tail -n 80 tools/telegram/lee-grieg-telegram-connector/bot.log`
2) Check engine: `ss -ltnp 'sport = :8000'` → adapter listening?
3) Probe engine: `curl -sS -H 'Content-Type: application/json' -d '{"prompt":"A -> B"}' http://127.0.0.1:8000/api/v1/evaluate`
4) If 8000 is taken, restart adapter on 8010 and switch `.env`.

**Symptom:** All phases are `VAC/ALIVE`
- Wrong endpoint (stub/wrong process). Use `127.0.0.1:8000` for WSL real engine.

**Symptom:** “Failed to connect”
- Engine down or port blocked. Start `grieg-http` again.
