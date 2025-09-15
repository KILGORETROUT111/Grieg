# Grieg Project Snapshot — September 2025

## Current State (as of v0.3.1)

### Engine (Rust)
- Stable public facade: `Evaluator`, `Phase`, `Expr`.
- Four-fold phases working end-to-end (ALIVE, JAM, MEM, VAC) with correct dominance.
- CLI: eval single expr / JSONL batch / REPL; pretty/ast flags functional.
- Python binding (`grieg.expr`) returning structured dicts: `{expr, mem, phase, value}`.
- Features: `emit_geometry` stubbed, new `lam` scaffold compiling (feature-gated).

### Frontends
- CLI and Python both demonstrable with canonical examples (`phase-tour-complete.md`).
- Local wheel/dev install confirmed for Python.

### Conformance & Docs
- `conformance/smoke.jsonl` canonicalized for phase correctness.
- `examples/phase-tour-complete.md` with Python mirrors.
- `README.md` cleaned with banner + quickstart pointer.
- `README-QUICKSTART.md` ready for researchers.
- `docs/verification.md` added.
- IP/Trademarks/Security docs in place.

### Build/Release
- Workspace builds clean.
- Tag `v0.3.1` created; wheel workflow staged.

---

## Next Steps (To-Do)

### 1. Researcher UX
- Add Python adapter for values: map Bool → bool, Unknown → None, else string.
- Add `examples/README-EXAMPLES.md` as index.

### 2. Conformance in CI
- GitHub Action step to run CLI against `smoke.jsonl`.
- Fail on mismatches.
- Keep λ tests feature-gated.

### 3. Release Hygiene
- Update `.gitignore`:
  ```
  .venv/
  target/
  __pycache__/
  *.Zone.Identifier
  ```
- Remove large zips from git history going forward.

### 4. λ/Y Path
- Decide surface: API or syntax.
- Implement β-reduction with fuel.
- Map divergence → JAM, normal form → ALIVE.
- Add `examples/onramp/04_lambda.md`.
- Ship `conformance/lambda.jsonl`.

### 5. Performance & Determinism
- Add `cargo bench` scaffolding.
- Add CLI `--manifest` subcommand to output version/features.

### 6. API Polish
- Python: keyword args for `expr`, `version()`, `features()`.
- Optional bindings for quick researcher use.

### 7. Docs Small Wins
- `verification.md`: add “how to re-run proofs”.
- Add SECURITY contact to README footer.

### 8. Integration Seeds
- Draft `examples/python/quickstart.ipynb` (or .py fallback).

---

## Assessment

- Substrate is **solid**: phases, CLI, Python, conformance, docs reproducible.
- Repo is **funder-ready** for methods demos + light integrations.
- λ/Y not baseline but clean feature-gated lane exists.
- Path to **provable/demonstrable**: conformance in CI + reproducible outputs across CLI/Python.

---

## Immediate Priorities
1. Lock researcher UX (Python adapter + examples index).
2. Conformance smoke test wired into CI.
3. Release hygiene commit.
4. λ demo optional, gated.
