#!/usr/bin/env bash
set -euo pipefail
kill "$(cat "$HOME/grieg_http.pid" 2>/dev/null)" 2>/dev/null || true
