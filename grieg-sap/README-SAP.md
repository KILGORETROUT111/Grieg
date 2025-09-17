# Grieg × SAP Integration Pack (v1)
**Build:** 2025-09-14 04:50:34Z

This bundle lets SAP systems **call** the Grieg engine (outbound) and/or **expose** an SAP-hosted proxy endpoint (inbound) that forwards requests to a Grieg service (CLI/HTTP). It also includes a **CPI iFlow** normalizer, docs for Basis & ABAP, and a **Postman** collection.

## Contents
- `abap/outbound_client/ZCL_GRIEG_HTTP_CLIENT.abap` — ABAP class for outbound HTTPS calls to Grieg
- `abap/icf_inbound/ZCL_GRIEG_ICF_HANDLER.abap` — ICF HTTP handler (IF_HTTP_EXTENSION) to proxy requests to Grieg
- `abap/demo/ZGRIEG_DEMO.abap` — tiny SE38 demo program
- `cpi/Normalizer.groovy` — Groovy script for CPI iFlow (JSON normalization + header mapping)
- `docs/basis/BASIS_INSTALL.md` — Basis playbook (SICF, STRUST, destinations, TLS)
- `docs/user/EXEC_SUMMARY.md` — Executive summary for stakeholders
- `docs/user/POWER_USER_GUIDE.md` — Power-user instructions
- `postman/Grieg.postman_collection.json` — Postman collection
- `sidecar/README.md` — Run Grieg next to SAP or CPI (Docker), incl. example env
- `security/SECURITY_CONTACT.md` — PSIRT contact & disclosure

## Quick Start
1. **Outbound from SAP** (recommended): Import `ZCL_GRIEG_HTTP_CLIENT.abap`, set SM59 destination or maintain URL, call method `EVALUATE_JSON` or `EVALUATE_EXPR`.
2. **Inbound ICF proxy**: Create SICF service, bind class `ZCL_GRIEG_ICF_HANDLER`, set target Grieg URL in a parameter or table.
3. **CPI iFlow**: Add a Groovy step with `Normalizer.groovy`, route to Grieg HTTP receiver.
4. **Sidecar**: Start Grieg CLI/HTTP in Docker; point SAP/CPI to it.

See `docs` for full details.

````markdown
# Grieg — SAP Integration Pack v1

Grieg is a **phase-aware reasoning engine** (Rust) with four phases — **JAM, MEM, VAC, ALIVE** — and strict dominance (**JAM > MEM > VAC > ALIVE**).  
This pack connects Grieg’s evaluator to SAP-aligned workflows at the **edge** (factory cells, retail stores, clinics, field/defense nodes), using **JSON/JSONL** as a first-class I/O surface and `grieg-ipc` for integration plumbing.

- **Engine crates:** `grieg-engine`, `grieg-parser`, `grieg-cli`, `grieg-proptest`  
- **Integration crates:** `grieg-ipc/grieg-ipc-cli`, `grieg-ipc/grieg-telemetry`  
- **Status:** mock smoke is **green**; adapter path for SQL/RFC/ODATA is defined; production wiring is gated on credentials and sandbox access.

---

## 0) TL;DR Quickstart (no SAP backend yet)

```bash
# From repo root
cargo build -p grieg-cli

# Run the SAP mock smoke (JSONL; compares expected phases → ok:true/false)
cargo run -p grieg-cli -- --jsonl conformance/sap-smoke.jsonl --mem --ast
````

**Expected output (example):**

```json
{"input":"@mem(sap_ok)","phase":"MEM","ok":true,"expect_phase":"MEM"}
{"input":"sap_unbound","phase":"VAC","ok":true,"expect_phase":"VAC"}
{"input":"@jam(true) -> true","phase":"JAM","ok":true,"expect_phase":"JAM"}
{"input":"(true -> true) -> @mem(true)","phase":"MEM","ok":true,"expect_phase":"MEM"}
```

**Why this matters:** the smoke proves **wiring & semantics** end-to-end (JSONL → parser → evaluator → JSON out) without requiring an SAP system.

---

## 1) Installation & Layout

```bash
# Build everything
cargo build --workspace

