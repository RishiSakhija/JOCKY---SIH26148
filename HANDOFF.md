# Handoff Document

**For:** Next AI/Developer implementing JOCKY
**From:** Lead Repository Architect (Phase 0)
**Date:** October 2026
**Phase:** 0 Complete — Specification Only

---

## Project Purpose

**JOCKY — Evidence-Contract Forensics Platform**

SIH Problem Statement 26148: Create a programming language/framework for computer and network forensic analysis with cross-platform Windows/Ubuntu operation, centralized multi-system analysis, automated forensic functions, and security-resilient execution.

**Core USP:** Evidence-Contract Forensics — JOCKY integrates forensic workflow definition, controlled execution, evidence receipts, provenance, deterministic correlation, and verified investigation output into one traceable workflow.

**NOT:** Replacing Velociraptor/KAPE/Volatility. JOCKY compiles TO them.

---

## Locked Architecture (Cannot Change Without Approval)

### 1. Pipeline (Immutable)
```
JOCKY Script -> pest Parser -> AST -> IR -> Execution Contract -> Policy Validation
-> Rust Runtime -> Windows/Linux Collectors -> Common Evidence Model
-> Evidence Receipt -> Provenance -> Deterministic Correlation
-> Investigation Graph -> Verification -> Dashboard/Report
```

### 2. Technology Stack (Locked)
| Layer | Technology | Version |
|-------|------------|---------|
| Language | Rust | 2021 edition, stable |
| Parser | pest | 2.7 |
| Crypto | blake3, ed25519-dalek | Latest |
| Graph | petgraph | 0.6 |
| Database | SQLite (rusqlite) | Latest |
| API | Axum | 0.7 |
| Frontend | HTML/CSS/JS + Chart.js + Cytoscape.js | CDN + local |
| IDs | ulid | 1.0 |
| Time | chrono (UTC only) | 0.4 |

### 3. Evidence Types (Locked - 7)
- Process (`artifact/process`)
- File (`artifact/file`, `artifact/ntfs`, `artifact/mft`)
- Registry/Config (`artifact/reg`, `artifact/config`)
- LogEvent (`artifact/evt`, `artifact/journald`, `artifact/syslog`, `artifact/audit`)
- NetworkConnection (`artifact/connection`, `artifact/netflow`, `artifact/pcap`)
- Indicator (`indicator/ioc`, `indicator/yara`, `indicator/sigma`)
- Finding (`finding/detection`, `finding/hypothesis`)

### 4. MVP Collectors (Locked - 10)
| Windows (5) | Linux (5) |
|-------------|-----------|
| proc_windows | proc_linux |
| file_windows | file_linux |
| registry_windows | config_linux |
| logs_windows | logs_linux |
| net_windows | net_linux |

### 5. Correlation Rules (Locked - 9)
- pid_link (spawned, 1.0)
- file_write (wrote, 0.95)
- file_read (read, 0.9)
- net_link (connected, 0.95)
- ioc_match (resolved_to, 1.0)
- yara_match (contains, 0.9)
- sigma_match (contains, 0.9)
- rule_match (indicates, 0.8)
- timestamp_join (timeline_adjacent, 0.7)

### 6. DSL Statements (Locked - 10)
`investigation`, `hypothesis`, `collect`, `filter`, `match`, `correlate`, `timeline`, `bind`, `export`, `verify`

### 7. Crypto Primitives (Locked)
- Hash: blake3 (primary), SHA-256 (fallback)
- Signatures: Ed25519
- IDs: ULID (monotonic, sortable)
- Time: UTC only, RFC3339

---

## Repository Map

