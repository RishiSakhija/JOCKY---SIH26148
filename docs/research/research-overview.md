# Research Overview

**Status**: SPECIFIED

---

## Purpose

This document provides an overview of the research foundation for JOCKY, mapping academic and industry work to architectural decisions.

---

## Research Categories

| Category | Description | Key Sources |
|----------|-------------|-------------|
| **Forensic DSLs** | Domain-specific languages for forensics | Nugget (2018), VQL, KQL, ES\|QL |
| **Provenance Systems** | Evidence provenance tracking | Provexa/ProvQL (2025), DFIR-ORC, CASE/UCO |
| **Memory Forensics** | Volatile memory analysis | Volatility 3, LEMON (2026), Rusting Volatility |
| **eBPF Forensics** | Kernel-level tracing | Aya, bpftrace, LEMON (2026) |
| **Timeline/Graph** | Causal reconstruction | Plaso, Timesketch, Maltego |
| **Standards** | Interoperability formats | CASE/UCO, NIST, STIX |

---

## Key Research Questions

| Question | JOCKY Answer |
|----------|--------------|
| **Why a new DSL?** | Not for DSL novelty — for *compilation target* unification (VQL, SQL, KAPE, Volatility, CASE) |
| **Why provenance chains?** | Not for novelty — for *deterministic verification* integrated with execution contracts |
| **Why deterministic correlation?** | ML is opaque; deterministic rules provide *explainable, verification-backed* edges |
| **Why Rust?** | Memory safety, performance, WASM target, growing forensics ecosystem |
| **Why eBPF as extension?** | Kernel visibility without drivers; CO-RE for portability |

---

## Research-to-Design Mapping

See `docs/research/research-to-design.md` for detailed mapping table.

---

## Literature Review

See `docs/research/literature-review.md` for verified academic/industry sources.

---

## Competitor Analysis

See `docs/research/competitor-analysis.md` for detailed comparison.

---

## Novelty Analysis

See `docs/research/novelty-analysis.md` for explicit differentiation.

---

## Sources

See `docs/research/sources.md` for verified bibliography.

---

## Related Documents

- `literature-review.md` — Verified research sources
- `competitor-analysis.md` — Tool comparison
- `novelty-analysis.md` — Differentiation
- `research-to-design.md` — Research to architecture mapping
- `sources.md` — Bibliography