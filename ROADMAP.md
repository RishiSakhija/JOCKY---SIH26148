# JOCKY Roadmap

**Status**: SPECIFIED

---

## Current Phase

**PHASE 0 — Repository + Research + Specification** ✅ **COMPLETE**

---

## Phase Overview

| Phase | Focus | Duration | Status |
|-------|-------|----------|--------|
| **Phase 0** | Repository, Specifications, Research | Week 0 | ✅ Complete |
| **Phase 1** | JOCKY DSL (Parser, AST, Type Checker, IR) | Week 1-2 | 🔄 Next |
| **Phase 2** | IR + Execution Contract + Policy Engine | Week 2-3 | ⏳ Planned |
| **Phase 3** | Evidence Model + Receipts + Provenance | Week 3-4 | ⏳ Planned |
| **Phase 4** | Windows Collectors (5) + Linux Collectors (5) | Week 4-6 | ⏳ Planned |
| **Phase 5** | Correlation Engine + Investigation Graph | Week 6-7 | ⏳ Planned |
| **Phase 6** | Axum API + Frontend Dashboard | Week 7-8 | ⏳ Planned |
| **Phase 7** | Verification + Tamper Detection + Export | Week 8-9 | ⏳ Planned |
| **Phase 8** | SIH Demo Polish + Documentation | Week 9-10 | ⏳ Planned |

---

## Phase 0 — Repository + Research + Specification ✅ COMPLETE

### Deliverables
- [x] Repository structure (crates, collectors, docs, assets, examples)
- [x] Cargo workspace with 7 crates
- [x] GitHub CI/CD (fmt, check, clippy, test, docs, audit, deny)
- [x] Rust toolchain, rustfmt, clippy configs
- [x] README.md (judge-facing)
- [x] JOCKY_SPEC_SHEET.md (engineering source of truth)
- [x] COMPLETE_AUDIT_REPORT.md (honest audit)
- [x] HANDOFF.md (implementation guide)
- [x] ROADMAP.md (this file)
- [x] CONTRIBUTING.md, SECURITY.md, CHANGELOG.md, LICENSE
- [x] All documentation in docs/ (architecture, language, evidence, research, evaluation)
- [x] 14 Mermaid diagrams in assets/diagrams/
- [x] Research: literature-review, competitor-analysis, novelty-analysis, research-to-design, sources
- [x] Collector specs (Windows 5, Linux 5)
- [x] Evaluation: plan, success-criteria, demo-plan, judge-questions, limitations, risk-register
- [x] DATASETS.md, RUN_AND_EVALUATE.md, COMPLETE_AUDIT_REPORT.md, HANDOFF.md

### Architecture Decisions Locked
- Pipeline, tech stack, evidence types, collectors, correlation rules, DSL statements, crypto primitives
- Documented in JOCKY_SPEC_SHEET.md

---

## Phase 1 — JOCKY DSL (Week 1-2) 🔄 NEXT

### Goals
- Implement pest parser for JOCKY grammar
- Build AST with all 10 statement types
- Implement type checker with capability inference
- Lower typed AST to IR
- Comprehensive parser/typer tests

### Deliverables
- [ ] `crates/jockey-language/src/parser.pest` — Pest grammar
- [ ] `crates/jockey-language/src/parser.rs` — Parser + CST->AST
- [ ] `crates/jockey-language/src/ast.rs` — AST node definitions
- [ ] `crates/jockey-language/src/typeck.rs` — Type checker + capability inference
- [ ] `crates/jockey-language/src/ir.rs` — IR lowering
- [ ] `crates/jockey-language/src/diagnostics.rs` — Error reporting
- [ ] `crates/jockey-language/build.rs` — Pest grammar generation
- [ ] Unit tests: parser roundtrip, AST construction
- [ ] Property tests: proptest for parser/typer
- [ ] Golden tests: example scripts -> AST -> IR
- [ ] Integration test: example scripts from docs/language/examples.md

### Dependencies
- pest 2.7, pest_derive, pest_generator
- thiserror, anyhow, serde, ulid, chrono

### Success Criteria
- All 10 example scripts parse without errors
- Type checker infers correct capabilities
- IR output matches golden files
- `cargo test -p jockey-language` passes 100%

---

## Phase 2 — IR + Execution Contract + Policy (Week 2-3)

### Goals
- Define IR types and validation
- Build Execution Contract from IR + Target + Policy
- Implement Policy Engine with capability validation
- SQLite storage layer

