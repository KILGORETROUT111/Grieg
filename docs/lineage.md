---
title: "LEE → Grieg Lineage"
description: "Key ideas carried forward from LEE and how Grieg encodes them."
grieg_version: "0.2.0"
status: "informative"
last_updated: "2025-09-07"
---

## Berkeley line: Julian C. Boyd (logic of modality & syntax–semantics)

Grieg’s operator discipline descends from **Julian C. Boyd** (UC Berkeley), a
logician and “philosophical grammarian” who worked at the seam of **modal logic,
deontic reasoning, and the syntax–semantics interface**. Beyond classroom
sketches of the deontic square, Boyd treated modality as a *layered* phenomenon:
(1) surface operators and their compositional types, (2) speech-act force and
illocutionary profile, and (3) background **norm systems**. That multi-level
analysis directly informs Grieg’s four-phase operational semantics.

Boyd was also a rare expert reader of **Chomsky’s _Syntactic Structures_**
and adjacent program; his diagnostics for English modals (e.g., *must, may,
ought*) and his editorial work in speech-act theory shaped a generation’s
standards for sentence-level logical analysis. We acknowledge this lineage
explicitly: Grieg treats operators as *logical primitives tied to use*, and
lets geometry/phase be internal to the evaluation engine rather than imported
from physics metaphors.

# LEE → Grieg Lineage

This document captures the *canonical lineage* from LEE to Grieg and maps
the load-bearing concepts into Grieg’s specification and code surface.
It is **informative** (non-normative) unless a section is explicitly
marked “Normative”.

- **Spec home:** `spec/SPEC.md`
- **Design ledger:** `spec/LEDGER.md`
- **Conformance:** `conformance/` (when published)

---

## 1) Phase System (four-fold) — *Normative in Grieg*

### Phases
- **ALIVE** — normal evaluation
- **JAM** — contradiction / conflict detected
- **MEM** — persisted knowledge / recall
- **VAC** — unbound / undefined

### Canonical transitions (matrix)
Allowed edges that Grieg must preserve:


> The evaluator enforces this transition set; additional internal arcs are
> permitted only if they reduce to the above at the API boundary.

---

## 2) Covariant Geometry (toroidal intuition)

LEE described inference as motion on a manifold with toroidal expansion
and venturi-like collapse. Grieg treats this as **trace semantics**:
the engine records phase flow and (optionally) rotation metadata without
coupling core truth evaluation to any specific geometry. Visualizers may
render torus/venturi pictures from trace data.

- **Engine requirement:** emit structured phase transitions per step.
- **Optional:** attach `(θ₁, θ₂)` rotation deltas under a feature flag.

---

## 3) Conjugate Quantities (operational pairs)

Certain behaviors act as **conjugates** (cannot be maximized together,
but transform coherently):

- **ALIVE ↔ MEM** (active reasoning ↔ evidence recall)
- **Rule application ↔ inductive recall**
- **Strictness ↔ coverage** in pattern resolution

Grieg’s rule: transformations between conjugates must be **structure-preserving**
(covariant) at the AST boundary.

---

## 4) Phase Operators — *Surface Syntax (Normative)*

Operators exposed at the language level:

- `@mem(expr)` — evaluate with MEM semantics (recall/persist)
- `@jam(expr)` — force/mark a JAM context (diagnostic)
- `@alive(expr)` — force ALIVE evaluation
- `@vac(x)` — mark identifier/value as VAC

> These are stable, user-facing affordances. The evaluator maps them to the
> phase transition set above.

---

## 5) Diagnostic Torsion Map (DTM) — *Roadmap*

A scalar/field τ(x) that summarizes “winding”/tension in a sub-expression
or over a trace window. In Grieg this is an **optional** diagnostic:
compute τ from depth, contradiction density, or rotation rate; expose via
trace events or `--torsion` flag. No effect on truth/phase.

---

## 6) Contradiction Ring Buffer (CRB) — *Roadmap*

Bounded buffer that cycles contradictions for potential later resolution:


