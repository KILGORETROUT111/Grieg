# Grieg — MML: Phase Dominance under ERP/IPC Context

This document accompanies the **grieg-sap** integration.  
It introduces a **Medium-level Math & Logic (MML)** layer: **Phase Dominance**.  
This layer is falsifiable, testable, and endpoint-agnostic (SAP, Oracle, or other ERP/IPC systems).

---

## 0. TL;DR Quickstart

From repo root:

```bash
cargo build -p grieg-cli
cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast
````

If Grieg is consistent, the output phases will **match the `expect_phase` field**.
If not, the dominance law has been falsified.

---

## 1. The Phase Dominance Law

We define a strict order over phases:

$$
\text{JAM} \succ \text{MEM} \succ \text{VAC} \succ \text{ALIVE}.
$$

1. **Determinism**
   For any input expression $e$, evaluation yields a unique phase $p(e)$.
   Formal: $\forall e, \exists! p(e).$

2. **Dominance Invariance**
   If multiple triggers apply, the **maximal phase under $\succ$** is chosen.
   Example:
````markdown
# Grieg — MML: Phase Dominance under ERP/IPC Context

This document accompanies the **grieg-sap** integration.  
It introduces a **Medium-level Math & Logic (MML)** layer: **Phase Dominance**.  
This layer is falsifiable, testable, and endpoint-agnostic (SAP, Oracle, or other ERP/IPC systems).

---

## 0. TL;DR Quickstart

From repo root:

```bash
cargo build -p grieg-cli
cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast
````

If Grieg is consistent, the output phases will **match the `expect_phase` field**.
If not, the dominance law has been falsified.

---

## 1. The Phase Dominance Law

We define a strict order over phases:

$$
\text{JAM} \succ \text{MEM} \succ \text{VAC} \succ \text{ALIVE}.
$$

1. **Determinism**
   For any input expression $e$, evaluation yields a unique phase $p(e)$.
   Formal: $\forall e, \exists! p(e).$

2. **Dominance Invariance**
   If multiple triggers apply, the **maximal phase under $\succ$** is chosen.
   Example:

$$
p(@mem(true) \to @jam(false)) = \text{JAM}.
$$

3. **ERP Semantics**

   * **Bound symbol** (e.g., `sap_ok`) → counts as a witness.
   * **Unbound symbol** (e.g., `sap_unbound`) → no witness → forces VAC.

   This ties Grieg’s dominance law to SAP’s data model of *bound vs unbound material*.

4. **IPC Events**
   On dominance switches, Grieg’s IPC layer must emit an event:

   * MEM transport → `Transport`
   * JAM dominance → `Boundary`
   * VAC→ALIVE → `Witness`

   These events are **observational only** (no effect on truth).
   IPC events are deterministic, parameter-free, and testable against traces.

---

## 2. Example JSONL Falsifiers

File: `conformance/sap-dominance.jsonl`

```jsonl
{"expr":"@mem(sap_ok) -> @jam(false)","mem":true,"expect_phase":"JAM","note":"JAM dominates MEM"}
{"expr":"sap_unbound -> @mem(true)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE; antecedent VAC irrelevant"}
{"expr":"sap_unbound","mem":false,"expect_phase":"VAC","note":"Unwitnessed SAP symbol → VAC"}
{"expr":"(true -> true) -> @mem(sap_cached)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE"}
```

Notes on constructing falsifiers:

* Use witnessed `@mem(true)` tokens to model cached/confirmed inputs.
* Use plain identifiers (e.g., `sap_unbound`) to model missing/unwitnessed data.
* Keep consequents bound when asserting dominance that should override antecedents.

---

## 3. Running the Test

```bash
cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast
```

Expected shape of output (phases must match `expect_phase`):

