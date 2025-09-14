# 03 — Geometry trace (optional feature)

Build with feature:
```bash
cargo build -p grieg-cli --features emit_geometry
```

Then run CLI with your trace flag (example name):
```bash
target/release/grieg-cli --expr '(x & true) -> y' --mem --trace-json > trace.json
```

`trace.json` items should match `docs/trace.schema.json`.
