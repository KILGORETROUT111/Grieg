set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# --- Common flags ---
export RUSTFLAGS := ""
export CARGO_TERM_COLOR := "always"

alias b := build
alias t := test
alias r := run
alias cl := clippy

# Build / test / lint (workspace)
build:        # just b
	cargo build --workspace

test:         # just t
	cargo test  --workspace --no-fail-fast

clippy:       # just cl
	cargo clippy --workspace --all-targets -- -D warnings

fmt:          # just fmt
	cargo fmt --all

ci:           # just ci
	just fmt
	just cl
	just t

# Dev loop: re-check/test on change
watch:        # just watch
	cargo watch -q -c -w grieg-engine -w grieg-parser -w grieg-cli -x "check" -x "test -p grieg-proptest"

# CLI helpers
expr *ARGS:   # just expr '@mem(true -> false)' --mem --pretty
	cargo run -p grieg-cli -- --expr {{ARGS}}

repl *FLAGS:  # just repl --mem --ast
	cargo run -p grieg-cli -- --repl {{FLAGS}}

jsonl FILE *FLAGS:  # just jsonl ../../grieg_full_spec_pack/samples/expressions.txt --mem --ast
	cargo run -p grieg-cli -- --jsonl {{FILE}} {{FLAGS}}

manifest:     # just manifest
	target/debug/grieg-cli --manifest || cargo run -p grieg-cli -- --manifest

push:         # just push "msg here"
	git add -A
	git commit -s -m "{{1}}"
	git pull --rebase origin main
	git push

# Fast paths
engine:       # just engine
	cargo build -p grieg-engine

parser:       # just parser
	cargo build -p grieg-parser

cli:          # just cli
	cargo build -p grieg-cli
