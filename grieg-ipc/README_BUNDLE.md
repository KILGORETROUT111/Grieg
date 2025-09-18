# Grieg IPC Bundle (Telemetry + JSONL CLI)

This adds **two crates** to your Grieg workspace _without touching existing code_:

- `grieg-telemetry/` — tiny library with a JSONL sink (`JsonlSink`) and a `TelemetryEvent` struct.
- `grieg-ipc-cli/` — a small CLI that evaluates expressions via `grieg-engine` and writes one
  JSONL record per evaluation (an “invariant pulse channel” v0.1 end-of-expr record).

## Install (2 minutes)

1) Unzip at the **root** of your Grieg workspace (the folder that already contains `grieg-engine/`, etc.).
2) Edit your root `Cargo.toml` to include the new members:
   ```toml
   [workspace]
   members = [
       "grieg-engine",
       "grieg-parser",
       "grieg-cli",
       "grieg-proptest",
       "grieg-telemetry",
       "grieg-ipc-cli",
   ]
   ```
3) Build the new CLI:
   ```bash
   cargo build -p grieg-ipc-cli
   ```

## Usage

```bash
# Pretty output + JSONL emission
cargo run -p grieg-ipc-cli --   --expr "@mem(true -> false)" --mem --pretty --jsonl ./ipc.jsonl

# REPL mode with JSONL sink
cargo run -p grieg-ipc-cli -- --repl --mem --jsonl ./ipc.jsonl
> ~false & (true | false)
> :q
```

`ipc.jsonl` will contain one object per evaluation, for example:

```json
{"ts":"2025-09-10T12:34:56.789Z","expr":"@mem(true -> false)","ast":"(@mem (-> true false))","phase":"MEM","value_text":"Bool(false)","value_bool":false,"sink":false,"jam":false,"channel":"ipc.v0"}
```

## Notes

- The CLI calls `Evaluator::new(mem)` and `eval(&expr, None)`; it does **not** modify engine semantics.
- If/when you want **per-step** traces, we can add a feature-gated emitter inside `grieg-engine`; for now this bundle provides a safe end-of-expr channel.
- Schema is deliberately conservative (text + optional bool) to avoid tight coupling with internal `V` variants.

— Enjoy!
