# Grieg — Quickstart Guide

Grieg is a four-phase reasoning engine in Rust (`ALIVE`, `JAM`, `MEM`, `VAC`).  
This guide shows you how to clone, build, and run **your own instance** — in Rust or Python.

---

## 1. Clone & Build

```bash
git clone https://github.com/KILGORETROUT111/Grieg.git
cd Grieg
cargo build
```

Quick test:

```bash
cargo run -p grieg-cli -- --expr "A -> B" --pretty
```

Expected: **VAC** (no witness assigned).

---

## 2. Python Bindings

1. Create a venv:

   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   ```

2. Install [maturin](https://github.com/PyO3/maturin):

   ```bash
   pip install --upgrade pip maturin
   ```

3. Build & install bindings:

   ```bash
   maturin develop -m grieg-py/pyproject.toml
   ```

4. Test in Python:

   ```python
   import grieg
   print(grieg.expr("A -> B", mem=False))          # VAC
   print(grieg.expr("@mem(true -> false)", mem=True))  # MEM
   print(grieg.expr("@jam(true)", mem=False))          # JAM
   ```

---

## 3. Canonical Examples

The [phase-tour](examples/phase-tour-complete.md) shows each phase in action.

Run the conformance set:

```bash
cargo run -p grieg-cli -- --jsonl conformance/smoke.jsonl --pretty
```

---

## 4. Documentation

- [verification.md](docs/verification.md) — rationale and links to formal verification.  
- [spec/SPEC.md](spec/SPEC.md) — ontology and semantics.  
- [docs/Grieg-Whitepaper.md](docs/Grieg-Whitepaper.md) — overview.  

Serve docs locally with MkDocs:

```bash
pip install mkdocs
mkdocs serve
```

Then open [http://127.0.0.1:8000](http://127.0.0.1:8000).

---

## 5. Baseline & Extensions

You can stay with this baseline instance for reproducibility,  
or probe deeper with optional features (e.g., geometry emission, Python notebooks, bindings).  

The **same instance** remains valid as you explore.

---

👉 Start with section **1** (Rust) or **2** (Python).  
That’s all you need to run your own Grieg engine.

---

⚖️ *License: Apache-2.0. See [IP_Policy.md](IP_Policy.md) for background theory notes.*
