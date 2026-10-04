# JOCKY

**Evidence-Contract Forensics Platform**

---

## 1. Problem

Digital forensics and incident response (DFIR) today is fragmented across specialized tools that do not interoperate: Velociraptor for live hunting, KAPE for disk triage, Volatility for memory analysis, Plaso for timeline generation, Timesketch for collaborative analysis. Investigators must manually translate hypotheses between incompatible query languages, data formats, and collection paradigms. There is no single workflow that captures *intent*, executes *controlled collection*, produces *tamper-evident evidence*, and reconstructs a *causal attack narrative* with verifiable provenance.

---

## 2. Operational Context

- **SIH Problem Statement 26148**: "Creation of scripts/functions with a new programming language/framework 'JOCKY' for computer and network forensic analysis, including cross-platform Windows/Ubuntu operation, centralized multi-system analysis, automated forensic functions, and security-resilient execution."
- **Target users**: DFIR analysts, threat hunters, incident responders, forensic examiners
- **Environments**: Live endpoints (Windows/Linux), disk images, memory dumps, cloud snapshots, network captures
- **Constraints**: Forensic soundness, chain of custody, reproducibility, court-admissible evidence handling

---

## 3. Why Existing Approaches Are Fragmented

| Tool | Strength | Gap for Unified Investigation |
|------|----------|-------------------------------|
| Velociraptor | Live hunting, VQL artifacts, fleet management | Host-only; no native memory/network/cloud compilation; no built-in causal graph |
| KAPE | Windows triage speed, target/module abstraction | Windows-only; config-driven (no query language); no live/remote |
| osquery | Universal SQL interface to OS state | Snapshot-only; no procedural collection; no timeline engine |
| Volatility 3 | Deep memory analysis with symbols | Memory-only; offline; no query language; no collection |
| Plaso/log2timeline | 1000+ parser formats, super timelines | Batch-only; no live collection; no query language |
| Timesketch | Collaborative timeline analysis | Post-hoc only; no collection; requires external pipeline |
| GRR | Approval-based remote live forensics | Heavy infrastructure; Python flows not a query language |

**Result**: Investigators stitch together 5–7 tools per case. Hypothesis → collection → correlation → reporting is manual, error-prone, and non-reproducible.

---

## 4. Our Solution

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
    → Dashboard / Report (court-ready, citation-backed)
