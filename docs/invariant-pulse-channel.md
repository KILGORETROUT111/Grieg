# Invariant Pulse Channel (basis5-derived)

This note describes the optional event stream that Grieg can emit. Events are **parameter-free** and follow directly from basis5’s four-fold phases and sheet structure; they **never** modify evaluation.

## Event definitions

- **Winding**  
  **Trigger:** cumulative angular advance on the factual sheet completes a full turn (Δθ crosses \(2\pi\cdot k\), \(k \in \mathbb{N}\)).  
  **Semantics:** topological circuit count. *(Requires geometry emission.)*

- **Sink**  
  **Trigger:** first entry into a modus-ponens fixed point along a → chain (I1 radial monotonicity).  
  **Semantics:** commitment step; absorbing for implication segments.

- **Boundary (JAM)**  
  **Trigger:** transition into `JAM`.  
  **Semantics:** manifold boundary / forbiddenness (short-circuit).

- **Transport (MEM)**  
  **Trigger:** sheet exchange \(F \leftrightarrow C\) under `@mem(·)`.  
  **Semantics:** moves evaluation between factual and counterfactual sheets without changing truth.

- **Witness**  
  **Trigger:** identifier resolves **VAC → ALIVE** (witness arrival).  
  **Semantics:** informational completion; removes “no-witness” status.

## Guarantees

- **Deterministic & parameter-free.**  
- **Non-interfering.** Purely observational; does not affect truth/phase.  
- **Testable.** Invariants like “≤1 Sink per → chain” are assertable.

> Engineering teams MAY build non-normative telemetry (e.g., a leaky-integrator over these events) as a separate layer. That layer is explicitly non-canonical.
