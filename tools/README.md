# Tools

**Status**: PLACEHOLDER — Not implemented

---

## Purpose

External tools and utilities that integrate with JOCKY but are not part of the core crates.

---

## Planned Tools

| Tool | Purpose | Language |
|------|---------|----------|
| `jockey-vql-compiler` | Compile JOCKY IR → Velociraptor VQL | Rust |
| `jockey-kape-generator` | Compile JOCKY IR → KAPE targets | Rust |
| `jockey-case-exporter` | Export evidence → CASE/UCO JSON-LD | Rust |
| `jockey-timesketch-exporter` | Export timeline → Timesketch CSV | Rust |
| `jockey-sigma-converter` | Convert JOCKY rules → Sigma YAML | Rust |
| `jockey-yara-compiler` | Compile JOCKY indicators → YARA rules | Rust |

---

## Integration Points

These tools consume JOCKY's IR/Execution Contract output and produce artifacts for external platforms:

```
JOCKY Script
    │
    ▼
IR / Execution Contract
    │
    ├──▶ jockey-vql-compiler ──▶ hunt.vql
    ├──▶ jockey-kape-generator ──▶ kape_targets.json
    ├──▶ jockey-case-exporter ──▶ evidence.case.json
    ├──▶ jockey-timesketch-exporter ──▶ timeline.csv
    ├──▶ jockey-sigma-converter ──▶ rules.yaml
    └──▶ jockey-yara-compiler ──▶ rules.yara
```

---

## Current State

No tools implemented yet. This directory exists for future organization.