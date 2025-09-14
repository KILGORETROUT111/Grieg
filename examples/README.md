# Examples

Small, copy-pasteable snippets that exercise **Grieg** via the CLI.  
These examples are designed to be readable on mobile and runnable on a laptop without extra setup.


### Examples
See examples/ phase-tour-complete.md for copy-pasteable CLI runs

- ✅ No extra crates needed  
- ✅ Works with `grieg-cli`  
- ✅ Shows **phase** output (ALIVE / JAM / MEM / VAC)  

## How to run

From repo root:

```bash
# Build once
cargo build

# Run a single expression with pretty output and MEM enabled
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty

# Grieg — Reasoning Engine (Scaffold v2)

Four-fold phases: ALIVE, JAM, MEM, VAC.  
Dominance: JAM > MEM > VAC > ALIVE.

Crates: `grieg-engine`, `grieg-parser`, `grieg-cli`, `grieg-proptest`.

## Build (Rust)
```bash
cargo build



