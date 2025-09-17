# Grieg — SAP Integration (Mock + IPC)

This document introduces Grieg’s interaction layer for SAP-like environments.  
It shows how the reasoning engine can process SAP-style inputs (symbols, transport, JAM dominance)  
without requiring a live SAP system. This serves as a proof of wiring for researchers and vendors.


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

1) Scope
	•	ERP vendors: SAP, Oracle, others. Grieg stays endpoint-agnostic; SAP is chosen as the first mock.
	•	Government / Defense: Edge telemetry, legal, medical, aerospace.
	•	Vendors: Cybersecurity, industrial automation, robotics, retail.

2) IPC / Telemetry link

This builds on grieg-ipc and emits invariant pulse events (basis5-derived):
	•	JAM boundary
	•	MEM transport
	•	VAC → ALIVE witness arrival
	•	Sink (implication fixed-point)
	•	Winding (cumulative topological circuit)

These events are deterministic, non-interfering, and testable.
They can be exported as telemetry to monitoring systems.

3) JSONL Conformance Example

File: conformance/sap-dominance.jsonl

{"expr":"@mem(sap_ok) -> @jam(false)","mem":true,"expect_phase":"JAM","note":"JAM dominates MEM"}
{"expr":"sap_unbound -> @mem(true)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE; antecedent VAC irrelevant"}
{"expr":"sap_unbound","mem":false,"expect_phase":"VAC","note":"Unwitnessed SAP symbol → VAC"}
{"expr":"(true -> true) -> @mem(sap_cached)","mem":true,"expect_phase":"MEM","note":"MEM dominates ALIVE"}

Run:

cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast

4) Endpoints

This file is part of the broader endpoint strategy: see
👉 README-ENDPOINTS.md


5) Math & Logic (MML)

For a formal, verifiable note on phase dominance law and JSONL proofs, see:
👉 README-MML.md


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


7) One-liner Recap

cargo run -p grieg-cli -- --jsonl conformance/sap-dominance.jsonl --mem --ast
Status: SAP mock verified. IPC telemetry integrated.
Higher-level adapters (ODATA, RFC, SQL) may be layered on without changing Grieg’s core semantics.

---