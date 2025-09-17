**Lineage:** Operator discipline traces to Julian C. Boyd’s Berkeley program
(modality, deontic logic, syntax–semantics), with modern contrasts to context-
sensitive semantics and probabilistic / operational views. Grieg is the product
of 30 years of research; up to and including nights at the ETH library in Zürich
combing through the monographs of Alfred Lande and others. It is regrettable to say 
that most people are obtuse and stupid as a result of their own life-choices. Henrik 
Ibsen called this the 'life-lie.' We are doomed to repeat it forever.

> 🚀 **Quickstart available:** See [README-QUICKSTART.md](README-QUICKSTART.md) to build and run your own Grieg instance immediately.

> ⚖️ **Verification stance**  
> Grieg is a demonstrable substrate: phase outcomes (ALIVE, JAM, MEM, VAC) are reproducible at the CLI, with conformance sets and CI serving as live proofs.  
> See [docs/verification.md](docs/verification.md) for details.

# Grieg — Reasoning Engine (Scaffold v2)

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

