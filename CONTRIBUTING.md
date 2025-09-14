# Contributing to Grieg

## Build & test
```bash
cargo build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Style
- `cargo fmt --all`
- Keep SPEC authoritative; update `spec/LEDGER.md` for notable changes.

## PRs
- One focused change per PR
- Include tests where possible
- Fill out the PR checklist
