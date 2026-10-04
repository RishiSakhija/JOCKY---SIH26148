# Limitations

**Status**: SPECIFIED

---

## Purpose

Honest disclosure of current constraints and known limitations.

---

## Current Limitations (Phase 0 - Specification Only)

### 1. No Implementation Exists
- This repository contains **specifications, architecture, research, and documentation only**
- No Rust code compiled or tested
- No binaries produced
- All "implemented" items in MVP tables refer to **specification completeness**, not code

### 2. No Runtime Validation
- Architecture untested against real collectors
- No performance benchmarks
- No integration with actual Velociraptor/KAPE/Volatility
- CLI/API commands untested end-to-end

### 3. eBPF Not Implemented
- Documented as extensible architecture direction only
- No Aya-based collectors written
- No kernel version compatibility testing
- No verifier log capture or graceful fallback logic

### 3. No Court Admissibility Claim
- Provenance chain designed for verifiability
- Legal admissibility depends on jurisdiction and procedure
- No legal review performed
- No chain-of-custody forms or expert witness materials

### 4. No AI/ML Correlation
- Deterministic rules only (9 in MVP)
- AI assist explicitly deferred to post-MVP
- No local LLM integration
- No hypothesis suggestion engine

### 5. No Multi-Tenancy/RBAC
- Single-analyst model for MVP
- No orgs, teams, roles, permissions
- No data isolation between investigations
- No audit logging for compliance

### 6. No Remote Agent Protocol
- Local execution only (CLI + API server on same host)
- Remote via SSH/WinRM orchestration deferred
- No gRPC/mTLS agent protocol
- No fleet management capabilities

### 7. Performance Uncharacterized
- blake3 chosen for speed; no benchmarks yet
- Correlation latency unmeasured
- Verification time unmeasured
- Memory usage unmeasured
- Binary size unmeasured

### 8. Windows Kernel Drivers Not Used
- Userspace collectors only for MVP (ETW, WMI, PSAPI, Registry, EVTX)
- Kernel mode (ETW kernel provider, minifilter) deferred
- No VSS/USN journal from kernel perspective
- No live memory acquisition (WinPMEM integration deferred)

### 9. CASE/UCO Mapping Incomplete
- Core classes mapped (Observable, Action, Tool, etc.)
- Full UCO ontology not implemented
- No round-trip validation with CASE tools
- Extension vocabularies not defined

### 10. No Formal Verification
- No Prusti/Kani proofs for IR safety properties
- No model checking of correlation engine
- No proof of capability enforcement correctness
- Specified as future research direction

---

## Architectural Limitations

### 1. In-Memory Graph Only
- petgraph stores entire investigation in memory
- No persistent graph database (Neo4j, JanusGraph)
- Large investigations (>100k nodes) may exceed memory
- No incremental graph updates

### 2. SQLite Single-Writer
- SQLite doesn't support concurrent writers well
- Single investigation = single SQLite file
- No distributed/clustered deployment
- Backup via file copy only

### 3. Static Frontend Assets
- No build step for frontend (vanilla JS)
- No TypeScript, no bundler, no hot reload
- Manual dependency management for Chart.js/Cytoscape.js
- Limited to browser APIs available without build

### 4. No Authentication/Authorization in MVP
- API server has no auth
- WebSocket open to anyone with URL
- Evidence root accessible via API
- Intended for air-gapped/single-user deployment only

### 5. Collector Version Pinning Rigid
- Contract pins exact collector version
- No semantic version compatibility (e.g., 1.0.x -> 1.1.0)
- Requires contract rebuild for collector updates
- Future: compatibility matrix in manifest

### 6. Hypothesis Confidence Heuristic
- Auto-confidence = supporting / (supporting + refuting)
- No Bayesian updating
- No confidence propagation through graph
- Analyst manual override only

### 7. Export Formats Limited
- CASE/UCO: core classes only
- Cytoscape: JSON with styles
- Timesketch: CSV only (no JSONL)
- No GraphML, DOT, STIX, MISP export yet

---

## Operational Limitations

### 1. Air-Gapped Deployment Complexity
- Single binary but requires Rust toolchain to build
- Collector manifests with public keys must be distributed
- No offline package format (future: air-gapped bundle)

### 2. Evidence Retention Policy
- No automated retention/deletion
- Evidence root grows unbounded
- No compression/archival of old runs
- Manual cleanup required

### 3. No Backup/Restore Automation
- SQLite backup = file copy (must stop server)
- Evidence files = directory copy
- No point-in-time recovery
- No cross-region replication

### 4. Limited Logging/Observability
- Structured logging (tracing) but no metrics export
- No Prometheus/Grafana integration
- No distributed tracing
- Debug logs only via RUST_LOG

### 5. No Configuration Hot-Reload
- Config changes require server restart
- Collector registry changes require restart
- Rule changes require restart

---

## Future Work (Post-MVP)

See `ROADMAP.md` for phased approach.

---

## Related Documents

- `evaluation-plan.md` — Evaluation plan
- `success-criteria.md` — Success criteria
- `demo-plan.md` — Demo script
- `judge-questions.md` — Judge questions
- `risk-register.md` — Risk register