# Grieg — A Four-Fold Phase Semantics on an Ontic Logical Manifold
**PI:** William A. Patterson • **Repo:** KILGORETROUT111/Grieg • **Contact:** grieg@keemail.me • **Security:** psirt@keemail.me

## Problem
Classical logic hides evidence/witness; many-valued logics (Belnap/K3/LP) are phase-blind. We need a foundation that preserves classical truth **and** makes causal/operational structure explicit for counterfactuals and “licensed” inference (e.g., modus-ponens as an actual step, not just a truth-table artifact).

## Idea
**Grieg** evaluates truth with a concurrent **phase** over a four-fold basis **{ALIVE, JAM, MEM, VAC}** on an **ontic manifold** (factual torus + counterfactual fiber). Phase captures boundary/sink/transport invariants while truth stays classical where defined. A feature-gated **trace** witnesses invariants without changing results.

## Aims (Year 1)
1. **Theory.** Soundness/completeness for a sequent calculus; **conservativity** over CPL; NP/coNP bounds; **embedding + separation** vs Belnap–Dunn/K3/LP.
2. **Implementation.** Reference Rust engine + property tests; **trace adequacy** for invariants (sink on →, π rotation on ¬, etc.).
3. **Applications.** Counterfactual safety reasoning; licensed inference; pedagogy (observable witnesses).

## Novelty (audit targets)
- Operational, **history-sensitive** invariants (e.g., MP sinks) not expressible in BD/K3/LP.
- **Traced** semantics with observational adequacy (I1–I5).
- **Conservative** over CPL; **embeds** BD but not conversely (no compositional phase-erasing translation).

## Milestones
- M1: conservativity + soundness proofs (weeks 1–3)
- M2: trace adequacy + killer example (weeks 3–4)
- M3: arXiv v1 + talk deck + outreach to collaborators (week 4)

## Team/Budget (indicative)
1 postdoc (logic), 1 PhD, 0.5 FTE engineer, PI time, travel; **$350–400k** year-1 (host-dependent IDC).

## Artifacts
- Engine (MIT), docs (CC-BY), tests, examples, CLI flags: `--mem`, `--emit-geometry`, `--jsonl`, `--pretty`.