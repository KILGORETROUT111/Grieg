# Grieg — Reasoning Engine (Scaffold v2)

## Basis5 Contract — Self‑Projection Mandate

**Grieg** and **LEE** only code what they *self‑project* by **Basis5**. Outputs are not narratives first; they are *derivations* from the engine’s own phase geometry.

- **Phase Geometry = Proof (non‑optional).** The fourfold cycle **VAC → ALIVE → JAM → MEM** is the machine’s skeleton. Each run must emit its **deterministic geometry** alongside text.
- **Material Implication ≡ Phase Geometry (unified).** In Basis5 the “if–then” of material implication is **unpacked/excavated** into phase transitions. Inference is the geometry; geometry is the inference.
- **Artifacts per run (non‑negotiable):**
  - `result.json` — machine result (states, transitions, anchors, dominance)
  - `graph.svg` — deterministic phase graph (same seed ⇒ same graph)
  - `events.jsonl` — audit trail (provenance, witnesses, contradictions)
- **Dominance order:** `JAM > MEM > VAC > ALIVE` governs resolution/conflict.
- **Narration secondary.** Any GPT/LLM layer may describe results, but it **cannot replace or suppress geometry**. If geometry is missing, the run is invalid.

See **[CONTRACTS_BASIS5.md](docs/CONTRACTS_BASIS5.md)** for the compact, and **[docs/verification.md](docs/verification.md)** for conformance and golden proofs.


Four-fold phases: **ALIVE, JAM, MEM, VAC**.  
**Dominance:** `JAM > MEM > VAC > ALIVE`.

Crates: `grieg-engine`, `grieg-parser`, `grieg-cli`, `grieg-proptest`.

## Build (Rust)
```bash
cargo build


# Grieg — Reasoning Engine (Scaffold v2)

Four-fold phases: ALIVE, JAM, MEM, VAC.  
Dominance: JAM > MEM > VAC > ALIVE.

Crates: `grieg-engine`, `grieg-parser`, `grieg-cli`, `grieg-proptest`.

## Build (Rust)
```bash
cargo build

## IP & Trademarks
- Code: Apache-2.0 (see `LICENSE`).
- Patent notice: see `spec/SPEC.md` (Provisional details).
- Trademark: “Grieg” (see `TRADEMARKS.md`).
- Attribution: see `NOTICE`.


## Lineage & Rationale
- See **[LEE → Grieg Lineage](docs/lineage.md)** (informative).
- Spec: `spec/SPEC.md`
- Design ledger: `spec/LEDGER.md`
- Torwards Provable Substrates & Parallel to Rust Verification: `docs/VERIFICATION.md`

## Positioning w.r.t. AI (Searle)

Grieg is Weak-AI in Searle’s sense: a transparent, testable reasoning substrate.  
We make no Strong-AI claim (no “understanding” by program alone). Instead, Grieg
renders semantics operational: every evaluation returns a classical value and
a phase (ALIVE / JAM / MEM / VAC), surfacing edge conditions (unknowns, vacuity,
boundary failures, witness/memory transport) that most systems hide.

If grounding beyond symbols is desired, embed Grieg in systems with perception/actuation
or attach domain ontologies. Grieg’s role is to keep the inferential core explicit,
auditable, and falsifiable.

## Papers & Funding
- [Aims (2-page)](docs/aims.md)
- [Novelty Audit](docs/novelty-audit.md)
- [Main Paper LaTeX (LaTeX)](docs/grieg-main.tex)
- [Main Paper PDF (PDF)](docs/grieg-main.pdf)
- [Whitepaper](docs/Grieg-Whitepaper.md)

---

## Quickstart

```bash
# Build
cargo build

# Single expression
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --ast --mem

# REPL
cargo run -p grieg-cli -- --repl --mem

# JSONL batch (see samples)
cargo run -p grieg-cli -- --jsonl docs/samples/expressions.txt --mem --ast

# Manifest (version/build info)
target/debug/grieg-cli --manifest

# Verification example
cargo run -p grieg-cli -- --expr 'A -> B' --pretty


# About the maintainer and inventor


## Quality & Review Aids

- **QA Checklist:** [docs/quality/qa-checklist.md](docs/quality/qa-checklist.md)  
- **QA Positioning:** [docs/quality/qa-positioning.md](docs/quality/qa-positioning.md)
- **Runbook (Connector):** [docs/quality/runbook-telegram-connector.md](docs/quality/runbook-telegram-connector.md)
- **Runbook (HTTP Adapter):** [docs/quality/runbook-engine-http-adapter.md](docs/quality/runbook-engine-http-adapter.md).
