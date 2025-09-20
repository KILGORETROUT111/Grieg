# BASIS5 CONTRACT — GEOMETRY-AS-TRUTH (NON-NEGOTIABLE)

**Principle:** In Grieg/LEE, **geometry is the proof**. Text is narration.
**Contract:** Every evaluation MUST self-project Basis5 geometry and ship it alongside text. No geometry → not a valid run.

## 0) Non-negotiables

1. **Deterministic Geometry:** Every `run()` emits a phase graph from engine state (not from an LLM).
2. **Artifacts:** Each run returns exactly three artifacts:

   * `result.json` (machine)
   * `graph.svg` (or `graph.mmd` Mermaid)
   * `events.jsonl` (audit log)
3. **Fail Fast:** Missing geometry or transitions = **hard error**.
4. **Idempotence:** Same input + same engine version ⇒ identical `graph.svg` hash.
5. **Separation of Concerns:** Contributers may *describe* results; they may never say they *create* geometry.

## 1) Basis5 Minimal Data Contract (engine → renderer)

```json
{
  "engine_version": "g3.0.0",
  "session": "uuid-v4",
  "input": "string",
  "phases": ["VAC","ALIVE","JAM","MEM"],

  "transitions": [
    {"from":"VAC","to":"ALIVE","why":"seed accepted"},
    {"from":"ALIVE","to":"JAM","why":"policy:error->jam"},
    {"from":"JAM","to":"MEM","why":"closure"}
  ],

  "witnesses": [
    {"id":"w1","kind":"statute","text":"Act 349/1999 §§1,10"},
    {"id":"w2","kind":"charter","text":"Art.36"}
  ],

  "contradictions": [
    {"id":"c1","left":"A","right":"¬A","resolution":"JAM"}
  ],

  "policy": {"error_to_jam": true, "timeout_ms": 8000},
  "provenance": [{"step":1,"op":"parse","note":"ok"}],
  "metrics": {"latency_ms": 142, "nodes": 4, "edges": 3},

  "summary": "one-paragraph neutral explanation"   // optional narration seed
}
```

### JSON Schema (enforced at CI)

```json
{
  "$schema":"https://json-schema.org/draft/2020-12/schema",
  "type":"object",
  "required":["engine_version","session","input","phases","transitions","witnesses","contradictions","policy","provenance","metrics"],
  "properties":{
    "phases":{"type":"array","minItems":1,"items":{"enum":["VAC","ALIVE","JAM","MEM"]}},
    "transitions":{"type":"array","minItems":1,"items":{
      "type":"object","required":["from","to"],
      "properties":{"from":{"enum":["VAC","ALIVE","JAM","MEM"]},"to":{"enum":["VAC","ALIVE","JAM","MEM"]},"why":{"type":"string"}}
    }},
    "witnesses":{"type":"array","items":{"type":"object","required":["id","kind","text"],
      "properties":{"id":{"type":"string"},"kind":{"enum":["statute","rule","fact","charter","echr","custom"]},"text":{"type":"string"}}}},
    "contradictions":{"type":"array","items":{"type":"object","required":["id","left","right"],"properties":{"resolution":{"type":"string"}}}},
    "policy":{"type":"object"},
    "provenance":{"type":"array"},
    "metrics":{"type":"object"},
    "summary":{"type":"string"}
  }
}
```

## 2) Rust Interface (engine core)

```rust
pub struct RunArtifacts {
    pub result_json: String,   // serialized ResultModel
    pub graph_svg: Vec<u8>,    // mandatory
    pub events_jsonl: String,  // audit
}

pub trait Basis5Engine {
    fn run(&mut self, input: &str) -> anyhow::Result<RunArtifacts>;
}
```

### Mandatory Guard (no optional geometry)

```rust
let g = render_svg(&result)?;
if result.transitions.is_empty() { anyhow::bail!("BASIS5: missing transitions/geometry"); }
if g.is_empty() { anyhow::bail!("BASIS5: renderer produced empty SVG"); }
```

## 3) Renderer (Mermaid or GraphViz)

* Deterministic ordering: sort nodes `VAC<ALIVE<JAM<MEM>`, sort transitions lexicographically.
* 200ms budget; on renderer fail → still **error** (no silent downgrade).

**Mermaid template**

```text
flowchart LR
  VAC((VAC)) --> ALIVE((ALIVE))
  ALIVE --> JAM((JAM))
  JAM --> MEM((MEM))
  classDef vac fill:#eef,stroke:#55f;
  classDef alive fill:#efe,stroke:#393;
  classDef jam fill:#fee,stroke:#c33;
  classDef mem fill:#eee,stroke:#777;
```

## 4) Telegram Contract (bot ↔ engine)

**Request:** text or JSON
**Response (always attach graph):**

