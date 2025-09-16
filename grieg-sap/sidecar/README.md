# Sidecar: run Grieg near SAP/CPI
1) Build a tiny HTTP wrapper around `grieg-cli` or expose a Rust HTTP server crate.
2) Run on a VM/container reachable from SAP app servers and/or CPI.

**Example**: systemd service that runs `grieg-cli --repl` behind a small Rocket/Axum HTTP adapter.
