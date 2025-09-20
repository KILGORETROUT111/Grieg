# Runbook — Rust HTTP Adapter (grieg-http)

**Purpose:** Expose the Rust CLI (`grieg-cli`) on HTTP for the Telegram connector.

## Build
```
cargo build -p grieg-cli --release
cargo build -p grieg-http --release
```

## Run (foreground)
```
export GRIEG_CLI_CMD="$PWD/target/release/grieg-cli"
# export GRIEG_CLI_ARGS="--json"   # if your CLI needs it
target/release/grieg-http
# prints: grieg-http listening on 127.0.0.1:8000
```

## Run (background)
```
nohup env GRIEG_CLI_CMD="$GRIEG_CLI_CMD" GRIEG_CLI_ARGS="${GRIEG_CLI_ARGS:-}"   target/release/grieg-http > ~/grieg_http.log 2>&1 & echo $! > ~/grieg_http.pid
```

## Verify
```
curl -sS -H 'Content-Type: application/json'   -d '{"prompt":"A -> B"}' http://127.0.0.1:8000/api/v1/evaluate
```

## Change port if 8000 busy
- Start with `BIND=127.0.0.1:8010 target/release/grieg-http`
- Update `.env`: `LEE_ENDPOINT=http://127.0.0.1:8010/api/v1/evaluate`
