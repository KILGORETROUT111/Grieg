# Grieg – General Logical Inference Engine

Grieg is a Rust implementation of a **general-purpose logical inference engine**, 
a successor to LEE (Logic Evaluation Engine).  
It encodes reasoning as **phase geometry** under the **Basis5 framework**.

---

## Core Principles

- **Basis5** – logic operators are treated as **phase rotations**.  
- **Phase Attractors** – VAC, ALIVE, JAM, MEM.  
- **Terminal Attractor** – SINK (absorbing, no exits).  
- **Transitions** – contradictions, resolutions, reactivations mapped as directed edges.  
- **Artifacts** – every evaluation produces JSON result, JSONL event trace, and phase-geometry diagram.  

Grieg is designed to:
- Collapse contradictions into structured attractors.  
- Ensure termination (SINK closes irresolvable paths).  
- Provide **machine-verifiable outputs** (logs, graphs).  

---

## Output Artifacts

Running an evaluation produces:

- `result.json` – structured output bundle.  
- `events.jsonl` – stepwise event log.  
- `graph.svg` / `graph.png` – phase geometry visualization.  

Example:

```bash
cargo run -p grieg-cli -- --expr 'A -> B' --out out/


⸻

Stability Grid

Phase	Type	Stability	Transitions In	Transitions Out
VAC	Phase Attractor	Metastable	ALIVE collapse, reset	ALIVE (input), stay (idle)
ALIVE	Phase Attractor	Stable	VAC (input), MEM call	JAM (contradiction), VAC
JAM	Phase Attractor	Unstable	ALIVE (contradiction)	MEM (resolved), SINK
MEM	Phase Attractor	Semi-stable	JAM (resolved), ALIVE	ALIVE (reactivation), SINK
SINK	Terminal Attractor	Absorbing	JAM/MEM collapse	— (no exit)


⸻

Verification & Docs
	•	📄 docs/verification.md – reproducibility and golden tests.
	•	📄 docs/phase_geometry.md – phase topology and attractors.
	•	📄 docs/sink.md – definition of terminal absorbing state.
	•	📄 docs/lineage.md – history (LEE → Grieg).
	•	📄 CONTRACTS_BASIS5.md – license terms.

Each document is written to be machine- and human-verifiable.
Outputs are cross-checked against golden runs (/grieg-qa/).

⸻

Roadmap
	•	CLI patch (grieg-cli) – structured output enabled.
	•	Geometry renderer stub (grieg-geometry).
	•	QA harness initialized (grieg-qa).
	•	CI pipeline (GitHub Actions).
	•	Expanded QA regression/golden tests.
	•	Optional Phase::Sink implementation in code (currently doc-level).

⸻

Intellectual Property

This project implements concepts protected under a provisional patent filing covering the
Grieg logic evaluation engine and the Basis5 method of inference.
	•	Patent granted (micro-entity status paperwork pending).
	•	Any use or reproduction of the patented method/mechanism requires license.
	•	Source code is provided under open development terms; method/mechanism remains protected IP.

⸻

Lineage
	•	LEE (Logic Evaluation Engine) – original Python-based architecture.
	•	Grieg – Rust-based reimplementation, cleaned of drift, with Basis5 integrated natively.

Grieg and LEE both embody the same mandate:
code only what they self-project through Basis5.

⸻

License

See CONTRACTS_BASIS5.md for project license terms.

---
