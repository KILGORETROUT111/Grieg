# Runbook — Telegram Connector

**Files:** `tools/telegram/lee-grieg-telegram-connector/*`

## Configure
Create `.env`:
```
TELEGRAM_BOT_TOKEN=YOUR_TOKEN
LEE_ENDPOINT=http://127.0.0.1:8000/api/v1/evaluate
JSONL_MODE=engine
JSONL_MAX_LINES=1000
```

## Start
```
source ~/code/grieg/grieg_v2/.venv/bin/activate  # create if missing; install python-telegram-bot/httpx/python-dotenv
nohup env $(cat .env | xargs) python3 bot.py > bot.log 2>&1 & echo $! > bot.pid
tail -n 60 bot.log
```

## Use
- `/start` — help
- `/expr <code> [--mem] [--ast]`
- Upload `*.jsonl` with caption `/jsonl`

## Logs
- `bot.log`; errors:
```
grep -nE "ERROR|Traceback|Connect|Timeout" bot.log | tail -n 40
```
