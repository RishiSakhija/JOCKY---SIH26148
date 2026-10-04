# Architecture Overview

**Status**: SPECIFIED

---

## System Purpose

JOCKY is an **evidence-first forensic workflow language and runtime** that unifies the investigation lifecycle from investigator intent to verified report.

---

## High-Level Architecture

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

---

## Core Principles

| Principle | Implementation |
|-----------|----------------|
| **Evidence-First** | Every artifact typed, hashed, signed, provenance-tracked by default |
| **Controlled Execution** | Execution Contract declares capabilities; runtime enforces least privilege |
| **Deterministic Correlation** | Rule-based edges with explicit evidence citations (no ML) |
| **Verifiable Replay** | Same script + same target → bit-for-bit identical evidence bundle |
| **Cross-Platform Parity** | Same artifact semantics on Windows/Linux |
| **Court-Ready Export** | CASE/UCO + narrative report with clickable citations |

---

## Component Boundaries

| Layer | Crate | Responsibility | Public API |
|-------|-------|----------------|------------|
| **Language** | `jockey-language` | Parser, AST, type checker, diagnostics | `parse()`, `type_check()`, `lower_to_ir()` |
| **Core** | `jockey-core` | IR, Execution Contract, Policy, Storage | `IrModule`, `ExecutionContract`, `PolicyEngine` |
| **Runtime** | `jockey-runtime` | Execution engine, dispatcher, sandbox | `Runtime::execute(contract)` |
| **Evidence** | `jockey-evidence` | Evidence, Receipt, Provenance, Finding, Export | `Evidence`, `EvidenceReceipt`, `ProvenanceRecord` |
| **Collectors** | `jockey-collectors` | Collector trait, Win/Lin implementations | `Collector::execute()`, `CollectorRegistry` |
| **Correlation** | `jockey-correlation` | Rules engine, graph builder, query API | `Correlator::correlate()`, `InvestigationGraph` |
| **API** | `jockey-api` | Axum HTTP server, WebSocket, OpenAPI | `serve()`, `routes()` |

---

## Data Flow

```
JOCKY Script (DSL)
       │
       ▼
┌──────────────────┐
│  pest Parser     │ ◀── Grammar: docs/language/grammar.md
└────────┬─────────┘
         │ AST
         ▼
┌──────────────────┐
│  Type Checker    │ ◀── Semantic validation, capability inference
└────────┬─────────┘
         │ Typed AST
         ▼
┌──────────────────┐
│  IR Compiler     │ ◀── Platform-agnostic execution plan
└────────┬─────────┘
         │ IR (Forensic Intermediate Representation)
         ▼
┌──────────────────┐
│  Contract Builder│ ◀── Binds IR to Target + Policy
└────────┬─────────┘
         │ Execution Contract
         ▼
┌──────────────────┐
│  Policy Engine   │ ◀── Capability validation, allowlist, bounds
└────────┬─────────┘
         │ Validated Contract
         ▼
┌──────────────────┐
│  Runtime Engine  │ ◀── Controlled dispatch, timeout, retry
└────────┬─────────┘
         │ Evidence Objects
         ▼
┌──────────────────┐
│  Evidence Model  │ ◀── Typed, hashed, signed, receipted
└────────┬─────────┘
         │ Evidence + Receipts
         ▼
┌──────────────────┐
│  Provenance Chain│ ◀── Hash-linked append-only log
└────────┬─────────┘
         │ Provenance Records
         ▼
┌──────────────────┐
│  Correlation     │ ◀── Deterministic rules → petgraph
└────────┬─────────┘
         │ Investigation Graph
         ▼
┌──────────────────┐
│  Verification    │ ◀── Receipt sigs + provenance hashes
└────────┬─────────┘
         │ Verified Graph
         ▼
┌──────────────────┐
│  Export          │ ◀── CASE/UCO, Cytoscape, Timesketch, Markdown
└──────────────────┘
```

---

## Security Architecture

### Execution Contract as Security Boundary

The **Execution Contract** is the security boundary between intent and action:

```json
{
  "ir_id": "ULID",
  "run_id": "ULID",
  "target": { "type": "live", "identifier": "HOST-01", "os": "windows", "arch": "x86_64" },
  "steps": [
    { "step_id": "ULID", "collector": "windows_evt", "action": "query", "params": {...}, "timeout_ms": 30000 }
  ],
  "policy": {
    "parallelism": 2,
    "continue_on_error": false,
    "evidence_root": "C:\\jocky\\evidence\\<run_id>",
    "hash_algorithm": "blake3"
  }
}
```

### Policy Enforcement Points

1. **Collector Allowlist** — Only registered collectors for target OS
2. **Parameter Schema Validation** — JSON Schema per collector/action
3. **Capability Bounds** — Script declares capabilities; runtime enforces
4. **Timeout Bounds** — Min 1s, max 300s per step
5. **Evidence Root Isolation** — Chroot-style path restriction
6. **Hash Algorithm Pinning** — blake3 or SHA-256 only
7. **Collector Version Pinning** — Exact version in contract

### Evidence Integrity

| Property | Mechanism |
|----------|-----------|
| **Authenticity** | Ed25519 signature by collector identity key |
| **Integrity** | blake3 hash binds receipt to exact evidence bytes |
| **Non-repudiation** | Collector cannot deny producing this receipt |
| **Freshness** | Timestamp + ULID prevent replay |
| **Traceability** | Hash-linked provenance chain from collection to export |

---

## Deployment Models

| Model | Description | Status |
|-------|-------------|--------|
| **Single Binary (MVP)** | One `jockey` binary runs API + Runtime + Collectors | SPECIFIED |
| **Server + Agents (Future)** | Central server manages remote collectors via gRPC | PLANNED |
| **Air-Gapped (Future)** | Offline binary + pre-compiled collector bundles | PLANNED |

---

## Technology Stack

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

## Related Documents

- `system-architecture.md` — Detailed component architecture
- `data-flow.md` — Evidence pipeline and data flow
- `execution-contract.md` — Execution Contract specification
- `collector-architecture.md` — Collector trait and implementations
- `correlation-architecture.md` — Correlation rules and graph
- `deployment-model.md` — Deployment topology and models
- `evidence-pipeline.md` — Evidence pipeline details