# CLI help
cargo run -p grieg-cli -- --help
# (Adapters surface in grieg-ipc-cli as we enable them)
cargo run -p grieg-ipc-cli -- --help || true
```

**Where things live:**

* `grieg-cli/` – command-line evaluator with **JSONL mode** (`--jsonl <file>`), `--mem`, `--ast`, `--pretty`.
* `conformance/` – canonical JSONL suites (smoke, phase dominance, **sap-smoke**).
* `grieg-ipc/grieg-telemetry/` – JSON/chrono telemetry types (serde); feeds analytics/visualizations.
* `grieg-ipc/grieg-ipc-cli/` – CLI veneer for integrations; where `--sql`/`--rfc`/`--odata` adapter flags will land.

Optional Python binding (researcher convenience):

```bash
python3 -m venv .venv && source .venv/bin/activate
pip install -U pip maturin
maturin develop -m grieg-py/pyproject.toml
python - <<'PY'
import grieg
print(grieg.expr("@mem(true)", mem=True))
PY
```

---

## 2) Running the SAP Mock Suite

`conformance/sap-smoke.jsonl` (included):

```jsonl
{"expr":"@mem(sap_ok)","mem":true,"expect_phase":"MEM","note":"IPC layer reachable (mock witness)"}
{"expr":"sap_unbound","mem":false,"expect_phase":"VAC","note":"Unwitnessed SAP symbol → VAC"}
{"expr":"@jam(true) -> true","mem":false,"expect_phase":"JAM","note":"JAM dominates over ALIVE"}
{"expr":"(true -> true) -> @mem(true)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE when witnessed"}
```

Run:

```bash
cargo run -p grieg-cli -- --jsonl conformance/sap-smoke.jsonl --mem --ast
```

**Interpretation:**

* `@mem(sap_ok)` models a **witnessed** SAP signal (e.g., prior cached result) → MEM.
* `sap_unbound` with no witness → VAC (honest null; no silent assumption).
* `@jam(true) -> true` shows **JAM dominance** over ALIVE.
* `(true -> true) -> @mem(true)` shows **MEM dominance** when the witness is present.

> Prefer JSONL for conformance. For ad-hoc single expressions, use `--expr '<EXPR>'`.

---

## 3) Architecture (edge-first)

**Data flow (mock → real):**

1. **Ingestion** at the edge (factory/retail/clinic/field): events from SAP (IDocs, BAPIs, CDS/ODATA), sensors, local apps.
2. **Normalization**: adapt to **JSON/JSONL** (first-class) + typed structs (`grieg-telemetry`).
3. **Evaluation**: Grieg expressions compute a **value** and **phase** (ALIVE/JAM/MEM/VAC).
4. **Emission**: JSON results and optional traces feed notebooks, dashboards, tensors/heatmaps, or onward to SAP.

**Why phases matter**

* **JAM**: contradiction/boundary → stops bad cascades; dominates to keep systems safe.
* **MEM**: transport/cached witness → avoids recomputation; preserves provenance.
* **VAC**: no witness → honest “unknown”; prevents false certainty.
* **ALIVE**: ordinary truth-functional flow.

*Engine stays geometry-agnostic; any geometry/heatmaps are derived from traces.*

---

## 4) Adapter Surface (roadmap)

We expose **adapter flags** (in `grieg-ipc-cli`) to lift domain inputs into valid Grieg expressions:

* `--sql "<SELECT MATNR FROM MARA ...>"` → `sap_select(MARA, MATNR, ...)`
* `--rfc "<BAPI_GOODSMVT_CREATE {...}>”`  → `sap_rfc(GOODSMVT_CREATE, {...})`
* `--odata "</sap/opu/odata/...>"`        → `sap_odata(…query…)`

**Mock mode** maps these to witnessed/unwitnessed symbols to exercise phases.
**Real mode** binds credentials & executes via RFC/ODATA connectors, emitting JSON results + phases.

---

## 5) Guidance by Audience

### A) Researchers / Data Scientists

* Start with **JSONL** suites: `conformance/smoke.jsonl`, `conformance/phase-dominance.jsonl`, `conformance/sap-smoke.jsonl`.
* Use the Python binding for quick experiments; log inputs/outputs (JSON) for notebooks and BI.
* Treat **phase** as a dimension: aggregate counts, build **phase tensors** (JAM heatmaps), correlate with metadata (line/shop/ward/unit).
* Reproducibility: pin engine version; archive JSONL inputs/outputs.

### B) SAP Vendors / System Integrators

* Edge nodes: deploy `grieg-cli` / `grieg-ipc-cli` near equipment/storefront for **local** inference and guardrails.
* Prototype with **mock adapters**; swap to **real** RFC/ODATA connectors later.
* Use **MEM** to carry confirmed facts (stock positions, quality checks); treat contradictions as **JAM** events for alerting.
* Emit JSON into ELK/Grafana; phase becomes an alerting dimension.

### C) SAP (the company)