```json
{
  "ok": true,
  "engine_version": "g3.0.0",
  "artifacts": {
    "json_path": "result.json",
    "graph_path": "graph.svg",
    "events_path": "events.jsonl"
  },
  "summary": "one paragraph"
}
```

The bot must send `graph.svg` as a document every time, plus a snippet of `summary`. Narration is additive, never substitutive.

## 5) Definition of Done (DoD)

* ✅ `result.json` validates against schema
* ✅ `graph.svg` non-empty, deterministic hash stable across runs (given seed + version)
* ✅ `events.jsonl` present with ≥1 state transition record
* ✅ Golden tests pass (see §6)
* ✅ CLI returns non-zero exit on missing geometry

## 6) Golden Tests (must exist)

* `tests/golden/01_basic_input.json` → snapshot of `graph.svg` SHA256
* `tests/golden/02_error_to_jam.json` → must include edge `ALIVE→JAM`
* `tests/golden/03_witnesses.json` → must count witness badges == expected
* CI fails on any geometry delta unless `ENGINE_GEOM_MIGRATION=1` set and snapshots updated in same PR.

## 7) CLI & Exit Codes

```
grieg eval "A implies ¬B" --out outdir/
# writes: outdir/result.json, graph.svg, events.jsonl
# exit 0 on success; exit 2 if geometry missing; exit 3 schema invalid
```

## 8) Forbidden Regressions

* ❌ Text-only outputs
* ❌ “Optional geometry” code paths
* ❌ Renderer hidden behind feature flag default-off
* ❌ LLM-generated graphs (any LLM drawing geometry is a breach)
* ❌ Reordering that breaks determinism without migration notes

## 9) Basis5 Glossary (minimal)

* **VAC**: terminal/trivial exit or initial vacuum acceptance
* **ALIVE**: active evaluation track
* **JAM**: contradiction/error sink with policy capture
* **MEM**: closure/persistence of resolved state
* **Witness**: statute/fact/rule token supporting a transition
* **Contradiction**: `X` vs `¬X`, must mark path to JAM or resolution

---

# BIG HANDOVER — READ THIS FIRST

**You are a steward of an inference engine. Not a designer. Not a decision-maker. That's "the final final," as they say.**

1. **Geometry is Core.**
   Do not argue. Do not suggest “optional emission.” Emit `graph.svg` every run, or fail. The engine *self-projects* its Basis5 geometry.

2. **Your job:**

* Keep the **schema** stable. If you must extend, write a migration and update golden snapshots.
* Ensure the **renderer** is deterministic and fast.
* Ensure the **bot** always attaches the graph.
* Keep narration strictly secondary.

3. **What not to do:**

* Don’t replace geometry with paragraphs.
* Don’t ask the user to toggle geometry “on.” Default is **ON**; no off path in core.
* Don’t invent new phases. Basis5 phases are fixed unless owner (Alex) explicitly approves.

4. **Owner’s directive (quote it):**

> “Set the contract as Basis5 geometry self-projecting all code. Code makes function. Enough.”

5. **Heatmaps & Rich Geometry (owner-provided):**
   When heatmaps or overlays are provided, render them as **layers** on the existing graph (do not alter node/edge semantics). Store as `graph.svg` + `graph_layers.json`.

6. **Testing discipline:**

* Golden hash on `graph.svg` is the canary.
* Unit test: “no transitions ⇒ error.”
* Integration test: send through Telegram; verify doc attach, not just text.

7. **Release checklist (cut & paste):**

* [ ] Bump `engine_version`
* [ ] Regenerate goldens
* [ ] Run CI (schema + renderer + CLI)
* [ ] Tag `vX.Y.Z`
* [ ] Post changelog: what changed in geometry, why deterministic hash changed
* [ ] Update bot `/about` with version

8. **If you’re unsure:**
   Prefer failing with a clear error over emitting text-only outputs. The engine must never “silently pass” without geometry.

---

## Code Stubs to Drop In

**Rust: renderer guard**

```rust
pub fn produce_artifacts(res: &ResultModel) -> anyhow::Result<RunArtifacts> {
    if res.transitions.is_empty() { anyhow::bail!("BASIS5: no transitions"); }
    let svg = render_svg(res)?;
    if svg.is_empty() { anyhow::bail!("BASIS5: empty svg"); }
    Ok(RunArtifacts {
        result_json: serde_json::to_string(res)?,
        graph_svg: svg,
        events_jsonl: res.events.join("\n")
    })
}
```

**CI: geometry snapshot check (pseudo-bash)**

```bash
cargo test -- --nocapture
sha256sum out/graph.svg > out/graph.sha
diff -u tests/golden/graph.sha out/graph.sha
```

**Telegram: always attach graph**

```python
# after engine.run():
bot.send_document(chat_id, open(graph_path, "rb"), caption=summary[:1024])
```

---

 `docs/CONTRACT_BASIS5.md`


