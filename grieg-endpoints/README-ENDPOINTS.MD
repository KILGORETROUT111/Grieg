# Grieg — Endpoint-Agnostic Architecture

Grieg is a **phase-resolved reasoning engine**.  
It does not bind to one system; it evaluates expressions from **any endpoint** once translated into JSON/JSONL.

---

## Core Principle

- **Engine invariant**: Phases JAM, MEM, VAC, ALIVE always apply.
- **Adapters variable**: Translate external systems into JSONL for Grieg.
- **Outputs stable**: Deterministic JSON traces, suitable for analysis or feedback control.

---

## Supported & Declared Endpoints

- **ERP**  
  - SAP (declared vertical, see [SAP integration](../grieg-sap/README-SAP.md))  
  - Oracle EBS  
  - Workday  

- **Databases**  
  - Postgres  
  - MSSQL  
  - NoSQL systems  

- **Sensors**  
  - IoT devices (industrial, retail, medical)  
  - Defense telemetry  
  - Academic instrumentation  

- **Cybersecurity**  
  - IDS/IPS log feeds  
  - Threat intelligence streams  

- **Cybernetics**  
  - Feedback/control loops  
  - Robotics  

---

## Why It Works

- **Witnessing**: Signals and records act as MEM witnesses.  
- **Contradictions**: JAM dominates inconsistent or hostile inputs.  
- **Unknowns**: VAC phases mark uncertainty until resolved.  
- **Normal ops**: ALIVE covers baseline functioning.  

---

## Candidate Integrations (to be filled)

- Oracle EBS  
- Workday  
- Postgres  
- IoT devices  

---

📌 *For the canonical SAP integration, see [README-SAP.md](../grieg-sap/README-SAP.md).*
