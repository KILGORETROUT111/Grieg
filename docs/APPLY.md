# Grieg Researcher Onramp — APPLY.md

This bundle is additive. Unzip at the repo root (`grieg_v2/`) and then:

1) **Enable optional geometry feature**

Edit `grieg-engine/Cargo.toml` and add (or merge) this section at the end:
```toml
[features]
emit_geometry = []
```

Edit `grieg-cli/Cargo.toml` dependency to enable the feature in the CLI:
```toml
[dependencies]
grieg-engine = { path = "../grieg-engine", features = ["emit_geometry"] }
```

2) **Add researcher quickstart to README**  
Copy `README-QUICKSTART.md` into the top of your README (or link to it).

3) **Python binding (optional now, valuable for adoption)**
- Move `grieg-py/` into the workspace root (sibling of `grieg-engine`).
- Install: `pipx install maturin` (or `pip install maturin`).
- Local dev: `cd grieg-py && maturin develop`
- Test in Python:
  ```python
  import grieg
  print(grieg.eval("@mem(true -> false)", mem=True, ast=True))
  ```

4) **Docs & discoverability**
- Copy `CITATION.cff` to repo root (so GitHub shows “Cite this repository”).
- Copy `docs/trace.schema.json` into `docs/` and link it from `spec/SPEC.md`.
- Optional: use `mkdocs.yml` + `docs/index.md` for GitHub Pages later.

5) **Examples**
- Copy `examples/` folder as-is; try `examples/01_basics.md` commands.
- Geometry demo in `examples/03_geometry.md` needs the feature enabled.

6) **Contrib hygiene**
- Copy `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md` to root.

7) **Smoke check**
```bash
cargo build --release
target/release/grieg-cli --expr '@mem(true -> false)' --mem --pretty
# with geometry enabled:
cargo build -p grieg-cli --features emit_geometry
```

That’s it. Keep SPEC authoritative; examples and schema should point back to SPEC/LEDGER.
