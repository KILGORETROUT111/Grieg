# Grieg Dive Analyzer — Replay Report (Sample)

**Dive ID:** SARDINIA-2025-09-01-01  
**Device/Firmware:** Shearwater (mock) v4.3.2 • Kernel: ZHL-16C+GF 85/85  
**Rulepack:** rec-1.0.0 (hash: 5f2e1c9) • Generated: 2025-09-12T10:30:00Z

---

## Executive Summary
- **Boundary (JAM) events:** **1**
- **Sinks (commit points):** **1**
- **Witness gaps (VAC):** **1**
- **Top what-ifs:**  
  1. +3 min @ 30 m ⇒ **+8 min TTS** (M-03)  
  2. Gas switch EAN50 @ 21 m ⇒ **−5 min TTS** (M-11)

---

## Timeline (UTC)
| Time     | Depth | Event   | Details                                                      |
|----------|-------|---------|--------------------------------------------------------------|
| 10:23:41 | 28.6m | **JAM** | Ceiling 3.0 m & Ascent 11.5 m/min (policy R-12)             |
| 10:31:12 | 21.0m | **SINK**| Gas switch EAN50 committed (policy P-07)                     |
| 10:35:09 | 18.4m | **VAC** | PPO₂ sensor disagreement (cells 1 vs 2+3), window 00:02:15  |

---

## What-If Scenarios (MEM)
| Scenario                           | ΔTTS | ΔNDL | ΔStops              | Notes    |
|------------------------------------|------|------|---------------------|----------|
| +3 min @ 30 m                      | +8m  |  —   | +1 × 3 m stop       | M-03     |
| Gas switch EAN50 @ 21 m (vs 18 m)  | −5m  |  —   | Last stop shortened | M-11     |

---

## Rule Hits
- **R-12 Boundary:** `@jam( ceiling > 0 & ascent_rate > 10m/min )` — hit ×1  
- **T-02 Toxicity:** `@jam( PPO2 > 1.6 )` — hit ×0  
- **M-03 MEM:** `extend_bottom(3min) -> TTS_increase >= 8min` — evaluated ✓

---

## Sensor Integrity
- PPO₂ cells: disagreement 00:02:15 (10:34:05–10:36:20)  
- Tank pressure: stable; no dropouts  
- Temp: −2.4 °C gradient; no rule impact

---

## Provenance
- Input log hash: `sha256: e3b0c44298...`  
- Rulepack hash: `sha256: 5f2e1c9a7b...`  
- Kernel hash: `sha256: 1a2b3c4d5e...`  
- Tool: Grieg Dive Analyzer v0.2.0 (deterministic)