* Grieg is a **provable substrate** for edge reasoning that **does not modify** SAP core.
* Integrates via **standard surfaces** (RFC/ODATA/CDS, IDocs), producing **typed JSON** with **phase semantics** suitable for governance.
* Brownfield-friendly: carve-outs per module (MM/SD/PP/PM/IS-H/IS-DFPS), graduate to managed adapters.
* Coexistence: geometry/traces optional; engine behavior remains **geometry-agnostic**.

### D) Government / Defense / Regulated

* **Deterministic JSON I/O** and **phase-aware** evaluation yield transparent decision artifacts.
* **Disconnected/denied** environments: run at the edge, buffer JSONL; sync upstream when links restore.
* **Compliance**: PSIRT contact present; strict dominance rules avoid silent failures.
* **Auditability**: keep JSON inputs, outputs, and (optional) traces; replay is trivial.

### E) Cybersecurity

* Treat **JAM** as **threat dominance** (conflicts, integrity failures, malicious inputs).
* **MEM** as forensic cache; **VAC** as unknown/untrusted; **ALIVE** as normal ops.
* Ingest IDS/IPS logs, auth events, threat intel feeds; emit JSON decisions with phases for SIEM/SOAR.

### F) Cybernetics

* Use phases as **control states** in feedback loops:

  * JAM → halt/escalate,
  * MEM → reuse verified witness/state,
  * VAC → seek acquisition/measurement,
  * ALIVE → proceed nominally.
* Apply to robotics, manufacturing cells, autonomous platforms.

---

## 6) Security & Governance

* Engine is small, auditable, and does not alter SAP code; adapters wrap at the edge.
* **JAM dominance** prevents propagating contradictions.
* **MEM** captures explicit provenance instead of implicit caching.
* Security contact (PSIRT) in `SECURITY.md`.
* IP/trademark policy included; background theory proprietary unless explicitly granted.

---

## 7) Conformance & CI

Add these to CI to prevent regressions:

```yaml
- name: Conformance (core)
  run: cargo run -p grieg-cli -- --jsonl conformance/smoke.jsonl --mem

- name: Conformance (phase dominance)
  run: cargo run -p grieg-cli -- --jsonl conformance/phase-dominance.jsonl --mem

- name: Conformance (SAP mock)
  run: cargo run -p grieg-cli -- --jsonl conformance/sap-smoke.jsonl --mem
```

Optionally **fail** when any line prints `"ok": false"` (grep the output).

---

## 8) Troubleshooting

* **“expected primary” at column 1 on `{`** → you passed JSON to `--expr`; use `--jsonl <file>`.
* **`ok:false` on JAM/MEM tests** → avoid **unbound** identifiers in those examples, or change the evaluator to enforce global JAM dominance.
* **`DateTime<Utc>: Serialize`** → enable chrono with serde in the affected crate:
  `chrono = { version = "0.4", features = ["serde"] }`.
* **Duplicate keys in Cargo.toml** → keep exactly one `serde`, one `serde_json`, one `grieg-engine`, one `grieg-parser`.

---

## 9) Roadmap

* **Adapters**: `--sql`, `--rfc`, `--odata` (mock → real) in `grieg-ipc-cli`.
* **Trace/Geometry**: optional emission; heatmaps/tensors in notebooks/dashboards.
* **Policy**: per-vertical conformance packs (Factory, Retail, Medical, Defense, Legal, Academic, Cybersecurity, Cybernetics).
* **Bindings**: finalize Python ergonomics (`grieg.version()`, `grieg.features()`), minimal WASM demo.

---

## 10) Appendix — Minimal Expressions for SAP-Like Semantics

* **Witnessed MEM:** `@mem(true)` → MEM
* **Unwitnessed/VAC:** `sap_unbound` → VAC
* **JAM dominance:** `@jam(true) -> true` → JAM
* **MEM over ALIVE:** `(true -> true) -> @mem(true)` → MEM

Use these as building blocks in JSONL tests until SQL/RFC/ODATA adapters are enabled.

**How to run (one-liner recap):**

```bash
cargo run -p grieg-cli -- --jsonl conformance/sap-smoke.jsonl --mem --ast
```

---

## 11) Endpoint-Agnostic Architecture (Beyond SAP)

While this pack highlights **SAP** as the flagship integration, Grieg itself is **endpoint-agnostic**.

### Core Principle

Grieg evaluates **expressions**; its inputs/outputs are **JSON and JSONL**.
Adapters (`grieg-ipc`, `grieg-telemetry`) translate **any system** into this substrate:

* **ERP**: SAP (declared vertical), Oracle EBS, Workday
* **Databases**: Postgres, MSSQL, custom NoSQL
* **Sensors**: IoT devices, medical instruments, defense telemetry, academic experiments
* **Cybersecurity**: IDS/IPS logs, auth events, threat intel feeds
* **Cybernetics**: Feedback/control loops, robotics