### Deliverables
- [ ] `crates/jockey-core/src/ir.rs` — IR types (IrModule, IRStep, IROutput, IRMetadata)
- [ ] `crates/jockey-core/src/contract.rs` — Contract builder (IR + Target -> Contract)
- [ ] `crates/jockey-core/src/policy.rs` — PolicyEngine (7 enforcement points)
- [ ] `crates/jockey-core/src/target.rs` — TargetSpec types
- [ ] `crates/jockey-core/src/storage.rs` — SQLite schema + CRUD
- [ ] `crates/jockey-core/src/capability.rs` — Capability types + inference
- [ ] Unit tests: IR validation, contract building, policy validation
- [ ] Integration tests: IR -> Contract -> Policy validation
- [ ] Golden tests: contract serialization

### Policy Enforcement Points (7)
1. Collector allowlist per target OS
2. Parameter schema validation per collector/action
3. Timeout bounds (1s-300s)
4. Evidence root isolation
5. Hash algorithm pinning (blake3/SHA-256)
6. Collector version pinning
7. Capability validation against manifests

### Success Criteria
- Contract building from IR + Target works
- Policy validation catches all 7 violation types
- SQLite CRUD operations work
- `cargo test -p jockey-core` passes 100%

---

## Phase 3 — Evidence Model + Receipts + Provenance (Week 3-4)

### Goals
- Evidence types with serialization
- Evidence Receipt signing/verification (Ed25519 + blake3)
- Provenance chain logic (hash-linked, signed)
- Verification engine
- Export formats (CASE/UCO, Cytoscape, Timesketch, Markdown)

### Deliverables
- [ ] `crates/jockey-evidence/src/model.rs` — Evidence, EvidenceReceipt, ProvenanceRecord, Finding, CorrelationEdge
- [ ] `crates/jockey-evidence/src/receipt.rs` — Sign/verify, host info
- [ ] `crates/jockey-evidence/src/provenance.rs` — Chain building, verification
- [ ] `crates/jockey-evidence/src/verify.rs` — VerificationEngine (full pipeline)
- [ ] `crates/jockey-evidence/src/export.rs` — CASE/UCO, Cytoscape, Timesketch, Markdown
- [ ] `crates/jockey-evidence/src/crypto.rs` — blake3, Ed25519, key management
- [ ] Unit tests: crypto, serialization, provenance chains
- [ ] Integration tests: evidence -> receipt -> provenance -> verify
- [ ] Tamper tests: positive (pass) + negative (fail detection)
- [ ] Export validation: CASE/UCO schema, Cytoscape render, Timesketch import

### Crypto Primitives
- blake3 for hashing (fast, parallel)
- Ed25519 for signatures (small keys, fast verify)
- ULID for all IDs
- UTC-only timestamps (chrono)

### Success Criteria
- Receipt signing/verification works
- Provenance chain builds and verifies
- Tamper detection catches all mutations
- All 4 export formats valid
- `cargo test -p jockey-evidence` passes 100%

---

## Phase 4 — Windows + Linux Collectors (Week 4-6)

### Goals
- Implement Collector trait
- 5 Windows collectors (windows-sys)
- 5 Linux collectors (nix/libc)
- Collector manifests with capabilities, schemas, keys

### Windows Collectors (5)
| Collector | Actions | APIs |
|-----------|---------|------|
| proc_windows | list, get, tree | Toolhelp32, PSAPI, WMI |
| file_windows | enumerate, hash, collect, mft, usn, vss | FindFile, USN, VSS |
| registry_windows | enumerate, get, monitor | RegOpenKeyEx, RegEnumValue |
| logs_windows | query, export, tail | EvtQuery, EvtExportLog |
| net_windows | connections, listen, capture | GetExtendedTcpTable, NDIS |

### Linux Collectors (5)
| Collector | Actions | Interfaces |
|-----------|---------|------------|
| proc_linux | list, get, tree | /proc, auditd |
| file_linux | enumerate, hash, collect, fanotify | libc, fanotify |
| config_linux | enumerate, get, systemd, journald | /etc, systemd, sd_journal |
| logs_linux | query, export, audit | /var/log, auditd |
| net_linux | connections, listen, capture, conntrack | /proc/net, libnetfilter_conntrack |

