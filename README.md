# JOCKY

**Evidence-Contract Forensics Platform** — A unified workflow language and runtime for computer and network forensic analysis (SIH PS 26148).

---

## Problem

Digital forensics and incident response (DFIR) is fragmented across specialized tools that do not interoperate: Velociraptor for live hunting, KAPE for disk triage, Volatility for memory analysis, Plaso for timeline generation, Timesketch for collaborative analysis. Investigators manually translate hypotheses between incompatible query languages, data formats, and collection paradigms. No single workflow captures *intent*, executes *controlled collection*, produces *tamper-evident evidence*, and reconstructs a *causal attack narrative* with verifiable provenance.

---

## Our Approach

JOCKY is an **evidence-first forensic workflow language and runtime** that unifies the investigation lifecycle:

```
Investigator Intent
    → JOCKY Script (declarative, hypothesis-driven)
    → Lexer/Parser (pest)
    → AST (typed, validated)
    → Forensic IR (platform-agnostic execution plan)
    → Execution Contract (capability-validated, policy-bound)
    → Rust Runtime (controlled, sandboxed execution)
    → Windows/Linux Collectors (native, capability-scoped)
    → Common Evidence Model (typed, hashed, signed)
    → Evidence Receipt (Ed25519-signed, collector-attested)
    → Provenance Chain (hash-linked, append-only)
    → Deterministic Correlation (rule-based, evidence-cited)
    → Investigation Graph (causal, time-travel capable)
    → Verification (receipt + provenance validation)
    → Dashboard / Report (verification-backed export, citation-backed)
```

---

## Differentiation

> **JOCKY integrates forensic workflow definition, controlled execution, evidence receipts, provenance, deterministic correlation, and verified investigation output into one traceable workflow.**

JOCKY does **not** claim to invent forensic query languages, cross-platform collection, timelines, provenance, or investigation graphs. Instead, JOCKY unifies these capabilities in a single traceable workflow: one declarative script compiles to Velociraptor VQL, osquery SQL, KAPE targets, Volatility plugins, and a signed CASE/UCO bundle — with cryptographic evidence receipts and hash-linked provenance at every step.

---

## Status

- **Specification / Architecture**: Complete (Phase 0)
- **Core Implementation**: In progress (Phase 1+)
- **MVP Features**: Planned unless explicitly implemented
- **No implementation code exists yet** — this repository contains specifications, architecture, research, and documentation only

---

## Architecture

See the [System Architecture Diagram](assets/diagrams/system-architecture.mmd) and [Architecture Documentation](docs/architecture/).

---

## Documentation

| Document | Description |
|----------|-------------|
| [JOCKY_SPEC_SHEET.md](JOCKY_SPEC_SHEET.md) | Engineering source of truth |
| [JOCKY_DATA_CONTRACTS.md](JOCKY_DATA_CONTRACTS.md) | Evidence, Receipt, Provenance, Contract schemas |
| [Architecture](docs/architecture/) | System, data flow, contracts, collectors, correlation, deployment |
| [Language](docs/language/) | DSL grammar, AST, IR, examples |
| [Evidence](docs/evidence/) | Evidence model, receipts, provenance, verification |
| [Research](docs/research/) | Literature review, competitor analysis, novelty, sources |
| [Evaluation](docs/evaluation/) | Demo plan, success criteria, judge Q&A, limitations |
| [ROADMAP.md](ROADMAP.md) | 10-phase implementation plan |
| [SECURITY.md](SECURITY.md) | Security policy and vulnerability reporting |
| [CHANGELOG.md](CHANGELOG.md) | Version history |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines |

**Folders:** [assets/diagrams/](assets/diagrams/) · [collectors/](collectors/) · [data/](data/) · [frontend/](frontend/) · [tests/](tests/) · [screenshots/](screenshots/)

---

## Research

JOCKY's architecture is grounded in verified academic and industry research: Nugget (2018), Provexa/ProvQL (2025), LEMON (2026), Volatility 3, plus analysis of Velociraptor, osquery, GRR, KAPE, Plaso, Timesketch, and others. See [docs/research/](docs/research/) for literature review, competitor analysis, novelty analysis, and verified bibliography.

---

## Evaluation

Planned evaluation includes deterministic replay, tamper detection (cryptographic verification), and export validation. See [docs/evaluation/](docs/evaluation/) for demo plan, success criteria, judge Q&A, and limitations.

---

## Planned Demo Flow

The following CLI commands are **illustrative / planned** — not yet implemented:

```bash
# 1. Load synthetic sample case
jockey load sample_case/

# 2. Show causal graph
jockey graph show

# 3. Run hypothesis-driven investigation
jockey run investigate.jky

# 4. Compile to multiple backends
jockey compile investigate.jky
# → hunt.vql (Velociraptor)
# → kape_targets.json (KAPE)
# → evidence.case.json (CASE/UCO)

# 5. Export narrative report
jockey narrative

# 6. Verify integrity (tamper detection)
jockey verify
```

---

## Disclaimer

JOCKY is a research/prototype project for SIH 2026 Problem Statement 26148.

- No claim of legal admissibility — provenance chain designed for verifiability; legal admissibility depends on jurisdiction and procedure
- No guarantee of detection evasion or replacement of established DFIR tools
- No offensive capabilities — this project focuses on defensive forensic analysis, evidence collection, verification, and controlled investigation

---

**Repository:** [JOKEY---SIH26148](https://github.com/RishiSakhija/JOKEY---SIH26148)  
**License:** MIT — See [LICENSE](LICENSE)  
**Security:** See [SECURITY.md](SECURITY.md)