### Why This Works

* The **engine is invariant**: JAM, MEM, VAC, ALIVE always apply.
* Only the **adapters change**: `--sql`, `--rfc`, `--odata`, `--jsonl` are mappings.
* **Sensor capacity**: signals become **witnesses** (MEM) or **contradictions** (JAM), providing cybernetic stability at the edge.

### Declared Stance

* **SAP**: chosen ERP integration, full support via IPC/telemetry.
* **Other systems**: fully possible; Grieg makes no assumption about source.
* **Government/Defense**: treat JAM as *dominant threat state*, MEM as *forensic witness*, VAC as *no signal/uncertain*, ALIVE as *operational normal*.

### Vision

Grieg is a **substrate for edge intelligence**:

* Runs at **factory/store/clinic/field nodes**
* Consumes **ERP/DB/sensor/cyber feeds**
* Emits **deterministic JSON/phase traces**
* Powers both **enterprise workflows** (ERP) and **cybernetic systems** (security, robotics, defense)

📌 For broader endpoint strategy and candidates, see [README-ENDPOINTS.md](../grieg-endpoints/README-ENDPOINTS.md)

```
::contentReference[oaicite:0]{index=0}

# Grieg — SAP Integration (Mock + IPC)

This document introduces Grieg’s interaction layer for SAP-like environments.  
It shows how the reasoning engine can process SAP-style inputs (symbols, transport, JAM dominance)  
without requiring a live SAP system. This serves as a proof of wiring for researchers and vendors.

---

## 0) TL;DR Quickstart (no SAP backend yet)

```bash
# From repo root
cargo build -p grieg-cli

# Run the SAP mock smoke (JSONL; compares expected phases → ok:true/false)
cargo run -p grieg-cli -- --jsonl conformance/sap-smoke.jsonl --mem --ast

Expected output (example):

{"input":"@mem(sap_ok)","phase":"MEM","ok":true,"expect_phase":"MEM"}
{"input":"sap_unbound","phase":"VAC","ok":true,"expect_phase":"VAC"}
{"input":"@jam(true) -> true","phase":"JAM","ok":true,"expect_phase":"JAM"}
{"input":"(true -> true) -> @mem(true)","phase":"MEM","ok":true,"expect_phase":"MEM"}

Why this matters: the smoke proves wiring & semantics end-to-end
(JSONL → parser → evaluator → JSON out) without requiring an SAP system.

⸻

1) Scope
	•	ERP vendors: SAP, Oracle, others. Grieg stays endpoint-agnostic; SAP is chosen as the first mock.
	•	Government / Defense: Edge telemetry, legal, medical, aerospace.
	•	Vendors: Cybersecurity, industrial automation, robotics, retail.

⸻

2) IPC / Telemetry link

This builds on grieg-ipc and emits invariant pulse events (basis5-derived):
	•	JAM boundary
	•	MEM transport
	•	VAC → ALIVE witness arrival
	•	Sink (implication fixed-point)
	•	Winding (cumulative topological circuit)

These events are deterministic, non-interfering, and testable.
They can be exported as telemetry to monitoring systems.

⸻

3) JSONL Conformance Example

File: conformance/sap-dominance.jsonl

{"expr":"@mem(sap_ok) -> @jam(false)","mem":true,"expect_phase":"JAM","note":"JAM dominates MEM"}
{"expr":"sap_unbound -> @mem(true)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE; antecedent VAC irrelevant"}
{"expr":"sap_unbound","mem":false,"expect_phase":"VAC","note":"Unwitnessed SAP symbol → VAC"}
{"expr":"(true -> true) -> @mem(sap_cached)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE"}

Run:

cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast


⸻

4) Endpoints

This file is part of the broader endpoint strategy: see
👉 README-ENDPOINTS.md

⸻

5) Math & Logic (MML)

For a formal, verifiable note on phase dominance law and JSONL proofs, see:
👉 README-MML.md

⸻

6) Appendix — Minimal Expressions for SAP-Like Semantics
	•	Witnessed MEM:
@mem(true) → MEM
	•	Unwitnessed / VAC:
sap_unbound → VAC
	•	JAM dominance:
@jam(true) -> true → JAM
	•	MEM over ALIVE:
(true -> true) -> @mem(true) → MEM

Use these as building blocks in JSONL tests until SQL/RFC/ODATA adapters are enabled.

⸻

7) One-liner Recap

cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast


⸻

Status: SAP mock verified. IPC telemetry integrated.
Higher-level adapters (ODATA, RFC, SQL) may be layered on without changing Grieg’s core semantics.