```json
{"input":"@mem(sap_ok) -> @jam(false)","phase":"JAM","ok":true,"expect_phase":"JAM"}
{"input":"sap_unbound -> @mem(true)","phase":"MEM","ok":true,"expect_phase":"MEM"}
{"input":"sap_unbound","phase":"VAC","ok":true,"expect_phase":"VAC"}
{"input":"(true -> true) -> @mem(sap_cached)","phase":"MEM","ok":true,"expect_phase":"MEM"}
```

If any `"ok": false` appears, the dominance law is falsified for that case.

---

## 4. Formal Testable Assertions (conformance checklist)

1. **Uniqueness** — Each input expression yields exactly one phase.

   * Test: run conformance set; assert no null/multiple-phase outputs.

2. **Dominance** — When two or more phase-triggering conditions apply, the maximal phase under $\succ$ is selected.

   * Test: construct pairs where lower-order triggers are present along with higher-order triggers; check expected.

3. **ERP Binding Semantics** — Bound vs unbound symbol semantics map to MEM/VAC as prescribed.

   * Test: replace `sap_ok` ↔ `sap_unbound` and observe phase changes.

4. **IPC Emission & Non-interference** — When IPC is enabled (event emission on transitions), emitted events do not alter the evaluation outcome.

   * Test: run with IPC enabled/disabled; assert identical `{value, phase}` results and presence/absence of event log.

5. **Sink Uniqueness** — For linear `->` chains, assert at most one Sink event per chain (if the engine emits Sink events).

   * Test: contrive chains that produce sinks and assert uniqueness.

---

## 5. Why This Matters

* **For researchers:** A falsifiable algebra of phases, enabling reproducible experiments and formal analysis.
* **For SAP vendors:** Clear mapping from ERP data-state (bound/unbound) to provable phase judgments.
* **For Government & Defense:** Auditable, deterministic semantics for critical edge decisions.
* **For Academia:** A medium-level formalism that balances rigor and accessibility.
* **For Cybersecurity & Cybernetics:** A substrate where JAM captures integrity/threat conditions and MEM captures forensic/stateful witness information.

---

## 6. Endpoint Agnosticism

The dominance law is **not SAP-specific**. Replace `sap_ok` with `oracle_ok`, `sensor_bound`, or any domain-specific witness term — the algebra and test methodology remain the same.

Future endpoint modules (`grieg-endpoints/README-ENDPOINTS.md`) will carry similar MML assertions for Oracle, MS Dynamics, OData APIs, and raw sensor networks.

---

## 7. Implementation Notes for Maintainers

* Add `conformance/sap-dominance.jsonl` to CI with `grieg-cli` JSONL mode and gate the CI job to fail if any `"ok": false`.
* Keep IPC event emission behind a feature flag initially; tests for non-interference should run with the feature both off and on.
* Document any evaluator changes that affect implication/phase propagation rules; update MML assertions if semantics change.

Suggested CI job (example):

```yaml
- name: MML: Phase Dominance Conformance
  run: cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast | jq -r '.ok' | grep -v true && exit 1 || true
```

---

## 8. Appendix — Minimal Expressions

* **Witnessed MEM:** `@mem(true)` → MEM
* **Unwitnessed VAC:** `sap_unbound` → VAC
* **JAM dominance:** `@jam(true) -> true` → JAM
* **MEM over ALIVE:** `(true -> true) -> @mem(true)` → MEM

Use these as building blocks in JSONL test files.

---

## 9. Maintainer Note

This file is intended to live at:

```
grieg-sap/README-MML.md
```

and to be used by researchers and integrators as the canonical MML specification for the Grieg SAP integration.
If you want, I can also create `conformance/sap-dominance.jsonl` in the repo with the exact lines from section 2 — tell me and I'll provide that file content in the same ready-to-paste form.

```


This document accompanies the **grieg-sap** integration.  
It introduces a **Medium-level Math & Logic (MML)** layer: **Phase Dominance**.  
This layer is falsifiable, testable, and endpoint-agnostic (SAP, Oracle, or other ERP/IPC systems).

---

