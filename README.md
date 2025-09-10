# Grieg — Reasoning Engine (Scaffold v2)
Four-fold phases: ALIVE, JAM, MEM, VAC. Dominance: JAM > MEM > VAC > ALIVE.
Crates: grieg-engine, grieg-parser, grieg-cli, grieg-proptest.
Build: cargo build
Run: cargo run -p grieg-cli -- --expr "@mem(true -> false)" --ast --mem

## IP & Trademarks
- Code: Apache-2.0 (see `LICENSE`).
- Patent notice: see `spec/SPEC.md` (Provisional details).
- Trademark: “Grieg” (see `TRADEMARKS.md`).
- Attribution: see `NOTICE`.


## Lineage & Rationale
- See **[LEE → Grieg Lineage](docs/lineage.md)** (informative).
- Spec: https://github.com/KILGORETROUT111/Grieg/blob/main/spec/SPEC.md
- Design ledger: `spec/LEDGER.md`

## Papers & Funding
- [Aims (2-page)](docs/aims.md)
- [Novelty Audit](docs/novelty-audit.md)
- [Main Paper LaTeX (LaTeX)](docs/grieg-main.tex)
- [Main Paper PDF (PDF)](docs/grieg-main.pdf)
- [Whitepaper](docs/Grieg-Whitepaper.md)
