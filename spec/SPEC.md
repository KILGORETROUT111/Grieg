# Grieg Spec — Ontology, Semantics, Trace

> **Scope (v0.2 “Semantics Freeze”)**
> - **In-scope:** basis5 four-fold phases (ALIVE/JAM/MEM/VAC), implication sink, VAC witness behavior, MEM transport F↔C, CLI eval + REPL, JSONL batch, minimal AST print, property tests for dominance/implication.
> - **Optional (feature-gated, off by default):** geometry emission, invariant event emitter (winding/sink/boundary/transport/witness).
> - **Out of scope (v0.2):** solver heuristics, external KBs, network interfaces, GUI/visualizers, non-Rust bindings.
> - **Compatibility:** AST/CLI options may change between v0.2→v0.3; semantics are stable except explicit TODOs marked “SpecRef: TBD”.

## 1. Ontology and Coordinates (basis5 / four-fold)

Grieg treats the logical manifold as **ontic**. Evaluation enacts motion on this manifold; traces are operational witnesses of that motion.

- **Phase chart (intrinsic coordinates):** `ALIVE`, `JAM`, `MEM`, `VAC`.
- **Sheets:** factual sheet **F** (torus / “rest energy”) and counterfactual sheet **C** (orthogonal fiber).
- **Transport:** `MEM` transports between **F ↔ C`; `VAC` is projection into **C** when a witness is absent.

## 2. Torus-agnostic core semantics

Evaluation returns a classical value paired with an operational phase:

`⟦E⟧ = (v, φ)` where `v ∈ {true,false} ∪ {None}` and `φ ∈ {ALIVE,JAM,MEM,VAC}`.

Evaluation returns `(value, phase)` where:
- `value` is one of `true`, `false`, or `None` (no witness)
- `phase` is one of `ALIVE`, `JAM`, `MEM`, `VAC`

- **Values.** `None` = no witness; pairs with `VAC`.
- **Phases.** Operational modes; they do not alter classical truth on ground terms.

### Clauses (selected)
- **Negation** `¬A`: classical on values; phase updates operationally.
- **Disjunction** `A ∨ B`: classical on values; **JAM dominance** short-circuits if either branch is `JAM`.
- **Implication** `A → B`: desugar as `¬A ∨ B`; additionally mark a **sink** when MP fires (A true and B evaluated). Sinks are absorbing on right-nested chains.
- **Identifiers**: unbound `x` ⇒ `(None, VAC)`.
- **Phase ops**: `@mem(E)` preserves `v` but evaluates via MEM transport; `@vac(x)` yields `(None, VAC)`; `@jam(E)` marks boundary; `@alive(E)` forces an ALIVE channel for tests.

### Invariants
- **I1 Conservativity (F-sheet run):** If `E` has no free idents and no phase ops, `proj(v) ∈ {true,false}` equals classical truth.
- **I2 Sink monotonicity (→ chains):** once sink=true, all later `→` steps remain sink=true.
- **I3 JAM dominance (∨):** if either branch is `JAM`, the whole join is `JAM`.
- **I4 VAC discipline:** `v=None ⇔ φ=VAC`.
- **I5 Determinism:** with fixed left-to-right strategy, the phase trace is deterministic.

> Geometry (angles/“sheets”) is **optional** trace data. Enabling it must not change any truth result.


## 3. Semantic Clauses (truth & phase)

Let `⟦·⟧` be truth, `ϕ(·)` be phase.

- **Negation** `¬A`: truth as usual; phase rotates on **F** (π). Double negation composes to identity on **F**.
- **Disjunction** `A ∨ B`: truth as usual; phase is centrifugal on **F**, short-circuiting `JAM` if any branch marks boundary.
- **Implication** `A → B`: truth desugars as `¬A ∨ B`; **but** evaluation marks a fixed-point/sink when modus-ponens applies (A true, B evaluated). Sinks are absorbing for implication chains.
- **Identifiers:** without binding/witness, `ϕ = VAC`, `⟦·⟧ = ⊥/None` (no total boolean).
- **Phase operators:** `@mem(E)`, `@jam(E)`, `@vac(x)`, `@alive(E)` set/mark the phase channel and evaluate `E` accordingly.

## Observability (Optional)

### Invariant Event Emitter (optional, non-interfering)

When enabled, Grieg emits a stream of **invariant events** that are fully determined by the core basis5 semantics and do **not** alter evaluation:

- **Winding** — a complete angular circuit on the factual sheet (Δθ crosses \(2\pi\cdot k\); requires geometry emission).
- **Sink** — first entry into a modus-ponens fixed point along an implication (→) chain.
- **Boundary (JAM)** — transition into JAM (manifold boundary / forbiddenness).
- **Transport (MEM)** — sheet exchange \(F \leftrightarrow C\) under `@mem(·)`.
- **Witness** — identifier resolves **VAC → ALIVE** (witness arrival).

**Non-interference:** This emitter is a read-only projection of the evaluation trace; enabling it never changes truth values or phases. It exists for reproducible profiling, replay, and visualization and is disabled by default.


## 4. Invariants (geometry/phase)

- **I1 Sink (Implication):** along an implication chain, radial coordinate `ρ` is non-increasing; sinks are absorbing.
- **I2 Negation:** `¬¬A` preserves truth; total angular advance on **F** is `≈ 2π` (mod `2π`) when geometry is emitted.
- **I3 Disjunction:** under `ALIVE`, `ρ(A∨B) = max(ρ(A), ρ(B))`; `JAM` on either branch short-circuits and marks boundary.
- **I4 Transport (MEM):** transports between sheets **F ↔ C** preserving truth value; only phase/sheet changes.
- **I5 VAC:** `to_bool() == None ⇒ ϕ = VAC` (no witness).

## 5. Trace Schema (feature-gated, no behavioral change)

## Sheets (F:C) — Ontic Two-Sheet Toroidal Manifold

Each sheet is a **solid torus** Σ = S¹ × D² with angles (θ, φ) and radial potential ρ ∈ [0,1].
- **F** factual sheet (default); **C** counterfactual sheet (unwitnessed / what-if).

**Operators & motion**
- `@vac(x)`: projection to **C** (no witness) ⇒ boolean `None`, phase `VAC`.
- `@mem(E)`: transport **F ⇄ C** (truth-preserving); evaluates `E` on the opposite sheet with phase `MEM`, then returns to caller’s sheet.
- `@jam(E)`, `@alive(E)`: mark the phase channel; sheet unchanged.

**Toroidal dynamics (on F)**
- Negation: θ ← θ + π (mod 2π).  
- Conjunction: ρ(A ∧ B) = min(ρ(A), ρ(B)) (inward).  
- Disjunction: ρ(A ∨ B) = max(ρ(A), ρ(B)) (centrifugal); boundary ρ=1 marks `JAM`.  
- Implication: truth as ¬A ∨ B, with **modus-ponens sink** on F when antecedent holds; along implication chains ρ is **non-increasing**. Sinks are F-local (never on C).

**Engine convention (current build)**
- “On C” is reflected by `phase = VAC` with `value = null`.  
- “On F” has definite boolean with `phase ∈ {ALIVE, JAM, MEM}`. (Trace may later emit `sheet: F|C` without changing semantics.)


When `emit_geometry` is enabled, the engine MAY emit per-step records:

