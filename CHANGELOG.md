# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased] - Phase 0 Complete

### Added
- **Repository Structure**: Complete Cargo workspace with 7 crates
  - `jockey-core`: IR, Execution Contract, Policy, Storage
  - `jockey-language`: Parser, AST, Type Checker, IR Lowering
  - `jockey-runtime`: Execution Engine, Dispatcher
  - `jockey-evidence`: Evidence, Receipt, Provenance, Verification, Export
  - `jockey-collectors`: Collector Trait, Windows/Linux implementations
  - `jockey-correlation`: Rules Engine, Investigation Graph
  - `jockey-api`: Axum HTTP Server, WebSocket, Frontend
- **Configuration**: rust-toolchain.toml, rustfmt.toml, clippy.toml, .gitignore
- **CI/CD**: GitHub Actions workflow (fmt, check, clippy, test, docs, audit, deny)
- **GitHub Templates**: Bug report, feature request, PR template

### Documentation
- **README.md**: Judge-facing project overview with architecture diagram
- **JOCKY_SPEC_SHEET.md**: Engineering source of truth (specifications)
- **COMPLETE_AUDIT_REPORT.md**: Honest Phase 0 audit
- **HANDOFF.md**: Implementation guide for next agents
- **ROADMAP.md**: Phased implementation plan (10 weeks)
- **DATASETS.md**: Dataset specifications and provenance
- **RUN_AND_EVALUATE.md**: Build, run, test, and demo instructions
- **CONTRIBUTING.md**: Contribution guidelines
- **SECURITY.md**: Security policy and vulnerability reporting
- **LICENSE**: MIT License

### Architecture Documentation (`docs/architecture/`)
- `overview.md`: System overview and principles
- `system-architecture.md`: Crate structure, interfaces, IR, Contract, Runtime
- `data-flow.md`: End-to-end data flow with evidence pipeline
- `execution-contract.md`: Contract structure, policy validation
- `evidence-pipeline.md`: Evidence → Receipt → Provenance flow
- `collector-architecture.md`: Collector trait, 10 MVP collectors, eBPF direction
- `correlation-architecture.md`: 9 deterministic rules, petgraph, queries
- `deployment-model.md`: Single binary, future distributed, air-gapped

### Language Documentation (`docs/language/`)
- `language-overview.md`: Design philosophy, 10 statements, examples
- `grammar.md`: Complete Pest grammar
- `ast.md`: AST node definitions with Rust types
- `ir.md`: IR specification with JSON examples
- `examples.md`: 10 investigation scripts (APT29, PowerShell, SMB, SSH, Web Shell, DNS, Scheduled Task, WMI, Credential Dumping, Multi-stage)

### Evidence Documentation (`docs/evidence/`)
- `evidence-model.md`: 7 evidence types with JSON schemas
- `evidence-receipt.md`: Ed25519 + blake3 receipt specification
- `provenance.md`: Hash-linked chain specification
- `verification.md`: Verification engine, tamper detection

### Research Documentation (`docs/research/`)
- `research-overview.md`: Categories and key questions
- `literature-review.md`: 14 verified academic/industry sources
- `competitor-analysis.md`: 12 tools compared with detailed matrix
- `novelty-analysis.md`: Honest differentiation (what's NOT novel)
- `research-to-design.md`: Research → architecture mapping table
- `sources.md`: Verified bibliography with DOIs/URLs

### Evaluation Documentation (`docs/evaluation/`)
- `evaluation-plan.md`: Test categories, CI integration, benchmarks
- `success-criteria.md`: Functional, demo, performance, security criteria
- `demo-plan.md`: 8-step SIH demo script with tamper detection
- `judge-questions.md`: 25 anticipated questions with answers
- `limitations.md`: Honest disclosure of 10+ constraints
- `risk-register.md`: 20 risks with mitigation tracking

### Diagrams (`assets/diagrams/`)
- 14 Mermaid diagrams covering all architectural aspects
- `README.md` explaining diagram index and rendering

### Collector Specifications
- `collectors/windows/README.md`: 5 collectors with APIs, schemas
- `collectors/linux/README.md`: 5 collectors + eBPF extension architecture

### Placeholders
- `examples/investigations/`, `examples/jockey-scripts/`
- `frontend/README.md`, `screenshots/README.md`
- `scripts/README.md`, `tools/README.md`, `tests/README.md`
- `data/samples/README.md`, `data/test_cases/README.md`
- `assets/architecture/`, `assets/research/`, `assets/branding/`

### Configuration
- **Cargo.toml**: Workspace with 7 crates, shared dependencies
- **rust-toolchain.toml**: Stable, rustfmt, clippy, targets
- **rustfmt.toml**: Standard formatting config
- **clippy.toml**: Allow/deny lists
- **.gitignore**: Comprehensive exclusions
- **.github/workflows/ci.yml**: fmt, check, clippy, test, docs, audit, deny
- **.github/ISSUE_TEMPLATE/**: Bug report, feature request
- **.github/PULL_REQUEST_TEMPLATE.md**: Comprehensive checklist

---

## [0.1.0] - TBD (Phase 1 Target)

### Planned
- **Phase 1**: JOCKY DSL implementation (parser, AST, type checker, IR)
- **Phase 2**: IR + Execution Contract + Policy Engine
- **Phase 3**: Evidence Model + Receipts + Provenance + Export
- **Phase 4**: Windows Collectors (5) + Linux Collectors (5)
- **Phase 5**: Correlation Engine + Investigation Graph
- **Phase 6**: Axum API + Frontend Dashboard
- **Phase 7**: Verification + CLI + Demo Automation
- **Phase 8**: SIH Demo Polish

---

## Versioning Policy

- **MAJOR**: Breaking changes to DSL, IR schema, evidence model, API
- **MINOR**: New collectors, correlation rules, export formats, non-breaking features
- **PATCH**: Bug fixes, documentation updates, performance improvements

---

## Release Process

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml` (workspace)
3. Create release PR
4. Merge to `main`
5. Tag release: `git tag v0.x.0`
6. GitHub Actions builds and publishes release
7. Update `CHANGELOG.md` for next version

---

## Links

- **Repository**: https://github.com/RishiSakhija/JOKEY---SIH26148
- **Issues**: https://github.com/RishiSakhija/JOKEY---SIH26148/issues
- **Discussions**: https://github.com/RishiSakhija/JOKEY---SIH26148/discussions
- **Security**: SECURITY.md