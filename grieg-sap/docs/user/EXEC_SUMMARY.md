# EXEC_SUMMARY.md
**Grieg** is a compact reasoning engine (basis5 / four‑fold: ALIVE, JAM, MEM, VAC) usable from SAP via HTTPS.
It returns a classical value **and** an operational **phase**, so edge cases aren't hidden in ad‑hoc booleans.

**Why it matters in SAP landscapes**
- Data quality rules & gating: surface *vacuous* vs *boundary* vs *witness* states
- Safer automations: separate "no witness" (VAC) from "hard stop" (JAM)
- Explainability: optional AST & pretty output for audit

**Integration**
- Outbound ABAP client class (recommended)
- Optional ICF proxy endpoint
- CPI iFlow normalizer
