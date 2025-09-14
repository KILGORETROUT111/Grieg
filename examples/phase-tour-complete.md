# Phase Tour — Complete Canonical Demonstrability of Four-Phase Schemata

These examples show how the four phases — **ALIVE, JAM, MEM, VAC** — behave in Grieg’s CLI.  
Each is runnable from the repo root with `cargo run -p grieg-cli -- …`.  
This file is the complete canonical demonstrability set: all four phases and dominance relations are exercised.

---

## 0) Warm-up: MEM transport

**Idea:** enable MEM and use a minimal expression that lands in **MEM**.

```bash
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty
```

> **Expected:** Phase is `MEM`.

---

## 1) Vacuous context (no witness) → VAC

**Idea:** identifiers without a witness should evaluate in the **VAC** phase.

```bash
cargo run -p grieg-cli -- --expr 'A -> B' --pretty
```

> **Expected:** Phase is `VAC` (value likely `None`, no witness assigned).

---

## 2) Implication sink: modus-ponens fixed point (ALIVE)

**Idea:** implication sinks are absorbing in trace semantics (fixed-point behavior).  
Here a trivial implication is reduced; phase should remain **ALIVE** with MEM disabled.

```bash
cargo run -p grieg-cli -- --expr '(true -> true) -> true' --pretty
```

> **Expected:** Phase is `ALIVE`.

---

## 3) Toggle MEM on/off

**Idea:** same expression, compare with/without `--mem`.

```bash
# Without MEM
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --pretty

# With MEM
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty
```

> **Expected:** Without `--mem`, phase is outside MEM transport.  
> With `--mem`, phase is `MEM`.

---

## 4) Peek at the AST

**Idea:** parse-only view for quick sanity.

```bash
cargo run -p grieg-cli -- --expr 'A -> B' --ast
```

> **Expected:** An AST print showing the implication structure.

---

## 5) JAM — Engine exception

**Idea:** provoke a runtime evaluation error that the engine maps to **JAM** (per `error_to_jam` policy).

```bash
# Replace <EXPR_JAM> with the known JAM trigger expression
cargo run -p grieg-cli -- --expr '<EXPR_JAM>' --pretty
```

> **Expected:** Phase is `JAM`.

---

## 6) JAM — Dominance over MEM

**Idea:** show that JAM dominates MEM.

```bash
cargo run -p grieg-cli -- --expr '@mem(<EXPR_JAM>)' --mem --pretty
```

> **Expected:** Phase is `JAM` (dominance over MEM).

---

## 7) JAM > VAC — Unwitnessed implication with JAM consequent

**Idea:** combine an unwitnessed antecedent (would be VAC) with a JAMing consequent; **JAM must dominate**.

```bash
cargo run -p grieg-cli -- --expr 'A -> <EXPR_JAM>' --pretty
```

> **Expected:** Phase is `JAM` (dominance over VAC).

---

## 8) JAM > ALIVE — Trivial ALIVE context with JAM subterm

**Idea:** put a JAMing subterm into a context that would otherwise be trivially ALIVE; **JAM must dominate**.

```bash
cargo run -p grieg-cli -- --expr '(<EXPR_JAM>) -> true' --pretty
```

> **Expected:** Phase is `JAM` (dominance over ALIVE).

---

## Notes & Guardrails

- **Phase dominance:** `JAM > MEM > VAC > ALIVE`.  
- **Geometry:** Optional (`emit_geometry` feature). Evaluation outcomes remain invariant; traces are additive witnesses only.  
- **Witnessing:** Unbound identifiers → `VAC`. Bindings introduced by constructs like `@mem(...)` or user inputs.  
- **JAM triggers:** Replace `<EXPR_JAM>` with the evaluator’s official JAM trigger and record it in `spec/LEDGER.md`.

---
