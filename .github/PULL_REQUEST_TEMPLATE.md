## Summary
Explain what this PR does in one or two sentences.

## Changes
- Axum HTTP adapter (`grieg-http`) listening on 127.0.0.1:8000
- Telegram connector runbook + docs in `docs/quality/`
- QA gate + helper scripts (`scripts/grieg/*`)
- Hardened .gitignore for tokens/logs/venv

## Test plan
- `~/bin/grieg-up.sh` (starts adapter + bot)
- In Telegram: `/expr A -> B` returns a phase
- `~/bin/grieg-qa.sh` returns **PASS**

## Risks & mitigations
- Port 8000 could be busy → helper script evicts squatters and restarts adapter
- No secrets in repo: `.env` is gitignored; CI does not require tokens
