# Novelty Analysis

**Status**: SPECIFIED

---

## Purpose

Explicitly distinguish what is **NOT novel** from JOCKY's **genuine differentiation**.

---

## What is NOT Novel

| # | Capability | Existing Tools | Why Not Novel |
|---|------------|----------------|---------------|
| 1 | **Forensic query language** | Velociraptor VQL (2019+), osquery SQL (2014+), KQL (2019+), ES|QL (2023+), Sigma (2017+) | Mature languages with ecosystems |
| 2 | **Cross-platform collection** | Velociraptor, osquery, GRR, Wazuh, Fleet, Sysmon | Table stakes for modern DFIR |
| 3 | **Centralized multi-system management** | Velociraptor (server+hunts), GRR (server+approvals), Wazuh (manager+agents), Fleet (GitOps+400k hosts) | Solved at scale |
| 4 | **Automated artifact parsing** | KAPE (targets/modules), Plaso (1000+ parsers), Velociraptor artifacts, Zimmerman tools | Best-in-class exists |
| 5 | **Timeline generation** | Plaso (2009+), Timesketch (2016+), Velociraptor notebooks | Industry standard |
| 6 | **Evidence provenance** | Provexa/ProvQL, DFIR-ORC, Velociraptor notebooks, CASE/UCO | Existing implementations |
| 7 | **Investigation graph** | Timesketch, Maltego, KQL, Graph Explorer | Graph analysis exists |
| 8 | **Report/Dashboard** | Velociraptor notebooks, GRR flows, Timesketch, KAPE reports | Visualization solved |
| 9 | **Reproducible workflows** | Velociraptor artifacts/VQL, GRR flows, KAPE targets, Ansible | Automation exists |
| 10 | **Sigma/YARA integration** | Velociraptor, Wazuh, Chainsaw, Hayabusa, Timesketch analyzers | Standard integration |
| 11 | **Live response/remote shell** | Velociraptor (shell), GRR (interactive), Wazuh (active response) | Remote access exists |
| 12 | **Offline/air-gapped collection** | Velociraptor offline collector, KAPE, GRR air-gap flows | Supported |
| 13 | **AI/ML assistance** | Elastic (Attack Discovery), Splunk (MLTK), DFIR Companion, Clouseau | AI in DFIR exists |
| 14 | **Hashing/signatures** | SHA-256, Ed25519, blake3, GPG | Cryptographic primitives |
| 15 | **CASE/UCO export** | Plaso, Velociraptor, GRR, Timesketch | Standard export |

---

## What is PARTIALLY Differentiating

| # | Capability | Partial Differentiation | JOCKY Improvement |
|---|------------|------------------------|-------------------|
| 1 | **Execution Contracts** | GRR has approval workflows; Velociraptor has artifact parameters | Capability-based, least-privilege, policy-bound, collector allowlist, timeout bounds |
| 2 | **Evidence Receipts** | Provexa has provenance; Velociraptor has artifact signing | Per-evidence Ed25519 receipt at collection time + host context |
| 3 | **Deterministic Correlation** | Timesketch analyzers; KQL joins | Rule-based with explicit evidence citations, confidence, method |
| 4 | **Cross-Platform Abstraction** | Velociraptor/osquery/GRR have platform-specific artifacts | Semantic types (`process`, `file`) to platform-specific collectors |
| 5 | **Compilation Targets** | Some tools export to other formats | Single IR to VQL + SQL + KAPE + Volatility + CASE + Cytoscape + Timesketch |
| 6 | **Hypothesis Tracking** | Timesketch sketches; GRR flow parameters | First-class hypothesis with MITRE tags, confidence, evidence binding |

---

## OUR DIFFERENTIATION: Evidence-Contract Forensics

> **JOCKY integrates forensic workflow definition, controlled execution, evidence receipts, provenance, deterministic correlation, and verified investigation output into one traceable workflow.**

### The Specific Integrated Architecture

```
JOCKY Script (declarative, hypothesis-driven)
       |
       v
Lexer/Parser (pest) -> AST (typed, validated) -> IR (platform-agnostic)
       |
       v
Execution Contract (capability-validated, policy-bound)
       |
       v
Rust Runtime (controlled, sandboxed execution)
       |
       v
Windows/Linux Collectors (native, capability-scoped)
       |
       v
Common Evidence Model (typed, hashed, signed)
       |
       v
Evidence Receipt (Ed25519-signed, collector-attested)
       |
       v
Provenance Chain (hash-linked, append-only)
       |
       v
Deterministic Correlation (rule-based, evidence-cited)
       |
       v
Investigation Graph (causal, time-travel capable)
       |
       v
Verification (receipt + provenance validation)
       |
       v
Dashboard / Report (court-ready, citation-backed)
```

### Integration Points (The Real Novelty)

| Integration | What JOCKY Unifies | No Other Tool Does This |
|-------------|-------------------|------------------------|
| **Language to Collectors** | One script compiles to Velociraptor VQL, osquery SQL, KAPE targets, Volatility plugins, CASE/UCO bundle | Yes |
| **Intent to Execution Contract** | Script declares required capabilities; runtime validates and enforces least privilege | Yes |
| **Collection to Evidence Receipt** | Every artifact produces cryptographically signed receipt at collection time | Yes |
| **Evidence to Provenance** | Hash-linked chain from raw collection through every transformation | Yes |
| **Evidence to Correlation** | Deterministic rules produce cited edges, not opaque scores | Yes |
| **Graph to Verification** | Tamper detection via receipt + provenance re-validation | Yes |
| **Graph to Report** | Narrative export with clickable evidence citations | Yes |

### Single Declarative Script to Multiple Native Backends to Complete Signed Evidence Bundle to Causal Graph to Verified Report

---

## What We Do NOT Claim

| Claim | Status | Reason |
|-------|--------|--------|
| "First forensic DSL" | FALSE | VQL, osquery, KQL, ES|QL exist |
| "First cross-platform forensics" | FALSE | Velociraptor, osquery, GRR exist |
| "First provenance system" | FALSE | Provexa, CASE/UCO exist |
| "First investigation graph" | FALSE | Timesketch, Maltego exist |
| "First dashboard" | FALSE | Velociraptor, Timesketch, Kibana exist |
| "Automatic court admissibility" | FALSE | Legal admissibility depends on jurisdiction/procedure |
| "100% secure" | FALSE | No system is 100% secure |
| "Zero detection" | FALSE | Not a stealth tool |
| "Guaranteed stealth" | FALSE | Not an offensive tool |
| "AI-powered forensics" | FALSE | Deterministic only; AI assist deferred |
| "Replaces Velociraptor/KAPE/Volatility" | FALSE | JOCKY compiles TO them |

---

## Honest Positioning Statement

> **JOCKY is the only DFIR language that compiles a single investigation script into native collectors across Velociraptor, osquery, KAPE, and Volatility while emitting a complete, signed CASE/UCO evidence bundle with built-in provenance types -- no post-processing required.**

---

## Related Documents

- `research-overview.md` — Research categories
- `literature-review.md` — Verified research sources
- `competitor-analysis.md` — Tool comparison
- `research-to-design.md` — Research to architecture mapping
- `sources.md` — Bibliography