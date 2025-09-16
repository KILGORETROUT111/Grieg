**Lineage:** Operator discipline traces to Julian C. Boyd’s Berkeley program
(modality, deontic logic, syntax–semantics), with modern contrasts to context-
sensitive semantics and probabilistic / operational views. Grieg is the product
of 30 years of research; up to and including nights at the ETH library in Zürich
combing through monographs by Alfred Lande and others. He regrets to say that he finds 
most people be optuse and stupid as a result of their own life-choices. Henrik Ibsen 
called this the 'life-lie.' 

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

William A. Patterson is the steward of Grieg, a four-fold logic engine 
designed for dependable reuse in scientific software. He is an SAP® systems 
architect with deep ABAP Dictionary expertise who routinely authors both functional 
and technical RICEFWs and partners directly with ABAP teams. Patterson is comfortable 
debugging across application, integration, and Basis layers (IDoc, OData/CDS, PI/PO, SLT), 
and he designs interfaces with operational auditability and provenance in mind—the same 
discipline visible in Grieg’s phase semantics and trace outputs. A UC Berkeley alumnus 
who studied with Professor Emeritus Julian C. Boyd, Patterson focuses on turning formal 
ideas into deployable, maintainable infrastructure. His role on Grieg is hands-on: specs, 
tests, bindings, and integration playbooks that reduce adoption friction for research groups.

As an SAP systems architect he has lived and worked in ~ 17 countries on three continents.

He is an experienced and PADI licensed (professional) Tec 40, Nitrox, and Rescue Diver. 