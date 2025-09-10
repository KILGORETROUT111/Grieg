---
title: "Grieg — A Basis5 / Four-Fold Logic Engine on an Ontic Logical Manifold"
subtitle: "From LEE to Grieg: Spec, Semantics, and Operational Geometry"
version: "v0.1.0"
date: "2025-09-09"
authors:
  - William Alexander Patterson (editor & steward)
affiliations:
  - Grieg Project
contact:
  general: grieg@keemail.me
  security: psirt@keemail.me
repo: https://github.com/KILGORETROUT111/Grieg
license: MIT (engine) + CC-BY 4.0 (docs)
---

## 0. Executive Summary

Grieg is a **deterministic logic engine** that executes inference as **motion on an ontic logical manifold**. It inherits core ideas from the Logic Evaluation Engine (LEE) and formalizes them in **basis5 / four-fold** phase semantics: `ALIVE`, `JAM`, `MEM`, `VAC`. Truth evaluation is standard where defined; **phase** captures geometric/causal structure (torus for factual space, hyperbolic venturi for boundary/counterfactual flow). A feature-gated **geometry trace** witnesses this structure without changing results.

**Key contributions**
1. **Ontic manifold semantics** for logic (geometry is primary, visualization is derivative).
2. **Phaseful operators** (`@mem`, `@jam`, `@vac`, `@alive`) that expose transport, boundary, and witness behavior.
3. A compact **Rust** implementation (parser/engine/CLI) with invariants and tests that enforce the geometry.

---

## 1. Motivation & Lineage

- **LEE → Grieg.** LEE framed inference as **covariant flow** on a manifold (toroidal expansion / venturi pinch). Grieg preserves the ontic stance (geometry is real), cleans up the language, and hardens the engine.
- **Why phases?** Classical booleans hide causal structure. Basis5 phases provide operational coordinates: what is proven (`ALIVE`), what is at a caustic/boundary (`JAM`), what transports across factual/counterfactual sheets (`MEM`), and where witnesses are absent (`VAC`).

---

## 2. Ontology (basis5 / four-fold)

- **Phases (intrinsic coordinates):** `ALIVE`, `JAM`, `MEM`, `VAC`.
- **Sheets:** factual sheet **F** (torus / rest energy) and counterfactual sheet **C** (orthogonal fiber).
- **Transport:** `MEM` transports **F ↔ C**; `VAC` projects into **C** when a witness is absent.

Grieg evaluates **truth** and **phase** jointly. Truth behaves like classical logic where defined; phase encodes the manifold location and transitions.

---

## 3. Semantics (truth & phase)

Let `⟦·⟧` denote truth (bool/None), `ϕ(·)` phase.

- **Negation** `¬A`: truth as usual; on **F**, rotation by π (double negation ≡ identity on **F**).
- **Disjunction** `A ∨ B`: truth as usual with short-circuiting; on **F**, centrifugal advance; a `JAM` on any branch marks boundary.
- **Implication** `A → B`: truth via `¬A ∨ B`; additionally, **modus-ponens** marks a **sink** (fixed point) along the flow (absorbing for chains).
- **Identifiers**: unbound identifiers yield `ϕ = VAC`, `⟦·⟧ = None`.
- **Phase ops**:
  - `@mem(E)`: evaluate with **MEM** transport (preserves truth; changes sheet/phase).
  - `@jam(E)`: mark boundary/caustic (truth unchanged; phase becomes/retains `JAM`).
  - `@vac(x)`: project variable `x` into `VAC` (no witness).
  - `@alive(E)`: force evaluation on `ALIVE` channel (useful for tests/spec).

---

## 4. Invariants (geometry/phase)

- **I1 Sink (Implication):** along implies-chains, radial coordinate `ρ` is non-increasing; sinks are absorbing.
- **I2 Negation:** `¬¬A` preserves truth; cumulative `θ ≈ 2π (mod 2π)` on **F** when geometry is emitted.
- **I3 Disjunction:** under `ALIVE`, `ρ(A∨B) = max(ρ(A), ρ(B))`; `JAM` short-circuits with boundary mark.
- **I4 Transport (MEM):** `@mem(E)` preserves truth, alters sheet/phase.
- **I5 VAC:** `to_bool() == None ⇒ ϕ = VAC` (no witness).

---

## 5. Trace (observational, feature-gated)

When `--emit-geometry` is enabled, Grieg **may** emit per-step trace:

```text
TraceStep {
  op:    "not" | "and" | "or" | "implies*" | "@mem" | "@jam" | "@vac" | "@alive",
  pre:   Phase,     post: Phase,
  sink:  bool,      jam:  bool,
  theta: Option<f32>,  // radians
  rho:   Option<f32>   // arbitrary units
}
