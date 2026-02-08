# sink.md

## Definition: Terminal Absorbing State

In Grieg, a **terminal absorbing state** is a state of evaluation from which no further logical transformation produces new information.

A state is terminal and absorbing when:

- All admissible transformations (paraphrase, assumption exposure, counterfactual modification) have been applied.
- Further transformations either:
  - reproduce already-evaluated states, or
  - introduce no additional contradiction, divergence, or instability.
- The system’s evaluative measures converge within defined tolerance bounds.

Once reached, the state is **absorbing**: subsequent inputs mapped to the same equivalence class collapse into this state without altering its metrics.

The terminal absorbing state does **not** assert truth.
It asserts **evaluative exhaustion**.

---

## Purpose

The concept exists to prevent infinite linguistic drift and to give Grieg a principled stopping condition that is:

- deterministic,
- auditable,
- independent of model confidence or narrative plausibility.

A terminal absorbing state marks the point at which further linguistic variation no longer changes the evaluative outcome.

---

## To-Do List

### Conceptual
- Formalize criteria for evaluative convergence.
- Define equivalence classes over paraphrase and counterfactual space.
- Specify tolerance thresholds for “no new information.”

### Architectural
- Identify where terminal-state detection lives (core vs analytic).
- Decide whether terminal states are cached or recomputed.
- Define how terminal states interact with contradiction tracking.

### Algorithmic
- Implement detection of state recurrence under transformation.
- Penalize oscillatory but non-convergent transformations.
- Distinguish between local minima and true absorbing states.

### Interface
- Expose terminal-state status to downstream modules.
- Decide whether terminal absorption is visible to users or internal only.
- Log terminal transitions for audit and replay.

### Open Questions
- Can multiple terminal absorbing states exist for a single initial claim?
- Under what conditions should absorption be forced vs deferred?
- How should probabilistic linguistic expansion affect convergence guarantees?

---

## Non-Goals

- Determining factual truth.
- Declaring correctness.
- Resolving ambiguity beyond evaluative exhaustion.

The terminal absorbing state is a stopping condition, not a verdict.