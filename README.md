# Grieg — Reasoning Engine (Scaffold v2)
Four-fold phases: ALIVE, JAM, MEM, VAC. Dominance: JAM > MEM > VAC > ALIVE.
Crates: grieg-engine, grieg-parser, grieg-cli, grieg-proptest.
Build: cargo build
Run: cargo run -p grieg-cli -- --expr "@mem(true -> false)" --ast --mem
