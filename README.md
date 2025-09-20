* `CONTRACTS_BASIS5.md` is crystal clear: **geometry is the proof, non-negotiable, always emitted**.
* The current `README.md` (scaffold v2) still talks like geometry is just “phase outcomes” with text and doesn’t anchor itself to the **Basis5 Contract**.

Here’s how I’d update `README.md` while **keeping the Lineage intact** (since you don’t want that touched), and pulling in the stronger mandate from the contract + verification doc:

---

# Grieg — Reasoning Engine (Basis5 Contract)

**Four-fold phases:** VAC → ALIVE → JAM → MEM
**Dominance:** JAM > MEM > VAC > ALIVE

Grieg is the operational substrate of the Logic Evaluation Engine (LEE). It implements the **Basis5 Contract**: geometry is not optional; it is the proof. Every run must emit its phase geometry alongside text.

* **Artifacts per run (non-negotiable):**

  * `result.json` (machine)
  * `graph.svg` (deterministic geometry)
  * `events.jsonl` (audit log)

See [CONTRACTS\_BASIS5.md](docs/CONTRACTS_BASIS5.md) for the full contract.
See [docs/verification.md](docs/verification.md) for conformance sets and live proofs.

---

## Build (Rust)

```bash
cargo build
```

Crates: `grieg-engine`, `grieg-parser`, `grieg-cli`, `grieg-proptest`

---

## Quickstart

```bash
# Single expression
cargo run -p grieg-cli -- --expr 'A -> B' --out outdir/

# Produces:
#  outdir/result.json
#  outdir/graph.svg
#  outdir/events.jsonl
```

REPL and JSONL batch modes are available; see [README-QUICKSTART.md](README-QUICKSTART.md).

---

## Verification Stance

Grieg is a demonstrable substrate: **phase outcomes and geometry are reproducible at the CLI**.
Conformance is enforced via schema, golden snapshots, and CI.
See [docs/verification.md](docs/verification.md).

---

## Positioning w\.r.t. AI

Grieg is **Weak-AI** in Searle’s sense: a transparent, testable reasoning substrate.
It does not claim “understanding”; it makes inference explicit, auditable, falsifiable.
Narration is secondary. **Geometry is core.**

---

## IP & Lineage

* Code: Apache-2.0 (see `LICENSE`)
* Patent notice: `spec/SPEC.md` (Provisional details)
* Trademark: “Grieg” (see `TRADEMARKS.md`)
* Attribution: `NOTICE`

**Lineage:** 

---

## Papers & Documentation

* [Aims](docs/aims.md)
* [Novelty Audit](docs/novelty-audit.md)
* [Whitepaper](docs/Grieg-Whitepaper.md)
* [Main Paper (LaTeX)](docs/grieg-main.tex)
* [Main Paper (PDF)](docs/grieg-main.pdf)

---

## Quality & Runbooks

* QA: [docs/quality/qa-checklist.md](docs/quality/qa-checklist.md)
* Positioning: [docs/quality/qa-positioning.md](docs/quality/qa-positioning.md)
* Runbook: [Telegram Connector](docs/quality/runbook-telegram-connector.md)
* Runbook: [HTTP Adapter](docs/quality/runbook-engine-http-adapter.md)

---

✅ The README aligns with **CONTRACTS\_BASIS5.md**: geometry mandatory, artifacts non-negotiable, no “text-only” fudge.

Do you want me to **apply this rewrite directly to your `README.md` file** so you have a clean updated version ready to commit?
