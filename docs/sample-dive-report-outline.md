# Grieg Dive Analyzer — Sample Report Outline

## 1) Cover
- Title: Grieg Dive Analyzer — Replay Report
- Dive ID / Device / Firmware / Date
- Rulepack ID + hash, Kernel ID + hash
- Analyst / Org

## 2) Executive Summary (one page)
- Total events: Boundary (JAM), Sinks, Witness gaps (VAC)
- Highest severity boundary (time/depth/cause)
- Top 3 what-if outcomes (ΔTTS / ΔNDL / Δstops)

## 3) Timeline (UTC)
| Time     | Depth | Event      | Details                                      |
|----------|-------|------------|-----------------------------------------------|
| 10:23:41 | 28.6m | **JAM**    | Ceiling 3.0m & Ascent 11.5 m/min (policy R-12) |
| 10:31:12 | 21.0m | **SINK**   | Gas switch EAN50 committed (policy P-07)       |
| 10:35:09 | 18.4m | **VAC**    | PPO₂ sensor disagreement (cells 1 vs 2+3)     |

*Optional thumbnail chart: depth vs time with colored markers.*

## 4) What-If Scenarios (MEM)
| Scenario                            | ΔTTS | ΔNDL | Δstops               | Notes          |
|-------------------------------------|------|------|----------------------|----------------|
| +3 min @ 30 m                       | +8m  | —    | +1×3m stop added     | Rule M-03      |
| Gas switch EAN50 at 21 m (vs 18 m)  | −5m  | —    | Last stop shortened  | Rule M-11      |

## 5) Rule Hits
- R-12 Boundary: `@jam( ceiling > 0 & ascent_rate > 10m/min )` — hit ×1
- T-02 Toxicity: `@jam( PPO2 > 1.6 )` — hit ×0
- M-03 MEM: `extend_bottom(3min) -> TTS_increase >= 8min` — evaluated ✓

## 6) Sensor Integrity
- PPO₂ cells: disagreement window 00:02:15 (10:34:05–10:36:20)
- Tank pressure: stable; no dropouts
- Temp: −2.4 °C gradient; no effect on rules

## 7) Appendix A — Event Log (JSON)
- Machine-readable events (see schema)

## 8) Appendix B — Provenance
- File hashes (input logs, rulepack, kernel)
- Tool version, OS