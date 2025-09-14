# Verification and Provable Substrates

This note explains **what Grieg can guarantee**, **how reviewers can verify it locally**, and **why the engine’s phase geometry (when enabled) helps expose operational witnesses of consistency**.

---

## 1) Gödel and the boundaries of consistency

Gödel shows: any sufficiently strong formal system cannot (from within) prove its own consistency. In practice, robust systems **lean on an external structure**—traditionally **geometry**—to exhibit invariants that are observable and testable.

**Implication for Grieg:** we do not promise an internal “once-for-all” consistency proof. Instead, we provide **operational witnesses** (phases and traces) that a reviewer can run, observe, and falsify if broken. That’s how we make consistency *demonstrable* rather than merely asserted.

---

## 2) Geometry, autonomy, and uncertainty

Carrying axioms into **geometric form** yields partial autonomy:

- Distinct operational states can **coexist without collapsing** (no ad‑hoc error/exception paths).
- The system aligns with **uncertainty**: some states are **witness‑absent** (no total boolean), yet still **typed and ordered**.
- **Consistency is preserved through transformation**, not by static completeness claims.

In Grieg, the minimal geometric signal—**phase**—is exposed as:
`ALIVE`, `JAM`, `MEM`, `VAC`, with dominance `JAM > MEM > VAC > ALIVE`.
This is independent of any particular picture; when geometry emission is enabled, traces attach optional coordinates.

---

## 3) Relativistic domain and the role of time

If geometry and consistency alone sufficed, **time would be unnecessary**. In reality, scientific reasoning unfolds as **transformations across states**. Grieg treats evaluation as a **process**: stepwise traces and phase transitions are the primary observable. Time (as step order) is therefore **structural** to demonstrating invariants.

---

## 4) Parallel: Rust verification ↔ Grieg

- **Rust substrate:** we leverage the language’s memory safety and can layer **model checking** (e.g., Kani/ESBMC) against core library functions, with CI enforcing invariants.
- **Grieg semantics:** we expose **phase‑resolved evaluation** where consistency promises are **checkable**:
  - No silent collapse of edge conditions (they are phases, not ad‑hoc booleans).
  - **Dominance** and **short‑circuit** laws are explicit and testable.
  - Optional **trace geometry** annotates the same outcomes for downstream analysis.

---

## 5) What reviewers can run (now)

### A. CLI semantics (phase outcomes are observable)
```bash
# from the workspace root
cargo build

# Minimal: classical + phases (no geometry emission required)
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty
# Expect (values may format slightly differently):
# Input: @mem(true -> false)
# Value: false
# Phase: MEM
```

### B. Conformance runs (JSONL oracle)
We keep a simple **oracle file** of inputs and expected phases/values.

`conformance/smoke.jsonl` (example lines):
```jsonl
{"input":"true","value":true,"phase":"ALIVE"}
{"input":"false","value":false,"phase":"ALIVE"}
{"input":"true -> false","value":false,"phase":"ALIVE"}
{"input":"@mem(true -> false)","value":false,"phase":"MEM"}
{"input":"@jam(true & true)","value":true,"phase":"JAM"}
{"input":"@vac(x)","value":null,"phase":"VAC"}
{"input":"(x & true) -> y","value":null,"phase":"VAC"}
```

Run the batch:
```bash
cargo run -p grieg-cli -- --jsonl conformance/smoke.jsonl --mem
# Exit code 0 and no diffs => pass
```

### C. Property tests (dominance & tables)
```bash
# Fast logical properties (no geometry needed)
cargo test -p grieg-proptest
# Example properties include:
# - jam_dominance_in_join
# - implication_truth_table
# - vac_for_free_ident
# - mem_recovery_simple
```

### D. Optional geometry traces
If your build exposes the `emit_geometry` feature, Grieg **adds trace records** without changing outcomes:

```bash
# Build engine with geometry feature (example; may vary by workspace setup)
cargo build -p grieg-engine -F emit_geometry

# Then run your CLI as usual (phases/values invariant; traces become available
# to downstream consumers if your local CLI build enables the dependency feature).
cargo run -p grieg-cli -- --expr '~false & (true | false)' --mem --ast
```

> **Invariant:** With `emit_geometry` **off**, outcomes are baseline. With it **on**, outcomes are the *same*, and **extra trace data** (e.g., step transitions) is available to tools/visualizers.

---

## 6) What counts as a “pass”

- **Truth/phase oracle holds** across the conformance JSONL.
- **Dominance & short‑circuit properties** hold (JAM dominates; VAC for witness‑absent; MEM transports).
- **Geometry optionality:** enabling geometry **does not change** values/phases, only the emitted trace.

---

## 7) CI gates you can adopt

- **Conformance gate:** runs the JSONL batch; fails on any mismatch.
- **Property gate:** runs `grieg-proptest` properties.
- **Security & hygiene:** `cargo fmt`, `cargo clippy -- -D warnings`, dependency audit (e.g., `cargo deny`).
- **Optional:** model checking harness on small, pure functions.

These can all be replicated locally by any reviewer.

---

## 8) Reviewer checklist

1. Build and run the CLI example above; confirm phases.
2. Run the **conformance JSONL** and verify no diffs.
3. Run **property tests**; confirm dominance laws.
4. (Optional) Enable `emit_geometry` and confirm outcomes unchanged.
5. Inspect the code paths for phase handling (look for explicit `JAM`, `MEM`, `VAC`, `ALIVE` cases).

If any step fails, file an issue with the exact CLI command and output.

---

## 9) Why we emphasize the **torus** when geometry is on

When geometry emission is enabled, we often render operational flow on a **torus** for two pragmatic reasons:

1. **Low ↔ high dimensional compression:** toroidal coordinates compactly encode cyclic/phase behavior while allowing layered metadata (angles, radii, sheet transitions). This helps map complex traces into a stable, analyzable shape.
2. **Empirical habit in complex systems:** toroidal manifolds (and torus‑like flows) repeatedly appear in the analysis of high‑dimensional telemetry; emphasizing a torus gives a **standard frame** for comparing runs and for attaching additional structure (e.g., phase rotations).

> Importantly, the **torus is optional**: it’s a *rendering* of the same operational truths. Turn geometry off, and you still get the full, verifiable semantics via phases and values. Turn it on, and you gain a compact geometric “carrier” for deeper analysis without changing the results.

---

**Bottom line:** Gödel sets limits on internal proofs; Grieg responds by making **consistency demonstrable** through **operational witnesses** (phases, traces) that any reviewer can run, inspect, and try to break. If a future change violates the invariants, the conformance/property gates will surface it immediately.
