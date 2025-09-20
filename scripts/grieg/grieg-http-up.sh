#!/usr/bin/env bash
set -euo pipefail
cd "$HOME/code/grieg/grieg_v2"
export GRIEG_CLI_CMD="$PWD/target/release/grieg-cli"
# Uncomment if your CLI has a JSON flag:
# export GRIEG_CLI_ARGS="--json"
nohup env GRIEG_CLI_CMD="$GRIEG_CLI_CMD" GRIEG_CLI_ARGS="${GRIEG_CLI_ARGS:-}" \
  target/release/grieg-http > "$HOME/grieg_http.log" 2>&1 & echo $! > "$HOME/grieg_http.pid"
sleep 0.3