```
JOKEY---SIH26148/
├── .github/           # CI, issue/PR templates
├── crates/            # 7 Rust crates (Cargo.toml each)
│   ├── jockey-core/       # IR, Contract, Policy, Storage
│   ├── jockey-language/   # Parser, AST, Type Checker
│   ├── jockey-runtime/    # Execution Engine
│   ├── jockey-evidence/   # Evidence, Receipt, Provenance, Export
│   ├── jockey-collectors/ # Collector Trait, Win/Lin Impl
│   ├── jockey-correlation/# Rules, Graph, Queries
│   └── jockey-api/        # Axum Server, WebSocket
├── collectors/        # Platform-specific specs
│   ├── windows/README.md
│   └── linux/README.md
├── data/              # Samples + test cases (READMEs only)
├── docs/              # All documentation
│   ├── architecture/ (7 files)
│   ├── language/ (4 files)
│   ├── evidence/ (4 files)
│   ├── research/ (5 files)
│   └── evaluation/ (6 files)
├── examples/          # Investigation scripts
├── frontend/          # Static HTML/CSS/JS (README only)
├── screenshots/       # Placeholder (README only)
├── scripts/           # Build/demo scripts (README only)
├── tools/             # Compiler targets (README only)
├── tests/             # Test organization (README only)
├── assets/            # Diagrams, branding
│   └── diagrams/ (14 .mmd files)
├── README.md          # Judge-facing
├── JOCKY_SPEC_SHEET.md  # Engineering source of truth
├── COMPLETE_AUDIT_REPORT.md
├── DATASETS.md
├── RUN_AND_EVALUATE.md
├── HANDOFF.md         # This file
├── CONTRIBUTING.md
├── SECURITY.md
├── ROADMAP.md
├── CHANGELOG.md
├── LICENSE (MIT)
├── Cargo.toml (workspace)
├── rust-toolchain.toml
├── rustfmt.toml
├── clippy.toml
└── .gitignore
```

---

## Current Status

**Phase 0: COMPLETE** — Specification, research, documentation, diagrams, configs, CI/CD all done.

**NO IMPLEMENTATION CODE EXISTS YET.**

| Component | Status |
|-----------|--------|
| JOCKY DSL Grammar | SPECIFIED |
| AST Definition | SPECIFIED |
| IR Specification | SPECIFIED |
| Execution Contract | SPECIFIED |
| Evidence Model | SPECIFIED |
| Evidence Receipt | SPECIFIED |
| Provenance Chain | SPECIFIED |
| Collector Interface | SPECIFIED |
| Correlation Rules | SPECIFIED |
| Investigation Graph | SPECIFIED |
| Windows Collectors | NOT IMPLEMENTED |
| Linux Collectors | NOT IMPLEMENTED |
| Runtime Engine | NOT IMPLEMENTED |
| Dashboard/Frontend | NOT IMPLEMENTED |

---

## Implementation Order (Phase 1-8)

### Phase 1: JOCKY DSL (Week 1-2)
1. `crates/jockey-language/src/parser.pest` — Pest grammar
2. `crates/jockey-language/src/parser.rs` — Parser + CST to AST
3. `crates/jockey-language/src/ast.rs` — AST node definitions
4. `crates/jockey-language/src/typeck.rs` — Type checker + capability inference
5. `crates/jockey-language/src/ir.rs` — IR lowering
6. Tests: parser roundtrip, type checking, golden files

### Phase 2: IR + Execution Contract (Week 2-3)
1. `crates/jockey-core/src/ir.rs` — IR types
2. `crates/jockey-core/src/contract.rs` — Contract builder
3. `crates/jockey-core/src/policy.rs` — Policy engine + capability validation
4. `crates/jockey-core/src/storage.rs` — SQLite schema + migrations
5. Tests: contract building, policy validation, storage CRUD

### Phase 3: Evidence + Provenance (Week 3-4)
1. `crates/jockey-evidence/src/model.rs` — Evidence, Receipt, Provenance types
2. `crates/jockey-evidence/src/receipt.rs` — Receipt signing/verification
3. `crates/jockey-evidence/src/provenance.rs` — Provenance chain logic
4. `crates/jockey-evidence/src/verify.rs` — Verification engine
5. `crates/jockey-evidence/src/export.rs` — CASE/UCO, Cytoscape, Timesketch, Markdown
5. Tests: crypto, provenance chains, verification, exports

### Phase 4: Windows/Linux Collectors (Week 4-6)
1. `crates/jockey-collectors/src/trait.rs` — Collector trait + registry
2. Windows: proc, file, registry, logs, net (windows-sys)
3. Linux: proc, file, config, logs, net (nix, libc)
4. Collector manifests with capabilities, schemas, public keys
5. Tests: contract tests, mock execution, output validation

### Phase 5: Correlation (Week 6-7)
1. `crates/jockey-correlation/src/rules.rs` — Rule engine
2. `crates/jockey-correlation/src/engine.rs` — Correlation logic
3. `crates/jockey-correlation/src/graph.rs` — InvestigationGraph + queries
4. Hypothesis overlay
5. Tests: rule matching, graph queries, hypothesis tracking

### Phase 6: API + Frontend (Week 7-8)
1. `crates/jockey-api/src/server.rs` — Axum server + routes
2. `crates/jockey-api/src/ws.rs` — WebSocket handler
3. Frontend: 8 HTML pages + CSS + JS (Cytoscape, Chart.js)
4. Static asset serving
5. Tests: API endpoints, WebSocket, frontend integration

### Phase 7: Verification + Polish (Week 8-9)
1. End-to-end verification pipeline
2. Tamper detection demo
3. CLI: `jockey run|compile|graph|verify|export|load|narrative`
4. Demo script automation
6. Performance optimization

### Phase 8: SIH Demo (Week 9-10)
1. Full demo rehearsal
2. Pre-recorded video backup
3. Judge Q&A prep
4. Final documentation polish
5. Release build + binary size optimization

---

## Coding Conventions

### Rust Style
- `cargo fmt --all` — Enforced by CI
- `cargo clippy -- -D warnings` — Enforced by CI
- No `unwrap()`/`expect()` in production paths — Use `?` and `thiserror`/`anyhow`
- `unsafe` only with documented invariants — Audit required
- Async with `tokio` — No blocking in async contexts

### Error Handling
```rust
// Good
fn foo() -> Result<Output, CollectorError> {
    let data = collector.execute(...).await?;
    Ok(process(data))
}

// Bad
fn foo() -> Output {
    collector.execute(...).await.unwrap()
}
```

### Testing
- Unit tests in `src/**/*.rs` as `#[cfg(test)] mod tests`
- Integration tests in `tests/**/*.rs`
- Golden files in `tests/golden/`
- Property tests with `proptest`
- Contract tests for Collector trait

### Documentation
- All public items documented (`///`)
- Architecture decisions in `docs/architecture/`
- API docs via `cargo doc` (enabled in CI)
- Diagrams as Mermaid in `assets/diagrams/`

---

## Security Boundaries

### DO Implement
- Forensic analysis and evidence collection
- Verification and tamper detection
- Controlled investigation execution
- CASE/UCO export for interoperability

### DO NOT Implement
- EDR disabling or bypass
- BYOVD (Bring Your Own Vulnerable Driver) exploitation
- Process injection (CreateRemoteThread, etc.)
- Covert C2 channels
- Malware persistence techniques
- Offensive credential theft (LSASS dumping for attack)
- Security control bypass (AMSI, ETW, etc.)

The original PS may discuss these as threat context, but our implementation focuses on **detection, collection, and verification** of such techniques, not their execution.

---

## Documentation Rules

1. **Update specs before code** — Specs are source of truth
2. **Terminology consistency** — Use `JOCKY_SPEC_SHEET.md` glossary
3. **Diagram updates** — Mermaid in `assets/diagrams/`, explain in `docs/architecture/`
3. **Research citations** — Add to `docs/research/sources.md` with DOI/URL
4. **Changelog** — Update `CHANGELOG.md` for every PR
5. **Audit updates** — Update `COMPLETE_AUDIT_REPORT.md` at each phase

---

## How Future Agents Should Update the Audit

At each phase completion:
1. Run `cargo fmt --check`, `cargo clippy`, `cargo test`
2. Update `COMPLETE_AUDIT_REPORT.md` with:
   - New component statuses (SPECIFIED -> IMPLEMENTED -> VERIFIED)
   - New test coverage numbers
   - New performance benchmarks
   - Any new limitations discovered
   - Any terminology changes
3. Re-run unsupported claims check
4. Update `CHANGELOG.md` with version bump
5. Tag release: `git tag v0.x.0`

---

## Key References

| Document | Purpose |
|----------|---------|
| `README.md` | Judge-facing overview |
| `JOCKY_SPEC_SHEET.md` | Engineering source of truth |
| `COMPLETE_AUDIT_REPORT.md` | Honest status |
| `ROADMAP.md` | Phased plan |
| `COMPETITOR_RESEARCH_REPORT.md` | Competitor analysis |
| `JOCKY_DATA_CONTRACTS.md` | Data schemas |
| `docs/architecture/*.md` | Deep technical specs |
| `docs/research/*.md` | Research foundation |
| `assets/diagrams/*.mmd` | Architecture diagrams |

---

## Final Notes

This is a **specification-first** project. The documents in this repository ARE the product at Phase 0. They must remain accurate, consistent, and honest.

**When in doubt:** Check `JOCKY_SPEC_SHEET.md` — it is the single source of truth for all engineering decisions.

**When blocked:** Check `docs/architecture/` for detailed specs, `docs/research/` for design rationale, `HANDOFF.md` for implementation order.

**Remember:** JOCKY's value is the **integration** — one script to VQL + SQL + KAPE + Volatility + CASE/UCO, with cryptographic evidence integrity throughout. Every component serves that integration.

---

**Good luck. Build something that matters.**