Implementation notes:
- Small queue with age/attempt counters
- Off by default; `--crb` enables
- Print/inspect with `--crb-dump`

---

## 7) Fixed Points & Cycles — *Roadmap*

When the evaluator returns to an equivalent state (AST + phase + env),
enter **fixed-point mode**:

- either emit a MEM summary and stop, or
- cut the loop with a JAM.

This guard is optional but recommended for total strategies.

---

## 8) Counterfactual / Nomic Regions — *Roadmap (feature-gated)*

Add region tags to AST nodes and a lightweight distance Δ between regions
to support counterfactual queries. Keep core four-fold semantics unchanged.

---

## 9) Basis5 Alignment (Design Rationale)

The four-fold phases are the operational layer matching **basis5**’s logic
geometry. Grieg encodes the four-fold directly in the evaluator; toroidal/
venturi pictures remain external *views* derived from traces.

---

## 10) Design Guarantees

| Guarantee                                | What it means                                                          | Where enforced                       |
|---|---|---|
| Four-fold phases are first-class         | ALIVE/JAM/MEM/VAC in the engine, not just docs                         | `grieg-engine` evaluator API         |
| Canonical transition set only            | No hidden phase edges at API boundary                                  | evaluator state machine              |
| Stable phase operators                   | `@mem/@jam/@alive/@vac` remain and map to the same semantics           | parser + evaluator mapping           |
| Traces are structured                    | Stepwise phase/value events for external tools                         | evaluator trace hooks                |
| Diagnostics don’t affect truth           | DTM/CRB/fixed-point are observability features only                    | feature flags (`--torsion`, `--crb`) |
| Conformance decides “Compatible with Grieg™” | Must pass public tests, not claim-by-analogy                          | `conformance/` (normative tests)     |

---

## 11) Implementation Mapping (Now vs Roadmap)

**Already implemented**
- Four phases + evaluator results
- Phase operators (`@mem/@jam/@alive/@vac`)
- JSONL batch mode, REPL, `--pretty`
- Optional `--mem-db` persistence

**Next (incremental)**
- Structured trace events (phase/value per step)
- Feature flags: `--torsion`, `--crb`, `--fixed-point`
- Minimal counterfactual region tagging behind `--counterfactual`

---

## 12) Versioning & Spec Pointers

- This document is informative; normative rules live in `spec/SPEC.md`.
- All lineage-driven changes must be reflected in `spec/LEDGER.md`.
- Claims of compatibility rely on the conformance suite when published.

## Engine Tests (unit & property)

**Unit tests**
1. **Implication sink:** `((p -> q) & p)` marks at least one step with `sink=true`; final phase `ALIVE`, truth per table.
2. **Double negation:** `~~A` preserves truth; with geometry on, cumulative `theta ≈ 2π (mod 2π)`.
3. **Disjunction centrifugal:** with geometry on, `rho(A ∨ B) ≥ max(rho(A), rho(B))`; equals max when both `ALIVE`.
4. **MEM transport:** `@mem(E)` preserves truth; phase toggles consistently between sheets; `VAC -> MEM` on witness.
5. **VAC projection:** free identifier → `to_bool()==None` and `phase==VAC`.

**Property tests (proptest)**
- **P1 (DN-Id):** for arbitrary boolean `A`, `⟦~~A⟧ == ⟦A⟧`.  
- **P2 (Imp-Sink Monotone):** random implication chains produce non-increasing `rho` (if emitted) and mark `sink` where MP triggers.  
- **P3 (Or-Max):** for random boolean forms `A,B`, if both evaluate in `ALIVE`, then `rho(A∨B) == max(rho(A), rho(B))` (if emitted).  
- **P4 (Mem-Truth):** for random `E`, `⟦@mem(E)⟧ == ⟦E⟧` while phases reflect sheet transport.  
- **P5 (Vac-None):** any free identifier yields `to_bool()==None` and `phase==VAC`.

These tests codify the ontic-manifold stance: geometry is primary; traces witness it without mutating semantics.