### Deliverables
- [ ] `crates/jockey-collectors/src/trait.rs` — Collector trait + Registry
- [ ] `crates/jockey-collectors/src/manifest.rs` — CollectorManifest + ActionManifest
- [ ] `crates/jockey-collectors/src/windows/` — 5 collectors
- [ ] `crates/jockey-collectors/src/linux/` — 5 collectors
- [ ] Collector manifests with capabilities, schemas, Ed25519 keys
- [ ] Contract tests: trait compliance for all 10 collectors
- [ ] Integration tests: mock execution with synthetic data
- [ ] Golden tests: output evidence format

### eBPF Architecture (Documented, Not Implemented)
- Aya-based eBPF collectors (proc_ebpf, file_ebpf, net_ebpf, cred_ebpf)
- BTF/CO-RE, kernel 5.10+, graceful fallback
- Documented in `collectors/linux/README.md`

### Success Criteria
- All 10 collectors implement trait correctly
- Manifests validate against schemas
- Evidence output matches golden files
- `cargo test -p jockey-collectors` passes 100%

---

## Phase 5 — Correlation + Investigation Graph (Week 6-7)

### Goals
- Correlation rule engine (9 deterministic rules)
- petgraph-based InvestigationGraph
- Graph queries (path, backward, forward, hypothesis)
- Hypothesis overlay

### Deliverables
- [ ] `crates/jockey-correlation/src/rules.rs` — Rule engine + config
- [ ] `crates/jockey-correlation/src/engine.rs` — Correlation logic
- [ ] `crates/jockey-correlation/src/graph.rs` — InvestigationGraph + queries
- [ ] `crates/jockey-correlation/src/hypothesis.rs` — Hypothesis overlay
- [ ] Unit tests: rule matching, graph construction
- [ ] Property tests: graph queries, path finding
- [ ] Golden tests: graph output for known cases
- [ ] Hypothesis confidence auto-update

### Graph Queries
- `path(from, to, max_hops)` — Causal chain
- `backward_from(node, window)` — Precursors
- `forward_from(node, window)` — Consequences
- `hypothesis_support(id)` — Evidence for hypothesis
- `subgraph(evidence_ids)` — Export subset
- `graph_at(timestamp)` — Time travel

### Success Criteria
- All 9 rules produce correct edges
- Graph queries return expected results
- Hypothesis overlay updates confidence
- `cargo test -p jockey-correlation` passes 100%

---

## Phase 6 — Axum API + Frontend Dashboard (Week 7-8)

### Goals
- Axum REST + WebSocket server
- Static frontend (8 pages, Cytoscape + Chart.js)
- Real-time updates via WebSocket

### API Endpoints
| Method | Path | Purpose |
|--------|------|---------|
| POST | /api/v1/investigations | Create from script |
| GET | /api/v1/investigations/{id} | Status |
| GET | /api/v1/investigations/{id}/graph | Cytoscape JSON |
| GET | /api/v1/investigations/{id}/timeline | Timeline events |
| GET | /api/v1/investigations/{id}/evidence | List evidence |
| GET | /api/v1/investigations/{id}/evidence/{eid} | Evidence + receipt + provenance |
| GET | /api/v1/investigations/{id}/findings | Findings list |
| POST | /api/v1/investigations/{id}/verify | Trigger verification |
| GET | /api/v1/investigations/{id}/export/{format} | Export |
| WS | /api/v1/investigations/{id}/stream | Live updates |

### Frontend Pages (8)
| Page | Components |
|------|------------|
| index.html | Dashboard, recent investigations |
| investigation.html | Tabs: Graph, Timeline, Evidence, Findings, Export, Verify |
| graph.html | Cytoscape interactive, filters, hypothesis overlay |
| timeline.html | Chart.js zoomable, annotations |
| evidence.html | Table + detail (receipt/provenance) |
| findings.html | Kanban (open/triaged/resolved), citations |
| verify.html | Tamper check results, hash chain |
| report.html | Markdown render with evidence links |

### Tech Stack
- Vanilla HTML/CSS/JS (ES2022)
- Chart.js 4.4 (CDN + local fallback)
- Cytoscape.js 3.29 (CDN + local fallback)
- marked.js for Markdown
- Lucide icons

### Deliverables
- [ ] `crates/jockey-api/src/server.rs` — Axum server + routes
- [ ] `crates/jockey-api/src/ws.rs` — WebSocket handler
- [ ] `frontend/` — 8 HTML pages + CSS + JS
- [ ] `frontend/lib/` — Local CDN fallbacks
- [ ] Integration tests: API endpoints, WebSocket
- [ ] E2E tests: Full investigation via API

### Success Criteria
- All API endpoints respond correctly
- Frontend loads and interacts with API
- WebSocket streams live updates
- `cargo test -p jockey-api` passes 100%

---

## Phase 7 — Verification + Export + CLI (Week 8-9)

### Goals
- End-to-end verification pipeline
- Tamper detection demo
- CLI tool with all commands
- Demo script automation

### Deliverables
- [ ] `crates/jockey-cli/src/main.rs` — CLI with subcommands
- [ ] CLI commands: `run`, `compile`, `graph`, `verify`, `export`, `load`, `narrative`, `serve`
- [ ] VerificationEngine integration in CLI
- [ ] Tamper detection: modify file -> verify FAIL -> restore -> verify PASS
- [ ] Demo script automation: `scripts/run_demo.sh`
- [ ] Pre-recorded demo video backup
- [ ] Performance optimization (binary size < 50MB, memory < 512MB)

### CLI Commands
```bash
jockey run investigate.jky --target live --host HOST
jockey compile investigate.jky --output-dir ./out
jockey graph show --run-id RUN_ID --format cytoscape
jockey verify --run-id RUN_ID [--evidence EVIDENCE_ID]
jockey export --run-id RUN_ID --format case_uco|cytoscape|timesketch|markdown
jockey load sample_case/
jockey narrative --run-id RUN_ID
jockey serve --addr 0.0.0.0:8080
```

### Success Criteria
- All CLI commands work end-to-end
- Tamper demo works reliably
- Binary size < 50MB (release, stripped, LTO)
- Memory < 512MB for 1000 evidence
- Demo script runs in < 8 minutes

---

## Phase 8 — SIH Demo Polish (Week 9-10)

### Goals
- Full demo rehearsal
- Judge Q&A preparation
- Final documentation polish
- Release build

### Deliverables
- [ ] Full demo rehearsal (multiple times, < 8 min)
- [ ] Pre-recorded demo video (backup)
- [ ] Judge Q&A prep document (25 Q&A)
- [ ] Final README.md polish
- [ ] Final JOCKY_SPEC_SHEET.md review
- [ ] All documentation cross-links validated
- [ ] Release build: `cargo build --release --workspace`
- [ ] Binary tested on Windows 10/11 + Ubuntu 22.04/24.04
- [ ] GitHub release with binary + checksums
- [ ] Final CHANGELOG.md update
- [ ] Final COMPLETE_AUDIT_REPORT.md update (Phase 8 status)

### Demo Script (8 Steps, ~7 min)
1. Load Sample
2. Show Graph
3. Run DSL
4. Compile (VQL + KAPE + CASE)
5. Export Narrative
6. Verify Integrity
7. Tamper Demo (FAIL -> PASS)
8. Close

### Success Criteria
- Demo runs smoothly < 8 minutes
- All 8 steps work reliably
- Judge Q&A handled confidently
- Binary runs on target platforms
- Documentation complete and accurate

---

## Post-SIH (Future Phases)

| Phase | Focus | Effort |
|-------|-------|--------|
| **Phase 9** | eBPF Collectors (Linux) | High |
| **Phase 10** | Kernel Drivers (Windows) | Very High |
| **Phase 11** | Remote Agent Protocol | High |
| **Phase 12** | Multi-tenancy + RBAC | Medium |
| **Phase 13** | AI Assist (Local LLM) | Medium |
| **Phase 14** | Collaborative Investigation | High |
| **Phase 15** | Cloud Collection | Medium |
| **Phase 16** | Container Forensics | Medium |
| **Phase 17** | Formal Verification | Very High |

---

## Milestone Tracking

| Milestone | Target Date | Status |
|-----------|-------------|--------|
| Phase 0 Complete | Week 0 | ✅ |
| Phase 1 Complete | Week 2 | 🔄 |
| Phase 2 Complete | Week 3 | ⏳ |
| Phase 3 Complete | Week 4 | ⏳ |
| Phase 4 Complete | Week 6 | ⏳ |
| Phase 5 Complete | Week 7 | ⏳ |
| Phase 6 Complete | Week 8 | ⏳ |
| Phase 7 Complete | Week 9 | ⏳ |
| Phase 8 Complete | Week 10 | ⏳ |
| SIH Demo | Week 10 | ⏳ |

---

## Related Documents

- `HANDOFF.md` — Implementation guide
- `JOCKY_SPEC_SHEET.md` — Engineering source of truth
- `COMPLETE_AUDIT_REPORT.md` — Current status
- `docs/evaluation/demo-plan.md` — Demo script
- `docs/evaluation/judge-questions.md` — Q&A prep