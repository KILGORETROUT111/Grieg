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
```

---

## Stability Grid

| Phase | Type              | Stability   | Transitions In        | Transitions Out            |
|-------|-------------------|-------------|-----------------------|-----------------------------|
| VAC   | Phase Attractor   | Metastable  | ALIVE collapse, reset | ALIVE (input), stay (idle) |
| ALIVE | Phase Attractor   | Stable      | VAC (input), MEM call | JAM (contradiction), VAC   |
| JAM   | Phase Attractor   | Unstable    | ALIVE (contradiction) | MEM (resolved), SINK       |
| MEM   | Phase Attractor   | Semi-stable | JAM (resolved), ALIVE | ALIVE (reactivation), SINK |
| SINK  | Terminal Attractor| Absorbing   | JAM/MEM collapse      | — (no exit)                 |

---

## Verification & Docs

- 📄 [docs/verification.md](docs/verification.md) – reproducibility and golden tests.  
- 📄 [docs/phase_geometry.md](docs/phase_geometry.md) – phase topology and attractors.  To-Do List (simplifying the big mathematics)
- 📄 [docs/sink.md](docs/sink.md) – definition of terminal absorbing state.  To-Do List
- 📄 [docs/lineage.md](docs/lineage.md) – history (LEE → Grieg).  
- 📄 [docs/CONTRACTS_BASIS5.md](docs/CONTRACTS_BASIS5.md) - license terms.  

Each document is written to be **machine- and human-verifiable**.  
Outputs are cross-checked against golden runs (`/grieg-qa/`).  

---

## Roadmap

- [x] CLI patch (`grieg-cli`) – structured output enabled.  
- [x] Geometry renderer stub (`grieg-geometry`).  
- [x] QA harness initialized (`grieg-qa`).  
- [ ] CI pipeline (GitHub Actions).  
- [ ] Expanded QA regression/golden tests.  
- [ ] Optional `Phase::Sink` implementation in code (currently doc-level).  

---

## Intellectual Property

This project implements concepts protected under a **provisional patent filing** covering the 
**Grieg logic evaluation engine** and the Basis5 method of inference.  

- Patent granted (micro-entity status paperwork pending).  
- Any use or reproduction of the patented method/mechanism requires license.  
- Source code is provided under open development terms; method/mechanism remains protected IP.  

---

## Lineage

- **LEE (Logic Evaluation Engine)** – original Python-based architecture.  
- **Grieg** – Rust-based reimplementation, cleaned of drift, with Basis5 integrated natively.  

Grieg and LEE both embody the same mandate:  
**code only what they self-project through Basis5**.  

---

## License

See `CONTRACTS_BASIS5.md` for project license terms.  


# How Grieg Differs

Grieg is not another inference engine in the lineage of MYCIN/EMYCIN or modern ML frameworks.  
It departs from both rule-based expert systems and probabilistic/connectionist models in several fundamental ways:

## 1. Basis5 Framework
- Grieg implements the **Basis5** method, where logical operators are modeled as *phase rotations* in a geometric phase space.  
- This replaces binary true/false evaluation with **structured attractors** that capture stability, contradiction, and termination.  
- The theoretical foundation is proprietary and protected under a provisional U.S. patent.

## 2. Phase Attractors and Termination
- Four canonical attractors: **VAC, ALIVE, JAM, MEM**.  
- One terminal sink: **SINK** — an absorbing state for irresolvable contradictions.  
- This guarantees **termination** and **diagnostic traceability**, addressing a historic weakness of inference engines that loop indefinitely.

## 3. Machine-Verifiable Artifacts
- Every evaluation emits:  
  - `result.json` — structured outcome bundle  
  - `events.jsonl` — stepwise transition log  
  - `graph.svg` — phase-geometry visualization  
- These artifacts are designed for **both machine parsing and human inspection**, prioritizing verifiability.

## 4. Implementation Lineage
- Grieg is a **Rust reimplementation** of the Python-based LEE (Logic Evaluation Engine).  
- Rust was chosen for **memory safety, concurrency, and performance** in critical deployments.  
- Unlike Rust ML toolchains (e.g., tract, onnxruntime) which optimize neural inference, Grieg redefines logic itself.

## 5. Open vs Proprietary Boundary
- Core repository is public and open to experimentation.  
- **Basis5 method** remains proprietary under patent protection.  
- Research use is permitted; **commercial deployment requires licensing**.

---

Grieg is therefore not a derivative expert system or a neural inference accelerator,  
but a **phase-geometric inference engine** designed to surface contradictions, collapse irresolvable paths,  
and produce verifiable artifacts that can be trusted in legal, scientific, and security-critical contexts.