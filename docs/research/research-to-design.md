# Research to Design Mapping

**Status**: SPECIFIED

---

## Purpose

Trace each architectural decision in JOCKY to its research foundation.

---

## Mapping Table

| Research / Tool | Finding | Design Decision | JOCKY Component |
|----------------|---------|-----------------|-----------------|
| **Nugget (2015)** | Forensic DSLs with evidence types + provenance already exist | Do not claim DSL novelty alone; position JOCKY DSL as compilation target | `jockey-language`, USP positioning |
| **Velociraptor VQL** | Mature forensic query language with 500+ artifacts, live hunting | JOCKY compiles TO VQL; VQL is a backend target | `jockey-language` (compiler), `jockey-runtime` |
| **osquery SQL** | Universal SQL interface to OS state; snapshot-only | JOCKY compiles TO SQL; adds procedural collection + timeline | `jockey-language` (compiler) |
| **GRR** | Approval-based remote live forensics; Python flows | JOCKY uses native DSL (not Python); execution contracts replace approval flows | `jockey-core` (contracts), `jockey-language` |
| **KAPE** | Target/Module abstraction; Windows triage speed; VSS | Adopt Target/Module pattern but make query-driven, cross-platform, centrally deployable | `jockey-language` (collect -> KAPE targets) |
| **Volatility 3** | Symbol-backed memory analysis; Python plugins | JOCKY compiles TO Volatility plugins; adds live memory acquisition | `jockey-language` (compiler), `collectors` |
| **Plaso/log2timeline** | 1000+ parsers; super timeline generation | JOCKY consumes Plaso output; does not replicate parsers | `jockey-correlation` (timeline), `examples` |
| **Timesketch** | Collaborative timeline analysis; sketches = hypotheses | JOCKY exports to Timesketch; hypothesis tracking is first-class in DSL | `jockey-language` (hypothesis), `jockey-correlation` |
| **Provexa/ProvQL** | DSL + provenance graph for investigation | Proves DSL+provenance works; JOCKY adds execution contracts + compilation targets | `jockey-evidence` (provenance), `jockey-core` (contracts) |
| **DFIR-ORC** | Evidence collection format with integrity | Consider ORC as export format; JOCKY has native evidence model | `jockey-evidence` (export) |
| **CASE/UCO** | Ontology for cyber-investigation exchange | JOCKY exports to CASE/UCO; not a replacement | `jockey-evidence` (export) |
| **LEMON (2021)** | eBPF for memory acquisition without kernel module | Validates eBPF approach; document as future Linux collector extension | `collectors/linux/README.md` (eBPF section) |
| **Aya (Rust eBPF)** | Pure Rust eBPF with BTF/CO-RE | Technical foundation for Linux eBPF collectors (post-MVP) | `collectors/linux/README.md` |
| **Rusting Volatility** | Rust in memory forensics for performance/safety | Supports Rust architectural direction | `Cargo.toml`, all crates |
| **NIST SP 800-86** | Forensic methodology, evidence handling | Align evidence handling, chain of custody with NIST | `docs/evidence/`, `SECURITY.md` |
| **STIX/TAXII** | Threat intel sharing, structured indicators | Support STIX indicators in JOCKY indicator evidence type | `docs/evidence/evidence-model.md` |

---

## Architecture Decisions Traced to Research

### 1. Why Rust?
- **Rusting Volatility**: Rust improves performance/safety in forensics
- **Aya**: Rust eBPF for Linux collectors
- **Memory safety**: Critical for forensic tool correctness
- **WASM target**: Future collector plugin portability

### 2. Why pest Parser?
- **PEG grammar**: Better error messages than LR parsers
- **Precedence climbing**: Natural expression parsing
- **Used by**: Many production Rust projects (Rust Analyzer, etc.)

### 3. Why Execution Contracts?
- **GRR approval workflows**: Proves need for authorization
- **Velociraptor artifact parameters**: Proves capability declaration
- **JOCKY adds**: Least privilege, policy bounds, collector allowlist, timeout enforcement

### 4. Why Evidence Receipts (Ed25519 + blake3)?
- **Provexa**: Proves provenance value
- **Velociraptor artifact signing**: Proves signing value
- **blake3**: Fast, parallel, 256-bit (chosen over SHA-256 for speed)
- **Ed25519**: Small keys, fast verify, constant-time (chosen over RSA/ECDSA)

### 5. Why Deterministic Correlation (Not ML)?
- **Timesketch analyzers**: ML exists but opaque
- **KQL/ES|QL joins**: Deterministic but not evidence-cited
- **JOCKY**: Rule-based with explicit evidence IDs, confidence, method — verification-backed

### 6. Why petgraph for Investigation Graph?
- **Pure Rust**: No FFI, memory safe
- **Flexible algorithms**: A*, Dijkstra, DFS, topological sort
- **No external deps**: Single binary deployment

### 7. Why SQLite + JSON/JSONL?
- **Embedded**: No separate DB server
- **ACID**: Transactional evidence integrity
- **JSON1 extension**: Query JSON columns
- **Portable**: Single file per investigation

### 8. Why Axum + Static Frontend?
- **Axum**: Type-safe, middleware, OpenAPI generation
- **Static HTML/CSS/JS**: Zero build, works offline, portable
- **Chart.js + Cytoscape.js**: Standard viz libraries, CDN + local fallback

### 9. Why ULIDs?
- **Monotonic**: Sortable by creation time
- **No coordination**: Distributed generation
- **128-bit**: Collision resistant
- **URL-safe**: Base32 encoding

### 10. Why Compilation Targets (VQL, SQL, KAPE, Volatility, CASE)?
- **Velociraptor/osquery/KAPE/Volatility**: Best-in-class collectors exist
- **JOCKY value**: Unification — one script to all backends
- **No replication**: JOCKY does not reimplement collectors

---

## Related Documents

- `research-overview.md` — Research categories
- `literature-review.md` — Verified research sources
- `competitor-analysis.md` — Tool comparison
- `novelty-analysis.md` — Differentiation
- `sources.md` — Bibliography