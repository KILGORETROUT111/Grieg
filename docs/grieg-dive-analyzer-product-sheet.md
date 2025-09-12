# Grieg Dive Analyzer — Offline “Shadow/Replay” Reasoning for Dive Computers
**Purpose:** Post-dive validation, training, and safety analytics—*without* touching firmware.

---

## What it does
Grieg replays completed dives and evaluates rulepacks over your logs to produce:
- **Boundary events (JAM):** hard-limit breaches (e.g., ceiling + ascent spike, PPO₂ > limit).
- **Sinks (commit points):** places where an “if…then” chain resolves (e.g., gas switch at MOD).
- **Witness gaps (VAC):** missing/implausible sensor data and its impact on decisions.
- **What-if (MEM) deltas:** counterfactual checks (e.g., “+3 min at 30 m ⇒ +8 min TTS?”).

Outputs: **PDF timeline** for humans + **JSON** for CI/QA pipelines.

---

## Who it’s for (and why)
- **Dive computer OEMs:** regression testing across thousands of dives; auditable risk evidence.
- **Training/cert agencies:** objective debriefs, classroom scenarios, instructor artifacts.
- **Research & safety units:** cohort analysis, incident reviews, policy evaluation.
- **Expeditions/commercial ops:** standardized after-action reports and trends.

---

## Inputs (adapter supports multiple formats)
- Time series: `t, depth, ascent_rate, ceiling, NDL, TTS, PPO2, CNS, OTU, tank_pressure, gas(O2/He), temp`
- Sensor flags: per-sensor OK/fault, disagreement bits
- Optional: distance-to-line, HR/SpO₂, motion

*Formats:* UDDF/CSV/JSON; vendor-specific adapters pluggable.

---

## Key features
- **Basis5 phase engine:** ALIVE / JAM / MEM / VAC dominance; clean boundary semantics.
- **Counterfactuals (off-device):** safe *what-if* analysis using the same deco kernel you specify.
- **Deterministic & auditable:** identical inputs ⇒ identical events; artifact bundle (PDF+JSON).
- **Policy-as-rules:** rules are versioned text (reviewable, diffable), not hidden heuristics.

---

## Deliverables
- **Desktop/CLI app**: `grieg-dive analyze --in logs/ --out reports/`
- **SDK/CI integration**: GitHub Actions example to fail builds on new boundary regressions
- **Template rulepacks**: Rec/tech starter sets; vendor-tuned packs on request

---

## Example rules (sketch)
- `@jam( ceiling > 0 & ascent_rate > 10m/min )`
- `@jam( PPO2 > 1.6 )`
- `@mem( extend_bottom(3min) -> TTS_increase >= 8min )`
- `@mem( gas_switch(EAN50@21m) -> TTS_decrease >= 5min )`
- `@vac( PPO2_sensor_major_disagree )`

---

## Report (at a glance)
- Header: dive meta, rulepack version, kernel version
- Timeline: **Boundary** (red), **Sink** (green), **Witness loss** (amber)
- What-if table: scenario ⇒ ΔNDL, ΔTTS, Δstops
- Appendix: raw events JSON + hashes for chain-of-custody

---

## Deployment & data
- Runs offline on Win/macOS/Linux. No cloud required.
- Privacy: logs stay on your machine unless you opt-in to share.
- Pricing: pilot/site/OEM license; adapters & custom rulepacks available.

**Contact:** psirt@keemail.me  
**Repo:** https://github.com/KILGORETROUT111/Grieg