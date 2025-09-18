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

When geometry emission is enabled, we often render operational flow on a **torus** for three pragmatic reasons:
the
1. **Low ↔ high dimensional compression:** toroidal coordinates compactly encode cyclic/phase behavior while allowing layered metadata (angles, radii, sheet transitions). This helps map complex traces into a stable, analyzable shape.
2. **Empirical habit in complex systems:** toroidal manifolds (and torus‑like flows) repeatedly appear in the analysis of high‑dimensional telemetry; emphasizing a torus gives a **standard frame** for comparing runs and for attaching additional structure (e.g., phase rotations).
3. It is a **Theorem** (we sometimes refer to it as a **carrier-Theorem**. See **Grieg: Toroidal Phase Flow — (carrier) Theorem & Verification Pack** below

**Bottom line:** Gödel sets limits on internal proofs; Grieg responds by making **consistency demonstrable** through **operational witnesses** (phases, traces) that any reviewer can run, inspect, and try to break. If a future change violates the invariants, the conformance/property gates will surface it immediately.

---

# Grieg: Toroidal Phase Flow — (carrier) Theorem & Verification Pack

> **Scope.** This note states the toroidal attractor **Theorem** for Grieg’s phase semantics, gives a semantic proof sketch, and shows how to verify its machine‑checkable consequences in the current Rust codebase. 

---

## 1) Setting & Notation

Grieg evaluates expressions over classical truth *and* an operational phase channel:

- Phases: **ALIVE**, **JAM**, **MEM**, **VAC** with dominance `JAM > MEM > VAC > ALIVE`.
- Intuition:  
  - **ALIVE** = ordinary evaluation on the factual sheet.  
  - **JAM** = boundary/constraint hit (absorbing marker).  
  - **MEM** = transporter between sheets (witness/cached state).  
  - **VAC** = vacuity/no-witness (counterfactual projection).

The core connectives: `~` (neg), `&` (and), `|` (or), `->` (implies) and phase operators `@mem(E)`, `@jam(E)`, `@alive(E)`, `@vac(x)`.

Let the enabled geometry emitter attach per-step trace fields `theta` (angle) and `rho` (radial), interpreted as *bookkeeping* of evaluation flow; disabling geometry must not change truth/phase outcomes.

---

## 2) Theorem (Torus Attractor of Phase Flow)

**Theorem (informal).** For any finite Grieg expression `E` in the core calculus with the standard evaluation rules and the phase dominance `JAM > MEM > VAC > ALIVE`, the induced stepwise phase flow on the factual sheet `F` exhibits a **toroidal attractor** structure when geometry emission is enabled:

1. There exists an invariant set `T ⊆ F` such that repeated evaluation of subterms (with identical inputs and environment) yields phase traces whose projected angles `theta` are periodic modulo `2π`, and whose radii `rho` remain bounded and non‑increasing along any implication chain.
2. Negation contributes an angular advance close to `π` (mod `2π`) on `F`. Double negation composes to identity on phases (up to an integer multiple of full turns).
3. Disjunction respects a max‑radius policy (`rho(A ∨ B) = max{rho(A), rho(B)}` under ALIVE) and short‑circuits to **JAM** if any branch is jammed.
4. Implication has an **absorbing sink** behavior: when modus ponens applies (antecedent true and consequent evaluated), the radius on `F` is non‑increasing along the chain until a fixed point is reached; afterwards, further implication steps do not increase `rho`.
5. Transports: **MEM** moves evaluation between the factual sheet `F` and the counterfactual sheet `C` without changing truth; **VAC** projects to `C` when no witness is present.

Hence, with geometry enabled, the long‑run flow on `F` lies on a toroidal limit set (periodic angular component; bounded radial component with monotone‑nonincreasing segments), and transitions to/from `C` are mediated via **MEM/VAC** without altering classical truth.

> **Note.** The torus here is a topological characterization of the emitted trace *structure*; turning geometry off leaves semantics invariant.

---

## 3) Proof Sketch (Semantic)

We sketch the structure as a discrete dynamical system over
`S = Phase × Ctx`, where `Ctx` carries the minimal evaluator state (witness/memory and, when enabled, `(theta, rho)` accumulators).

1. **Boundedness.** Each connective is a *local* transformer; evaluation is finite (AST finite). The radial update rules are chosen monotone‑nonincreasing under `->` and max‑preserving under `|`, so `rho` is bounded below and cannot diverge.
2. **Periodicity.** Negation is an involution on truth. With geometry enabled, it contributes a fixed angular delta; double negation yields net angle ≡ `0 (mod 2π)`. Thus repeated appearance of negations in evaluation paths induces an angular cycle.
3. **Short‑circuit laws.** For disjunction and boundary conditions (`JAM`), the phase dominance ensures *absorbing markers* (no “escape” to less‑dominant phases). This stabilizes the trace and prevents radial inflation via short circuits.
4. **Implication sink.** `A -> B` desugars to `~A | B` for truth, but Grieg marks a *modus‑ponens sink* whenever `A` is true and `B` is evaluated. By construction, this sink does not increase `rho` and is idempotent across chains.
5. **Transports (MEM/VAC).** **MEM** preserves truth and moves sheets (`F ↔ C`). **VAC** corresponds to no witness/identifier (value=None), forcing `C`. These transports do not change the correctness of classical evaluation, only its *location* (sheet).

Together, (1)–(5) yield: bounded radial flow, periodic angle on `F` (hence a toroidal limit set), absorbing boundaries via dominance, and sheet transport that preserves truth. □

---

## 4) Machine‑Checkable Consequences

These statements should hold whether or not geometry is compiled in:

- **Dominance law:** if any sub‑evaluation returns **JAM**, the overall phase is **JAM**.
- **VAC law:** if an unbound identifier is encountered, phase is **VAC** and value is `None`/`null`.
- **MEM transport:** `@mem(E)` preserves truth value of `E` but changes the phase to **MEM** (and when geometry is on, toggles sheet bookkeeping).
- **Implication sink:** evaluating `A -> B` with `A` true causes downstream steps to never increase radial measure (when geometry is on), and truth equals `¬A ∨ B`.

**CLI sanity (already works):**
```bash
cargo run -p grieg-cli -- --expr '~false & (true | false)' --mem --ast
# {"value":true, "phase":"ALIVE"}

cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty
# Input: @mem(true -> false)
# Value: false
# Phase: MEM
```

---

## 5) Property Tests (Proptest) — Sketches

> `grieg-proptest/src/lib.rs` (or extend existing props) area already dropped in. These assume Grieg's current API where `Evaluator::new(mem_enabled: bool)` and `eval(&mut self, &Expr, Option<&mut ()>)` exist.

```rust
use grieg_engine::{eval::Evaluator, phase::Phase, value::V};
use grieg_parser::parse_expr;
use proptest::prelude::*;

proptest! {
    // JAM dominance over OR/AND
    #[test]
    fn jam_dominates(a in any::<bool>(), b in any::<bool>()) {
        let mut ev = Evaluator::new(true);
        // craft an expression that jams a branch; here we just mark jam on a true
        let e = parse_expr("@jam(true) | @alive(false)").unwrap();
        let r = ev.eval(&e, None);
        prop_assert_eq!(r.phase, Phase::JAM);
    }

    // VAC on unbound identifiers
    #[test]
    fn vac_on_unbound() {
        let mut ev = Evaluator::new(false);
        let e = parse_expr("@vac(x)").unwrap();
        let r = ev.eval(&e, None);
        prop_assert_eq!(r.phase, Phase::VAC);
        prop_assert!(matches!(r.value, V::None));
    }

    // Implication truth equals (~A | B)
    #[test]
    fn implication_truth_equiv(a in any::<bool>(), b in any::<bool>()) {
        let mut ev = Evaluator::new(false);
        let e1 = parse_expr(&format!("({} -> {})", a, b)).unwrap();
        let e2 = parse_expr(&format!("(~{} | {})", a, b)).unwrap();
        let r1 = ev.eval(&e1, None);
        let r2 = ev.eval(&e2, None);
        prop_assert_eq!(r1.value.to_bool(), r2.value.to_bool());
    }
}
```

If `emit_geometry` is ON and the engine emits steps like the JSON below, you can add monotonicity checks on `rho` inside implication chains.

```jsonc
// Example trace step (illustrative)
{
  "op": "implies",
  "pre":  "ALIVE",
  "post": "ALIVE",
  "sink": true,
  "theta": 3.1415,    // optional
  "rho":   0.42       // optional
}
```

---

## 6) Enabling the Geometry Feature (Optional)

In **`grieg-engine/Cargo.toml`**, declare the feature:
```toml
[features]
emit_geometry = []
```

In **`grieg-cli/Cargo.toml`**, enable it for the engine dependency (as needed):
```toml
[dependencies]
grieg-engine = { path = "../grieg-engine", features = ["emit_geometry"] }
```

Then build/run as usual. If a consumer doesn’t need geometry, omit the feature; truth/phase results remain identical.

> If you use `#[cfg(feature = "emit_geometry")]` in code, the warning
> “unexpected cfg condition value: `emit_geometry`” disappears once the
> feature is declared as above.

---

## 7) README Callout (Paste‑Ready)

```md
### Toroidal Phase Flow (theorem)
With geometry emission enabled, Grieg’s evaluation flow on the factual
sheet exhibits a **toroidal attractor**: angles cycle (mod 2π) while
radii remain bounded and non‑increasing along implication chains;
JAM is absorbing and MEM/VAC transport preserves truth across sheets.
Turn geometry off, and outcomes (truth/phase) are identical.

```

### Appendix: Minimal Trace Schema (Optional JSON)

```json
{
  "op":   "not | and | or | implies | @mem | @jam | @vac | @alive",
  "pre":  "ALIVE | JAM | MEM | VAC",
  "post": "ALIVE | JAM | MEM | VAC",
  "sink": true,
  "theta": 0.0,      // optional
  "rho":   0.0       // optional
}
```

> Replace names/fields to match your actual engine output; the theorem only needs the qualitative properties (periodic angle, bounded radius, dominance, transport).


---

## Addendum — Partial Autonomy and the Coexistence Interval  
⚠️ *Important: do not skip this section.*  

Grieg’s four phases (ALIVE, JAM, MEM, VAC) do not always collapse immediately.  
During evaluation, there exists an **interval of coexistence** where multiple phases  
are simultaneously present as variables, prior to resolution.  

- **Coexistence:** Distinct phases may be simultaneously valid (e.g. MEM transport  
  alongside VAC on an unwitnessed identifier).  
- **Autonomy:** These coexisting states are typed and ordered, yet not deterministic.  
- **Resolution:** Collapse occurs only at **transformational boundaries** (JAM entry,  
  MEM transport, VAC persistence, ALIVE default).  
- **Dominance law:** At resolution, the order JAM > MEM > VAC > ALIVE guarantees a  
  single outcome.  
- **Efficacy:**  
  - For researchers: coexistence exposes signal richness, aligned with  
    uncertainty/relativistic parallels.  
  - For enterprises: resolution ensures determinism at the interface,  
    avoiding ad-hoc exception paths.  

This addendum shows that Grieg models **two layers**:  
1. **Coexistence layer** — phases as parallel variables (informational,  
   uncertainty-aligned).  
2. **Resolution layer** — single deterministic outcome (operational,  
   system-aligned).  

Together, they define Grieg’s unique capacity to balance **uncertainty and rigor**  
in applied reasoning, across ERP, cybernetics, defense, and beyond.

