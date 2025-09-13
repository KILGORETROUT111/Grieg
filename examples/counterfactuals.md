# Counterfactuals — Quick Tour

These examples show how **phases** interact with simple expressions using the `grieg-cli`.  
You can copy each command into a terminal on your laptop (Linux/macOS/WSL).

---

## 0) Warm-up: MEM transport

**Idea:** enable MEM and use a minimal expression that lands in **MEM**.

```bash
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty


Expected: Phase is MEM. (The value component may be true/false/None depending on evaluator rules; the point here is the phase.)






