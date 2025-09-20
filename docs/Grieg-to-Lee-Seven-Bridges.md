# LEE → Grieg: Seven Bridges

*A concise, copy-ready mapping from the original Logic Evaluation Engine (LEE) to the Grieg reasoning engine (basis5 / four-fold phase semantics).*

> **Thesis.** Grieg preserves the operational heart of LEE while making two things explicit:  
> (i) a **four-phase** evaluation channel (ALIVE, JAM, MEM, VAC) with a clear **dominance law**;  
> (ii) an **optional geometry emitter** (torus/flow traces) that never changes truth, only adds witness structure.

---

## Bridge 1 — Ontology & Naming

- **LEE**: logic engine with emergent winding/loops; “manifold” language used informally.  
- **Grieg**: explicit **ontic manifold** with **phase chart**  
  `ALIVE, JAM, MEM, VAC`, dominance `JAM > MEM > VAC > ALIVE`.  
  “Sheets” = factual (**F**) vs counterfactual (**C**); **MEM** transports between them; **VAC** = no witness.

**Why this matters:** Phases make edge conditions *first-class* (unknowns, boundary failures, cached witness, vacuity) instead of hiding them in booleans or exceptions.

---

## Bridge 2 — Truth & Phase Semantics (conservative over classical)

- Classical truth is **preserved**. Phase is **additional** structure.
- Key clauses (sketch):

  - `¬A`: truth as usual; phase rotates on **F**; `¬¬A` ~ identity on **F**.  
  - `A ∨ B`: truth as usual; short-circuit propagates **JAM** if any branch hits boundary.  
  - `A → B`: desugars to `¬A ∨ B`, **and** marks a **sink** when MP fires (A true, B evaluated).  
  - Identifiers with no witness → `phase = VAC`, `to_bool() = None`.

- Phase operators: `@mem(E)`, `@jam(E)`, `@vac(x)`, `@alive(E)` pin/transport phase during evaluation.

**Invariant highlights**

- I1 (Implication sink): along `→` chains, radial coordinate `ρ` is non-increasing; sinks are absorbing.  
- I3 (Disjunction): under ALIVE, `ρ(A ∨ B) = max(ρ(A), ρ(B))`; **JAM** short-circuits boundary.

---

## Bridge 3 — Optional Geometry (Torus), Always Truth-Invariant

- Geometry emission is **feature-gated**: `emit_geometry`.  
- When enabled, the engine *emits* structured **trace steps**; **truth results don’t change**.

```rust
// feature: emit_geometry
#[derive(Debug, Clone)]
pub struct TraceStep {
    pub op:   &'static str,      // "not", "or", "implies*", "@mem", ...
    pub pre:  Phase,             // ALIVE | JAM | MEM | VAC
    pub post: Phase,
    pub sink: bool,              // modus-ponens sink reached?
    pub theta: Option<f64>,      // optional angular delta (if modeled)
    pub rho:   Option<f64>,      // optional radial coord (if modeled)
}

> ** Why a torus?
> ** Empirically, LEE’s winding behavior produced toroidal structure without being pre-programmed. We keep torus as the default visual metaphor because it succinctly captures periodicity + locality and scales well to low/high-dimensional telemetry; when traces are off, evaluation is pure classical + phases.

⸻

Bridge 4 — Parser/AST (small & explicit)
	•	Grieg keeps a compact grammar; example:

expr  = implication ;
implication = disjunction ( "->" disjunction )* ;
disjunction = unary ( "|" unary )* ;
unary = "@" ident "(" expr ")"
      | "!" unary
      | "(" expr ")"
      | ident
      | "true" | "false" ;

	•	Phase operators are lexical (@mem, @jam, @vac, @alive).

⸻

Bridge 5 — Engine API (Evaluator) & CLI

Rust (library)

use grieg_engine::{eval::Evaluator, phase::Phase};
use grieg_parser::parse_expr;

let mut ev = Evaluator::new(/* mem_enabled = */ true);

// optional: persist/restore MEM for sessions
// ev.import_mem(my_hashmap);

let e = parse_expr("@mem(true -> false)")?;
let r = ev.eval(&e, None);
assert_eq!(r.value.to_bool(), Some(false));
assert_eq!(r.phase, Phase::MEM);

// optional save
// let mem = ev.export_mem();

CLI

# evaluate one expression (pretty mode)
grieg-cli --expr '@mem(true -> false)' --mem --pretty

# REPL
grieg-cli --repl --mem

# jsonl batch (see conformance/smoke.jsonl)
grieg-cli --jsonl conformance/smoke.jsonl --mem --pretty

# persist / load MEM map
grieg-cli --expr 'p' --mem --mem-db mem.json --pretty


⸻

Bridge 6 — Verification & Conformance
	•	Executable semantics: every run yields (value, phase); reviewers can see edge states.
	•	Smoke/conformance via JSONL:

{"expr":"true","want_phase":"ALIVE","want_bool":true}
{"expr":"false","want_phase":"ALIVE","want_bool":false}
{"expr":"@mem(true -> false)","want_phase":"MEM","want_bool":false}
{"expr":"p","want_phase":"VAC","want_bool":null}

Run:

cargo test
grieg-cli --jsonl conformance/smoke.jsonl --mem --pretty

	•	Property tests (proptest):
	•	double negation preserves truth;
	•	A ∨ B commutative on truth and monotone in ρ under ALIVE;
	•	implication sinks are absorbing.

⸻

Bridge 7 — Sustainability & Interfaces
	•	Language bindings (planned): Python first, then JS/Node.
	•	Adapters: CSV/JMESPath for data checks; event telemetry source (dive computer, lab instruments).
	•	Security: SECURITY.md, PSIRT contact psirt@keemail.me.
	•	IP: Code Apache-2.0; trademark “Grieg”; provisional patent note in spec/SPEC.md.

⸻

Quickstart (Workspace)

# build
cargo build

# run CLI
cargo run -p grieg-cli -- --expr '@mem(true | false)' --mem --pretty

Optional geometry (truth-invariant traces):

# grieg-engine/Cargo.toml
[features]
emit_geometry = []

# grieg-cli/Cargo.toml (enable from CLI crate)
grieg-engine = { path = "../grieg-engine", features = ["emit_geometry"] }


⸻

Notes for Reviewers
	•	Conservativity: With geometry disabled, Grieg behaves classically, exposing phases only.
	•	Why phases? Real systems need “witness present?”, “boundary hit?”, “vacuous pass?”. Phases make those questions typed and testable.
	•	Why torus (when on)? It compactly encodes cyclic progression and locality; and it matched LEE’s emergent dynamics. Geometry is observational, not required for truth.

⸻

Pointers
	•	Repo: https://github.com/KILGORETROUT111/Grieg
	•	Spec & Ledger: spec/SPEC.md, spec/LEDGER.md
	•	Whitepaper (PDF): docs/grieg-main.pdf
	•	Lineage: docs/lineage.md
	•	Security: SECURITY.md (PSIRT: psirt@keemail.me)

 