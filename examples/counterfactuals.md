# Counterfactuals — Quick Tour

These examples show how **phases** interact with simple expressions using the `grieg-cli`.  
You can copy each command into a terminal on your laptop (Linux/macOS/WSL).

---

## 0) Warm-up: MEM transport

**Idea:** enable MEM and use a minimal expression that lands in **MEM**.

```bash
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty
```

> **Expected:** Phase is `MEM`. (The value component may be `true/false/None` depending on evaluator rules; the point here is the **phase**.)

---

## 1) Vacuous context (no witness) → VAC

**Idea:** identifiers without a witness should evaluate in the **VAC** phase.

```bash
cargo run -p grieg-cli -- --expr 'A -> B' --pretty
```

> **Expected:** Phase is `VAC`, value likely `None` (no witness assigned).  
> This demonstrates that Grieg distinguishes “no witness” from ordinary truth-functional flow.

---

## 2) Implication sink: modus-ponens fixed point

**Idea:** implication sinks are absorbing in trace semantics (fixed-point behavior).  
Below is a simple implication chained with a fact; exact reduction is engine-specific, but the **phase** should stay **ALIVE** with MEM disabled.

```bash
cargo run -p grieg-cli -- --expr '(true -> true) -> true' --pretty
```

> **Expected:** Phase `ALIVE` (no MEM, no VAC needed, no JAM).  
> The example is intentionally trivial to avoid binding requirements.

---

## 3) Toggle MEM on/off

**Idea:** same expression, compare with/without `--mem`.

```bash
# Without MEM
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --pretty

# With MEM
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty
```

> **Expected:** Without `--mem`, the engine is free to stay outside MEM transport;  
> with `--mem`, phase should be `MEM`.

---

## 4) Peek at the AST

**Idea:** parse-only view for quick sanity.

```bash
cargo run -p grieg-cli -- --expr 'A -> B' --ast
```

> **Expected:** An AST print that shows the implication structure.  
> Use this to debug grammar expectations before chasing phase behavior.

---

## Notes & Guardrails

- **Phases:** `JAM > MEM > VAC > ALIVE` (dominance). If multiple conditions apply, the dominant phase wins.  
- **Geometry:** Optional (`emit_geometry` feature). These examples avoid it; evaluation outcomes must not depend on geometry.  
- **Witnessing:** Unbound identifiers lead to `VAC` phase (value often `None`). Bindings/witnesses are introduced by constructs like `@mem(...)` or your own inputs as the engine evolves.

---

## Next Steps

- Add a minimal **bindings** example once `pyo3` is in (e.g., Python one-liner calling `Evaluator`).  
- Expand with a small **JSONL** batch once we finalize the CLI JSONL schema.  
- Consider an **examples/provenance.ipynb** for trace visualization when `emit_geometry` stabilizes