```

---

## 5. Core USP: Evidence-Contract Forensics

JOCKY does **not** claim to invent:
- Forensic query languages (VQL, SQL, KQL exist)
- Cross-platform collection (Velociraptor, osquery, GRR exist)
- Timeline generation (Plaso, Timesketch exist)
- Evidence provenance (ProvQL, DFIR-ORC exist)
- Investigation graphs (Timesketch, Maltego exist)

**JOCKY's differentiation is the specific integrated architecture:**

| Integration Point | What JOCKY Unifies |
|-------------------|-------------------|
| **Language → Collectors** | One script compiles to Velociraptor VQL, osquery SQL, KAPE targets, Volatility plugins, CASE/UCO bundle |
| **Intent → Execution Contract** | Script declares required capabilities; runtime validates and enforces least privilege |
| **Collection → Evidence Receipt** | Every artifact produces a cryptographically signed receipt at collection time |
| **Evidence → Provenance** | Hash-linked chain from raw collection through every transformation |
| **Evidence → Correlation** | Deterministic rules produce cited edges, not opaque scores |
| **Graph → Verification** | Tamper detection via receipt + provenance re-validation |
| **Graph → Report** | Narrative export with clickable evidence citations |

**Single declarative script → multiple native backends → complete signed evidence bundle → causal graph → verified report.**

---

## 6. Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            JOCKY ARCHITECTURE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────┐  │
│  │   JOCKY      │    │   JOCKY      │    │   JOCKY      │    │  JOCKY   │  │
│  │   LANGUAGE   │───▶│   COMPILER   │───▶│   RUNTIME    │───▶│ COLLECTORS│  │
│  │  (DSL/parser)│    │  (AST→IR)    │    │ (execution)  │    │ (Win/Lin)│  │
│  └──────────────┘    └──────────────┘    └──────┬───────┘    └──────────┘  │
│                                                  │                           │
│                                                  ▼                           │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────┐  │
│  │  EVIDENCE    │◀───│  PROVENANCE  │◀───│   CORRELATION│◀───│  COMMON  │  │
│  │   MODEL      │    │   CHAIN      │    │   ENGINE     │    │ EVIDENCE  │  │
│  └──────────────┘    └──────────────┘    └──────┬───────┘    └──────────┘  │
│                                                  │                           │
│                                                  ▼                           │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │ INVESTIGATION│◀───│ VERIFICATION │◀───│   EXPORT     │                  │
│  │    GRAPH     │    │   ENGINE     │    │ (CASE/UCO,   │                  │
│  └──────────────┘    └──────────────┘    │  TIMESKETCH) │                  │
│                                           └──────────────┘                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Core Crates:**
- `jockey-language` — Pest parser, AST, type checker
- `jockey-core` — IR, Execution Contract, policy engine
- `jockey-runtime` — Controlled execution, capability validation
- `jockey-evidence` — Common Evidence Model, Receipts, Provenance
- `jockey-collectors` — Collector trait, Windows/Linux implementations
- `jockey-correlation` — Deterministic correlation rules, petgraph
- `jockey-api` — Axum HTTP API, WebSocket for live updates

---

## 7. How JOCKY Works

### 7.1 JOCKY Script (DSL)

```jocky
investigation "apt29-lateral-movement" {
    // Declare hypothesis
    hypothesis "APT29 used WMI for lateral movement" {
        mitre: ["T1047", "T1547.001"]
    }

    // Collection (platform-agnostic)
    collect process as wmi_providers where command_line contains "wmic"
    collect process as spawns where parent_name == "wmiprvse.exe"
    collect registry as run_keys where path matches "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run*"
    collect network as outbound where destination_port in [443, 80, 135, 445]

    // Correlation (deterministic, evidence-cited)
    correlate wmi_providers -> spawns by "pid_link" as wmi_chain
    correlate wmi_chain -> run_keys by "timestamp_join" window 300s as persistence
    correlate persistence -> outbound by "process_netlink" as c2_beacon

    // Timeline & Hypothesis Binding
    timeline "lateral_movement" from wmi_chain, persistence, c2_beacon
    bind hypothesis "APT29 used WMI for lateral movement" to wmi_chain, persistence

    // Output
    export case "apt29-case" format case_uco
    export graph "apt29-graph" format cytoscape
    export report "apt29-report" format markdown
}
```

### 7.2 Execution Flow

1. **Parse** → AST with semantic validation (types, scope, capability requirements)
2. **Compile** → Platform-agnostic IR (ordered steps, dependencies, outputs)
3. **Contract** → Execution Contract binds IR to target (live/dead/remote), injects policy (timeouts, parallelism, hash algorithm, evidence root)
4. **Validate** → Policy engine checks capabilities against collector manifests; rejects unauthorized operations
5. **Execute** → Runtime dispatches to collectors; each evidence piece hashed, signed, receipted immediately
6. **Correlate** → Deterministic rules join evidence by PID, timestamp, path, network flow
7. **Graph** → Causal graph built in petgraph; nodes=evidence, edges=correlations with confidence
8. **Verify** → Receipt signatures validated; provenance chain hash-checked; tamper detection
9. **Export** → CASE/UCO JSON-LD, Cytoscape.js graph, Markdown report, Timesketch CSV

---

## 8. Execution Contract

The **Execution Contract** is the security boundary between intent and action:

```json
{
  "ir_id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "run_id": "01ARZ3NDEKTSV4RRFFQ69G5FAZ",
  "target": { "type": "live", "identifier": "WORKSTATION-01", "os": "windows", "arch": "x86_64" },
  "steps": [
    { "step_id": "...", "collector": "windows_evt", "action": "query", "params": {...}, "timeout_ms": 30000 },
    { "step_id": "...", "collector": "windows_reg", "action": "enumerate", "params": {...}, "timeout_ms": 10000 }
  ],
  "policy": {
    "parallelism": 2,
    "continue_on_error": false,
    "evidence_root": "C:\\jocky\\evidence\\01ARZ3NDEKTSV4RRFFQ69G5FAZ",
    "hash_algorithm": "blake3"
  }
}
```

**Capabilities enforced:**
- Collector allowlist per target OS
- Parameter schema validation
- Timeout and retry bounds
- Evidence root isolation (no path traversal)
- Hash algorithm pinning (blake3/SHA-256)
- Collector version pinning

---

## 9. Evidence Receipt

Every evidence object produces a **cryptographic receipt** at collection time:

```json
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FB1",
  "evidence_id": "01ARZ3NDEKTSV4RRFFQ69G5FB0",
  "run_id": "01ARZ3NDEKTSV4RRFFQ69G5FAZ",
  "collector": "windows_evt",
  "collector_version": "0.1.0",
  "signature": "MEUCIQD...",
  "public_key": "a1b2c3d4e5f6...",
  "payload_hash": "a1b2c3d4e5f6...",
  "payload_hash_algo": "blake3",
  "timestamp": "2026-10-04T12:00:05Z",
  "host_info": { "hostname": "WORKSTATION-01", "os": "Windows 10 22H2", "kernel": "10.0.19045", "collector_pid": 4216 }
}
```

**Properties:**
- Ed25519 signature by collector identity key
- Payload hash binds receipt to evidence bytes
- Collector version pinned for reproducibility
- Host info provides collection context
- ULIDs throughout for sortable, coordination-free IDs

---

## 10. Correlation / Investigation Graph

**Deterministic Evidence Correlation** — not ML, not heuristic scoring.

| Relation | Source | Target | Method | Evidence |
|----------|--------|--------|--------|----------|
| `spawned` | Process A | Process B | PID/PPID link | Process create events |
| `wrote` | Process | File | Handle correlation | File create + process |
| `connected` | Process | NetworkConnection | PID + 4-tuple | Netstat/ETW + process |
| `resolved_to` | NetworkConnection | Indicator | IOC match | Threat intel feed |
| `indicates` | Evidence[] | Finding | Rule match | Detection rule + evidence IDs |
| `supported_by` | Finding | Evidence[] | Citation | Explicit evidence IDs |

**Graph Query Examples:**
- `graph.path(process_A, process_Z, max_hops=5)` → causal chain
- `graph.backward_from(ioc, window=1h)` → precursor events
- `graph.hypothesis_support("T1047")` → all evidence citing WMI lateral movement

---

## 11. Windows + Linux Support

| Collector Category | Windows | Linux |
|--------------------|---------|-------|
| Process | `proc_windows` (ETW, WMI, PSAPI) | `proc_linux` (/proc, auditd, sysfs) |
| File | `file_windows` (NTFS, USN, VSS) | `file_linux` (ext4/xfs, fanotify) |
| Registry/Config | `registry_windows` (native hive) | `config_linux` (/etc, systemd, journald) |
| Logs | `logs_windows` (EVTX, ETW) | `logs_linux` (journald, syslog, auditd) |
| Network | `net_windows` (ETW, netsh) | `net_linux` (conntrack, bpftrace, ss) |

**Linux eBPF Direction (Post-MVP):**
- Aya-based eBPF collectors for syscall tracing, socket monitoring, file integrity
- Documented as extensible architecture; not implemented in MVP
- Kernel version compatibility matrix maintained

---

## 12. Tech Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Language Runtime | **Rust** | Memory safety, performance, WASM target, ecosystem |
| Parser | **pest** | PEG grammar, precedence climbing, good error messages |
| Graph | **petgraph** | Pure Rust, flexible graph algorithms, no external deps |
| Crypto | **blake3**, **ed25519-dalek** | Fast hashing, small signatures, constant-time verify |
| Serialization | **serde**, **serde_json** | Ubiquitous, zero-copy deserialization |
| Database | **SQLite** (rusqlite) | Embedded, ACID, JSON1 extension, portable |
| IDs | **ulid** | Monotonic, sortable, 128-bit, no coordination |
| Time | **chrono** (UTC only) | Timezone-aware, RFC3339 |
| API | **Axum** + **Tower** | Async, middleware, OpenAPI generation |
| Frontend | **HTML/CSS/JS** + **Chart.js** + **Cytoscape.js** | Zero-build, portable, works offline |
| Testing | **cargo test**, **proptest** | Property-based, unit, integration |

---

## 13. MVP Scope

### Implemented in Specification (This Repository)
- [x] Complete DSL grammar and semantics
- [x] AST and IR specification
- [x] Execution Contract schema and policy model
- [x] Common Evidence Model (7 evidence types)
- [x] Evidence Receipt (Ed25519, blake3)
- [x] Provenance Record (hash-linked chain)
- [x] Collector Interface trait definition
- [x] 9 MVP Collector specifications
- [x] Deterministic Correlation Rules (6 relations)
- [x] Finding Object with evidence citations
- [x] Investigation Graph model (petgraph)
- [x] Export formats: CASE/UCO, Cytoscape, Timesketch, Markdown
- [x] Architecture diagrams (12 Mermaid diagrams)
- [x] Research documentation (11 tools analyzed)
- [x] Novelty analysis and differentiation
- [x] Evaluation plan and demo flow
- [x] Complete audit report (honest status)
- [x] Handoff document for implementers

### NOT Implemented (Deferred to Implementation Phases)
- [ ] Pest parser implementation
- [ ] AST/IR Rust types and compiler
- [ ] Runtime execution engine
- [ ] Windows collectors (native Rust + windows-sys)
- [ ] Linux collectors (userspace + Aya/eBPF stubs)
- [ ] Correlation engine
- [ ] Axum API server
- [ ] Frontend dashboard
- [ ] SQLite persistence layer
- [ ] CLI tooling
- [ ] Integration tests

---

## 14. Demo Flow (SIH Judging)

1. **Load Sample** — `jockey load sample_case/` reads pre-parsed Plaso CSV
2. **Show Graph** — `jockey graph show` displays causal chain (process → network → file → registry)
3. **Run DSL** — `jockey run investigate.jky` executes hypothesis-driven script
4. **Compile** — `jockey compile investigate.jky` emits:
   - `hunt.vql` (Velociraptor)
   - `kape_targets.json` (KAPE)
   - `evidence.case.json` (CASE/UCO with provenance)
5. **Export Narrative** — `jockey narrative` → `evidence_report.md`
6. **Verify Integrity** — Display hash chain; confirm deterministic replay
7. **Tamper Demo** — Modify evidence artifact → verification fails → restore → verify passes
8. **Close** — "One script. Three backends. Complete evidence chain. Zero post-processing."

---

## 15. Research Foundation

**11 primary tools analyzed** with verified sources:
- Velociraptor (VQL, artifacts, live hunting)
- osquery (SQL, virtual tables, extensions)
- GRR (flows, approval workflows, VFS)
- KAPE (targets/modules, Windows triage)
- Volatility 3 (memory plugins, symbol tables)
- Sysmon (kernel telemetry, ETW)
- Wazuh (XDR, rules, agents)
- Fleet (osquery management, GitOps)
- Timesketch (collaborative timelines)
- Plaso/log2timeline (1000+ parsers)
- Eric Zimmerman Tools (Windows artifact parsers)

**Key research documents:**
- `docs/research/literature-review.md` — Academic and industry sources
- `docs/research/competitor-analysis.md` — Detailed comparison matrix
- `docs/research/novelty-analysis.md` — Explicit differentiation
- `docs/research/research-mapping.md` — Architecture decision tracing
- `docs/research/sources.md` — Verified bibliography

---

## 16. Current Status

**Phase 0: Repository / Specification** ✅ **COMPLETE**

This repository contains **specifications, architecture, research, and documentation only**. No implementation code exists yet.

| Component | Status | Notes |
|-----------|--------|-------|
| JOCKY DSL Grammar | SPECIFIED | Pest grammar in `docs/language/grammar.md` |
| AST Definition | SPECIFIED | `docs/language/ast.md` |
| IR Specification | SPECIFIED | `docs/architecture/system-architecture.md` |
| Execution Contract | SPECIFIED | `docs/architecture/execution-contract.md` |
| Evidence Model | SPECIFIED | `docs/evidence/evidence-model.md` |
| Evidence Receipt | SPECIFIED | `docs/evidence/evidence-receipt.md` |
| Provenance Chain | SPECIFIED | `docs/evidence/provenance.md` |
| Collector Interface | SPECIFIED | `docs/architecture/collector-architecture.md` |
| Correlation Rules | SPECIFIED | `docs/architecture/correlation-architecture.md` |
| Investigation Graph | SPECIFIED | `docs/architecture/data-flow.md` |
| Windows Collectors | NOT IMPLEMENTED | Specs in `collectors/windows/README.md` |
| Linux Collectors | NOT IMPLEMENTED | Specs in `collectors/linux/README.md` |
| Runtime Engine | NOT IMPLEMENTED | Architecture only |
| Dashboard/Frontend | NOT IMPLEMENTED | `frontend/README.md` placeholder |

---

## 17. Roadmap

| Phase | Focus | Target |
|-------|-------|--------|
| **Phase 0** | Repository, Specifications, Research | ✅ Complete |
| **Phase 1** | DSL Parser, AST, IR Compiler | Week 1-2 |
| **Phase 2** | Runtime, Execution Contract, Policy | Week 2-3 |
| **Phase 3** | Evidence Model, Receipts, Provenance | Week 3-4 |
| **Phase 4** | Windows Collectors (5), Linux Collectors (4) | Week 4-6 |
| **Phase 5** | Correlation Engine, Investigation Graph | Week 6-7 |
| **Phase 6** | Axum API, Frontend Dashboard | Week 7-8 |
| **Phase 7** | Verification, Tamper Detection, Export | Week 8-9 |
| **Phase 8** | SIH Demo Polish, Documentation | Week 9-10 |

See `ROADMAP.md` for detailed milestones.

---

## 18. Limitations

**Honest disclosure of current constraints:**

1. **No implementation exists** — This is a specification repository only
2. **No runtime validation** — Architecture untested against real collectors
3. **eBPF not implemented** — Documented as future direction only
4. **No court admissibility claim** — Provenance chain designed for verifiability; legal admissibility depends on jurisdiction and procedure
5. **No AI/ML correlation** — Deterministic rules only; AI assist explicitly deferred
6. **No multi-tenancy/RBAC** — Single-analyst model for MVP
7. **No remote agent protocol** — Local execution only; remote via SSH/WinRM deferred
8. **Performance uncharacterized** — blake3 chosen for speed; no benchmarks yet
9. **Windows kernel drivers not used** — Userspace collectors only for MVP; kernel mode deferred
10. **CASE/UCO mapping incomplete** — Core classes mapped; full ontology deferred

---

## 19. Team / Contribution

**SIH 2026 Team — JOCKY (Problem Statement 26148)**

| Role | Responsibility |
|------|----------------|
| Lead Architect | Language design, IR, compiler, architecture |
| Runtime Engineer | Execution engine, capability validation, sandboxing |
| Collector Engineer | Windows/Linux collectors, native APIs, eBPF |
| Evidence Engineer | Evidence model, receipts, provenance, verification |
| Correlation Engineer | Deterministic rules, graph engine, hypothesis tracking |
| Frontend Engineer | Dashboard, graph visualization, timeline, reports |
| Research Lead | Competitor analysis, academic mapping, novelty |

**Contribution guidelines:** See `CONTRIBUTING.md`  
**Security reporting:** See `SECURITY.md`  
**Code of conduct:** Rust community standards apply

---

## 20. License

**MIT License** — See `LICENSE` file.

---

**Repository:** `JOCKEY---SIH26148`  
**Status:** Specification Phase Complete — Ready for Implementation  
**Last Updated:** October 2026