# Grieg — Frozen Snapshot (0–6)

**Date:** 2025-09-16

This snapshot freezes the current plan/items **0–6** for pickup in a few days.

---

## 0) Current Repo State (frozen)
- **Engine (Rust):** `Evaluator`, `Phase`, `Expr` stable; phases ALIVE/JAM/MEM/VAC working end-to-end.  
- **Features:** `emit_geometry` (optional), `lam` scaffold (feature-gated, compiles).  
- **Frontends:** CLI (expr/JSONL/REPL, `--ast`, `--pretty`), Python binding (`grieg.expr()`), maturin dev install.  
- **Conformance & Docs:** `conformance/smoke.jsonl`, `conformance/phase-dominance.jsonl`; `examples/phase-tour-complete.md`; `README.md` + `README-QUICKSTART.md`; `docs/verification.md`.  
- **IPC Bundle:** `grieg-ipc/{grieg-ipc-cli, grieg-telemetry}` integrated; deps/path fixes; chrono with `serde` feature enabled.  
- **Tag:** v0.3.1; workspace builds clean.  

---

## 1) Researcher UX (freeze scope)
- Python value adapter (map `Bool` → `bool`, Unknown → `None`, else `str`).  
- `examples/README-EXAMPLES.md` index linking phase tour & JSONL sets.  
- Keep Quickstart as single entry point for newcomers.  

**Status:** Planned — not yet implemented.  

---

## 2) Conformance in CI (freeze scope)
- Add GH Actions steps to run:  
  - `cargo run -p grieg-cli -- --jsonl conformance/smoke.jsonl --mem`  
  - `cargo run -p grieg-cli -- --jsonl conformance/phase-dominance.jsonl --mem`  
- Optional: fail job when any line reports `"ok": false`.  

**Status:** Planned — CLI JSONL handler patch drafted; integrate into CI next.  

---

## 3) Release Hygiene (freeze scope)
- `.gitignore`: add `.venv/`, `target/`, `__pycache__/`, `*.Zone.Identifier`.  
- Avoid committing large binaries/zips going forward.  

**Status:** Partially done; confirm `.gitignore` contains all entries.  

---

## 4) λ/Y Path (feature-gated) (freeze scope)
- Decide surface (API-first or mini-syntax).  
- Implement β-reduction (normal-order) with fuel; divergence → `JAM`, normal form → `ALIVE`.  
- `examples/onramp/04_lambda.md` + `conformance/lambda.jsonl` when evaluator is ready.  
- Keep under `--features lam` to protect baseline.  

**Status:** Scaffold present; evaluator TODO.  

---

## 5) Performance & Determinism (freeze scope)
- Add `cargo bench` scaffolding (criterion) with 2–3 canonical expressions.  
- Add CLI `--manifest` (print version + features) to stamp runs.  

**Status:** Planned.  

---

## 6) API Polish (freeze scope)
- Python: ensure keyword args (`expr`, `mem=False`), add `grieg.version()` & `grieg.features()`.  
- Optional: simple `.ipynb` or `.py` quickstart in `examples/python/`.  

**Status:** Planned.  

---

### Notes
- Phase dominance invariant: **JAM > MEM > VAC > ALIVE** (tests reflected in JSONL sets).  
- Keep geometry optional; evaluation outcomes must remain geometry-agnostic.  
- JSONL is first-class I/O for conformance & analytics.  
