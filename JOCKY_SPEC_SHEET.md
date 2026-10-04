# JOCKY Specification Sheet

**Engineering Source of Truth** — Single authoritative reference for all implementation decisions.

---

## 1. Problem Statement

**SIH 26148**: Create a programming language/framework "JOCKY" for computer and network forensic analysis with cross-platform Windows/Ubuntu operation, centralized multi-system analysis, automated forensic functions, and security-resilient execution.

**Core Problem**: DFIR investigations require stitching together 5–7 specialized tools (Velociraptor, KAPE, Volatility, Plaso, Timesketch, etc.) with incompatible languages, data formats, and collection paradigms. No unified workflow exists that captures investigator intent, executes controlled collection, produces tamper-evident evidence, and reconstructs a causal attack narrative with verifiable provenance.

---

## 2. Goals

| Goal | Description | Success Metric |
|------|-------------|----------------|
| **Unified Investigation Language** | Single DSL expressing collection → correlation → timeline → report | One script compiles to VQL, SQL, KAPE, Volatility, CASE/UCO |
| **Execution Contract** | Capability-validated, policy-bound execution plans | Runtime rejects unauthorized collectors/params |
| **Evidence-First Model** | Every artifact typed, hashed, signed, provenance-tracked | 100% evidence objects have receipt + provenance chain |
| **Deterministic Correlation** | Rule-based edges with explicit evidence citations | Zero ML/heuristic scoring; all edges traceable to evidence IDs |
| **Cross-Platform Parity** | Same artifact semantics on Windows/Linux | `collect process` works identically on both |
| **Verifiable Replay** | Same script + same target → bit-for-bit identical evidence bundle | Hash chain validation passes on re-execution |
| **Court-Ready Export** | CASE/UCO + narrative report with clickable citations | Export passes verification; citations resolve to evidence |

---

## 3. Non-Goals

| Non-Goal | Reason |
|----------|--------|
| Replace Velociraptor/osquery/KAPE/Volatility | JOCKY compiles TO them; they remain best-in-class collectors |
| Build a new timeline engine | Plaso/log2timeline is the standard; JOCKY consumes its output |
| Implement AI/ML correlation | Deterministic rules only; AI assist deferred |
| Multi-tenancy / RBAC / Org management | Single-analyst MVP; enterprise features post-SIH |
| Kernel-mode collectors / drivers | Userspace only for MVP; kernel mode adds complexity/risk |
| Remote agent protocol | Local execution only; SSH/WinRM orchestration deferred |
| Mobile / cloud / container collection | Host forensics MVP only |
| Real-time streaming / live dashboard | Batch execution MVP; streaming deferred |
| Custom graph database | petgraph in-memory; persistent graph DB deferred |

---

## 4. Core USP: Evidence-Contract Forensics

> **JOCKY integrates forensic workflow definition, controlled execution, evidence receipts, provenance, deterministic correlation, and investigation reporting into one traceable workflow.**

| Integration | What It Unifies |
|-------------|-----------------|
| Language → Collectors | One script → VQL, SQL, KAPE, Volatility, CASE/UCO |
| Intent → Execution Contract | Capability declarations → policy validation → least privilege |
| Collection → Evidence Receipt | Every artifact → Ed25519-signed receipt at collection time |
| Evidence → Provenance | Hash-linked chain from raw collection through every transform |
| Evidence → Correlation | Deterministic rules → cited edges (not opaque scores) |
| Graph → Verification | Tamper detection via receipt + provenance re-validation |
| Graph → Report | Narrative export with clickable evidence citations |

---

## 5. Architecture

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

## 6. Components

### 6.1 Crate Structure

| Crate | Responsibility | Public API |
|-------|----------------|------------|
| `jockey-language` | Parser, AST, type checker, diagnostics | `parse()`, `type_check()`, `lower_to_ir()` |
| `jockey-core` | IR, Execution Contract, Policy, Capabilities | `IrModule`, `ExecutionContract`, `PolicyEngine` |
| `jockey-runtime` | Execution engine, dispatcher, sandbox | `Runtime::execute(contract)` |
| `jockey-evidence` | Evidence, Receipt, Provenance, Finding | `Evidence`, `EvidenceReceipt`, `ProvenanceRecord` |
| `jockey-collectors` | Collector trait, Win/Lin implementations | `Collector::execute()`, `CollectorRegistry` |
| `jockey-correlation` | Rules engine, graph builder, query API | `Correlator::correlate()`, `InvestigationGraph` |
| `jockey-api` | Axum HTTP server, WebSocket, OpenAPI | `serve()`, `routes()` |

### 6.2 Component Interfaces

```rust
// jockey-language
pub fn parse(source: &str) -> Result<AstModule, ParseError>;
pub fn type_check(ast: &AstModule) -> Result<TypedAst, TypeError>;
pub fn lower_to_ir(typed: &TypedAst) -> Result<IrModule, LowerError>;

// jockey-core
pub fn build_contract(ir: &IrModule, target: TargetSpec, policy: Policy) -> ExecutionContract;
pub fn validate_contract(contract: &ExecutionContract, registry: &CollectorRegistry) -> Result<(), PolicyError>;

// jockey-runtime
pub async fn execute(contract: ExecutionContract) -> Result<ExecutionResult, RuntimeError>;

// jockey-evidence
pub fn verify_receipt(receipt: &EvidenceReceipt, evidence: &Evidence) -> Result<(), VerifyError>;
pub fn verify_provenance_chain(chain: &[ProvenanceRecord]) -> Result<(), VerifyError>;

// jockey-collectors
#[async_trait]
pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn supported_actions(&self) -> Vec<&'static str>;
    fn supported_os(&self) -> Vec<&'static str>;
    async fn execute(&self, action: &str, params: Value, ctx: &ExecutionContext) -> Result<CollectorOutput, CollectorError>;
}

// jockey-correlation
pub fn correlate(evidence: &[Evidence], rules: &[CorrelationRule]) -> Vec<CorrelationEdge>;
pub struct InvestigationGraph { /* petgraph::Graph<EvidenceNode, CorrelationEdge> */ }

// jockey-api
pub fn serve(addr: SocketAddr, state: AppState) -> Result<(), ServerError>;
```

---

## 7. JOCKY Language

### 7.1 Locked Commands (10 Statements)

| Command | Purpose | Example |
|---------|---------|---------|
| `investigation` | Top-level container | `investigation "name" { ... }` |
| `hypothesis` | Declare investigation hypothesis | `hypothesis "desc" { mitre: ["T1047"] }` |
| `collect` | Declare evidence collection | `collect process as procs where name == "svchost.exe"` |
| `filter` | Filter evidence stream | `filter procs where cpu > 50` |
| `match` | Pattern match on evidence | `match procs where command_line contains "powershell"` |
| `correlate` | Join evidence by relation | `correlate A -> B by "pid_link" as chain` |
| `timeline` | Build timeline from evidence | `timeline "name" from chain, other` |
| `bind` | Bind hypothesis to evidence | `bind hypothesis "name" to chain, other` |
| `export` | Declare output artifacts | `export case "name" format case_uco` |
| `verify` | Request verification step | `verify evidence_chain` |

### 7.2 Grammar Overview (Pest)

```pest
investigation = { "investigation" ~ identifier ~ "{" ~ statement* ~ "}" }
statement = { hypothesis | collect | filter | match | correlate | timeline | bind | export | verify }
collect = { "collect" ~ evidence_type ~ "as" ~ identifier ~ where_clause? }
where_clause = { "where" ~ expression }
correlate = { "correlate" ~ identifier ~ "->" ~ identifier ~ "by" ~ relation ~ "as" ~ identifier ~ correlate_params? }
timeline = { "timeline" ~ identifier ~ "from" ~ identifier~ ("," ~ identifier)* }
export = { "export" ~ output_type ~ identifier ~ "format" ~ format_spec }
```

Full grammar: `docs/language/grammar.md`

### 7.3 AST Nodes

```rust
pub enum Statement {
    Hypothesis(HypothesisDecl),
    Collect(CollectDecl),
    Filter(FilterDecl),
    Match(MatchDecl),
    Correlate(CorrelateDecl),
    Timeline(TimelineDecl),
    Bind(BindDecl),
    Export(ExportDecl),
    Verify(VerifyDecl),
}

pub struct CollectDecl {
    pub evidence_type: EvidenceType,  // Process, File, Registry, LogEvent, NetworkConnection, Indicator, Finding
    pub binding: String,
    pub filter: Option<Expression>,
}
```

Full AST: `docs/language/ast.md`

---

## 8. IR (Intermediate Representation)

```json
{
  "version": 1,
  "id": "ULID",
  "metadata": { "name": "", "description": "", "author": "", "created_at": "", "tags": [] },
  "steps": [
    {
      "id": "ULID",
      "collector": "windows_evt",
      "action": "query",
      "params": { "channel": "Security", "query": "*[System/EventID=4625]" },
      "binds": ["logon_events"],
      "depends_on": [],
      "condition": null
    }
  ],
  "outputs": [
    { "name": "findings", "from_step": "step_id", "field": "findings" }
  ]
}
```

**Key Properties:**
- ULIDs for all IDs (monotonic, sortable, no coordination)
- Explicit dependencies enable parallel execution
- Collector/action/params fully resolved (no DSL references)
- Output bindings declare data flow

Full IR spec: `docs/architecture/system-architecture.md`

---

## 9. Execution Contract

```json
{
  "ir_id": "ULID",
  "run_id": "ULID",
  "target": { "type": "live|dead|image|remote", "identifier": "", "os": "windows|linux|darwin", "arch": "" },
  "steps": [
    {
      "step_id": "ULID",
      "collector": "windows_evt",
      "action": "query",
      "params": { ... },
      "output_vars": ["logon_events"],
      "timeout_ms": 30000,
      "retry": { "max_attempts": 2, "backoff_ms": 1000 }
    }
  ],
  "policy": {
    "parallelism": 2,
    "continue_on_error": false,
    "evidence_root": "C:\\jocky\\evidence\\<run_id>",
    "hash_algorithm": "blake3"
  },
  "context": {}
}
```

**Policy Enforcement Points:**
- Collector allowlist per target OS
- Parameter schema validation per collector/action
- Timeout bounds (min 1s, max 300s per step)
- Evidence root isolation (chroot-style)
- Hash algorithm pinning (blake3 or SHA-256 only)

Full spec: `docs/architecture/execution-contract.md`

---

## 10. Evidence Model

### 10.1 Evidence Object

```json
{
  "id": "ULID",
  "run_id": "ULID",
  "step_id": "ULID",
  "collector": "windows_evt",
  "type": "artifact/evt",
  "name": "Security-4625-events",
  "path": "evidence/ULID.jsonl",
  "size": 24576,
  "hash": "hex(blake3(bytes))",
  "hash_algo": "blake3",
  "metadata": { "event_count": 47, "earliest": "...", "latest": "..." },
  "collected_at": "2026-10-04T12:00:05Z"
}
```

### 10.2 Evidence Types (MIME-like)

| Category | Types |
|----------|-------|
| Process | `artifact/process`, `artifact/process_tree` |
| File | `artifact/file`, `artifact/ntfs`, `artifact/mft` |
| Registry/Config | `artifact/reg`, `artifact/config` |
| LogEvent | `artifact/evt`, `artifact/journald`, `artifact/syslog` |
| NetworkConnection | `artifact/netflow`, `artifact/pcap`, `artifact/connection` |
| Indicator | `indicator/ioc`, `indicator/yara`, `indicator/sigma` |
| Finding | `finding/detection`, `finding/hypothesis` |

---

## 11. Evidence Receipt

```json
{
  "id": "ULID",
  "evidence_id": "ULID",
  "run_id": "ULID",
  "collector": "windows_evt",
  "collector_version": "0.1.0",
  "signature": "Ed25519(payload_hash || evidence_id || timestamp)",
  "public_key": "hex(collector_pubkey)",
  "payload_hash": "hex(blake3(evidence_bytes))",
  "payload_hash_algo": "blake3",
  "timestamp": "2026-10-04T12:00:05Z",
  "host_info": { "hostname": "", "os": "", "kernel": "", "collector_pid": 0 }
}
```

**Verification:**
1. Verify Ed25519 signature using collector public key
2. Recompute `blake3(evidence_bytes)` → match `payload_hash`
3. Verify timestamp within acceptable clock skew
4. Verify collector_version matches registry

---

## 12. Provenance Record

```json
{
  "id": "ULID",
  "evidence_id": "ULID",
  "receipt_id": "ULID",
  "prev_provenance_id": "ULID|null",
  "action": "collected|transformed|correlated|exported",
  "actor": "windows_evt|correlator|exporter",
  "actor_version": "0.1.0",
  "input_hashes": ["hash1", "hash2"],
  "output_hashes": ["hash3"],
  "parameters": { "rule": "timestamp_join", "window": 300 },
  "timestamp": "2026-10-04T12:00:05Z",
  "signature": "Ed25519(actor_key, payload)"
}
```

**Chain Properties:**
- `prev_provenance_id` creates hash-linked chain
- Each transformation produces new provenance record
- Actor signs with its identity key
- Immutable append-only log per evidence object

---

## 13. Collector Interface

```rust
#[async_trait]
pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn supported_actions(&self) -> Vec<&'static str>;
    fn supported_os(&self) -> Vec<&'static str>;
    async fn execute(
        &self,
        action: &str,
        params: Value,
        context: &ExecutionContext,
    ) -> Result<CollectorOutput, CollectorError>;
}

pub struct ExecutionContext {
    pub run_id: String,
    pub evidence_root: PathBuf,
    pub target: TargetSpec,
    pub temp_dir: PathBuf,
    pub credentials: Option<Value>,
}

pub struct CollectorOutput {
    pub evidence: Vec<Evidence>,
    pub artifacts: Vec<ArtifactRef>,
    pub metrics: CollectorMetrics,
}
```

### 13.1 MVP Collectors

| Collector | OS | Actions | Evidence Types |
|-----------|-----|---------|----------------|
| `proc_windows` | Windows | `list`, `get`, `tree` | `artifact/process` |
| `proc_linux` | Linux | `list`, `get`, `tree` | `artifact/process` |
| `file_windows` | Windows | `enumerate`, `hash`, `collect` | `artifact/file`, `artifact/ntfs` |
| `file_linux` | Linux | `enumerate`, `hash`, `collect` | `artifact/file` |
| `registry_windows` | Windows | `enumerate`, `get`, `monitor` | `artifact/reg` |
| `config_linux` | Linux | `enumerate`, `get` | `artifact/config` |
| `logs_windows` | Windows | `query`, `export`, `tail` | `artifact/evt` |
| `logs_linux` | Linux | `query`, `export`, `tail` | `artifact/journald`, `artifact/syslog` |
| `net_windows` | Windows | `connections`, `listen`, `capture` | `artifact/connection` |
| `net_linux` | Linux | `connections`, `listen`, `capture` | `artifact/connection` |

Full specs: `collectors/windows/README.md`, `collectors/linux/README.md`

---

## 14. Correlation Model

### 14.1 Deterministic Relations

| Relation | Source Type | Target Type | Method | Confidence |
|----------|-------------|-------------|--------|------------|
| `spawned` | Process | Process | PID/PPID link | 1.0 |
| `wrote` | Process | File | Handle + timestamp | 0.95 |
| `read` | Process | File | Handle + timestamp | 0.9 |
| `connected` | Process | NetworkConnection | PID + 4-tuple | 0.95 |
| `resolved_to` | NetworkConnection | Indicator | IOC match (IP/domain/hash) | 1.0 |
| `contains` | File | Indicator | YARA/Sigma match | 0.9 |
| `indicates` | Evidence[] | Finding | Rule match + evidence IDs | 0.8-1.0 |
| `supported_by` | Finding | Evidence[] | Explicit citation | 1.0 |
| `timeline_adjacent` | Evidence | Evidence | Temporal proximity | 0.7 |

### 14.2 Correlation Rules (Configuration)

```json
{
  "rules": [
    {
      "name": "pid_link",
      "relation": "spawned",
      "source_type": "artifact/process",
      "target_type": "artifact/process",
      "condition": "source.pid == target.ppid",
      "confidence": 1.0
    },
    {
      "name": "timestamp_join",
      "relation": "timeline_adjacent",
      "source_type": "*",
      "target_type": "*",
      "condition": "abs(source.timestamp - target.timestamp) <= window_seconds",
      "confidence": 0.7,
      "params": { "window_seconds": 300 }
    }
  ]
}
```

---

## 15. Investigation Graph

**Data Structure**: `petgraph::Graph<EvidenceNode, CorrelationEdge, Directed>`

**Node**: `EvidenceNode { evidence_id, evidence_type, timestamp, summary }`
**Edge**: `CorrelationEdge { relation, confidence, method, details, evidence_ids }`

**Queries:**
- `graph.path(from, to, max_hops)` — Causal chain
- `graph.backward_from(node, window)` — Precursor events
- `graph.forward_from(node, window)` — Consequent events
- `graph.hypothesis_support(hypothesis_id)` — All evidence citing hypothesis
- `graph.subgraph(evidence_ids)` — Induced subgraph for export

---

## 16. API Direction

**REST Endpoints (Axum):**

| Method | Path | Purpose |
|--------|------|---------|
| POST | `/api/v1/investigations` | Create investigation from script |
| GET | `/api/v1/investigations/{id}` | Get investigation status |
| GET | `/api/v1/investigations/{id}/graph` | Get investigation graph (Cytoscape JSON) |
| GET | `/api/v1/investigations/{id}/timeline` | Get timeline events |
| GET | `/api/v1/investigations/{id}/evidence` | List evidence objects |
| GET | `/api/v1/investigations/{id}/evidence/{eid}` | Get evidence + receipt + provenance |
| GET | `/api/v1/investigations/{id}/findings` | Get findings |
| POST | `/api/v1/investigations/{id}/verify` | Trigger verification |
| GET | `/api/v1/investigations/{id}/export/{format}` | Export CASE/UCO/Markdown |
| WS | `/api/v1/investigations/{id}/stream` | Live execution updates |

**WebSocket Events:**
- `step_started`, `step_completed`, `step_failed`
- `evidence_collected`, `receipt_generated`
- `correlation_found`, `finding_created`
- `verification_result`, `export_ready`

---

## 17. Storage Model

### 17.1 SQLite Schema (rusqlite)

```sql
-- Investigations
CREATE TABLE investigations (
    id TEXT PRIMARY KEY,           -- ULID
    script_hash TEXT NOT NULL,     -- blake3 of source script
    ir_json TEXT NOT NULL,         -- Serialized IR
    contract_json TEXT,            -- Serialized ExecutionContract
    status TEXT NOT NULL,          -- pending, running, completed, failed
    created_at TEXT NOT NULL,      -- RFC3339 UTC
    updated_at TEXT NOT NULL
);

-- Evidence Objects
CREATE TABLE evidence (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    step_id TEXT NOT NULL,
    collector TEXT NOT NULL,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,            -- Relative to evidence_root
    size INTEGER NOT NULL,
    hash TEXT NOT NULL,            -- Hex blake3
    hash_algo TEXT NOT NULL,
    metadata TEXT,                 -- JSON
    collected_at TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES investigations(id)
);

-- Evidence Receipts
CREATE TABLE receipts (
    id TEXT PRIMARY KEY,
    evidence_id TEXT NOT NULL UNIQUE,
    run_id TEXT NOT NULL,
    collector TEXT NOT NULL,
    collector_version TEXT NOT NULL,
    signature TEXT NOT NULL,
    public_key TEXT NOT NULL,
    payload_hash TEXT NOT NULL,
    payload_hash_algo TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    host_info TEXT NOT NULL,       -- JSON
    FOREIGN KEY (evidence_id) REFERENCES evidence(id)
);

-- Provenance Chain
CREATE TABLE provenance (
    id TEXT PRIMARY KEY,
    evidence_id TEXT NOT NULL,
    receipt_id TEXT NOT NULL,
    prev_provenance_id TEXT,       -- NULL for root
    action TEXT NOT NULL,
    actor TEXT NOT NULL,
    actor_version TEXT NOT NULL,
    input_hashes TEXT NOT NULL,    -- JSON array
    output_hashes TEXT NOT NULL,   -- JSON array
    parameters TEXT,               -- JSON
    timestamp TEXT NOT NULL,
    signature TEXT NOT NULL,
    FOREIGN KEY (evidence_id) REFERENCES evidence(id),
    FOREIGN KEY (receipt_id) REFERENCES receipts(id)
);

-- Correlation Edges
CREATE TABLE correlations (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    src_evidence_id TEXT NOT NULL,
    dst_evidence_id TEXT NOT NULL,
    relation TEXT NOT NULL,
    confidence REAL NOT NULL,
    method TEXT NOT NULL,
    details TEXT,                  -- JSON
    created_at TEXT NOT NULL,
    created_by TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES investigations(id)
);

-- Findings
CREATE TABLE findings (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    rule_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    severity TEXT NOT NULL,        -- critical, high, medium, low, info
    status TEXT NOT NULL,          -- open, triaged, false_positive, resolved
    evidence_ids TEXT NOT NULL,    -- JSON array
    correlation_ids TEXT,          -- JSON array
    tags TEXT,                     -- JSON array (MITRE ATT&CK)
    metadata TEXT,                 -- JSON
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES investigations(id)
);

-- Indexes
CREATE INDEX idx_evidence_run ON evidence(run_id);
CREATE INDEX idx_receipts_run ON receipts(run_id);
CREATE INDEX idx_provenance_evidence ON provenance(evidence_id);
CREATE INDEX idx_correlations_run ON correlations(run_id);
CREATE INDEX idx_findings_run ON findings(run_id);
```

---

## 18. Frontend Model

**Stack**: Static HTML/CSS/JS (no build step) + Chart.js + Cytoscape.js

### Pages

| Page | Purpose | Components |
|------|---------|------------|
| `index.html` | Dashboard overview | Recent investigations, quick stats |
| `investigation.html` | Investigation detail | Tabs: Graph, Timeline, Evidence, Findings, Export |
| `graph.html` | Cytoscape graph view | Interactive graph, filters, hypothesis overlay |
| `timeline.html` | Chart.js timeline | Zoomable, filterable, annotation layer |
| `evidence.html` | Evidence browser | Table + detail pane with receipt/provenance |
| `findings.html` | Findings board | Kanban (open/triaged/resolved), citations |
| `verify.html` | Verification report | Tamper check results, hash chain validation |
| `report.html` | Generated report view | Markdown render with evidence links |

**Static Assets**: Served by Axum `tower_http::services::ServeDir`

---

## 19. Security Principles

| Principle | Implementation |
|-----------|----------------|
| **Least Privilege** | Execution Contract declares required capabilities; runtime enforces |
| **Capability-Based** | Collectors registered with capability manifests; policy validates |
| **Immutable Evidence** | Evidence written once; hash computed; receipt signed immediately |
| **Verifiable Provenance** | Hash-linked chain; each actor signs transformations |
| **Tamper Detection** | Verification re-validates all receipts + provenance hashes |
| **No Arbitrary Code** | DSL has no eval, no FFI, no dynamic code loading |
| **Parameter Validation** | All collector params validated against JSON Schema |
| **Timeout Bounds** | Per-step and global timeouts enforced by runtime |
| **Path Isolation** | Evidence root chrooted; no absolute paths in params |
| **Version Pinning** | Collector versions pinned in contract; mismatch = rejection |

---

## 20. MVP Scope

| Component | In MVP | Notes |
|-----------|--------|-------|
| DSL Parser (pest) | Planned | 10 statements, full grammar |
| AST + Type Checker | Planned | Capability inference |
| IR Compiler | Planned | Platform-agnostic |
| Execution Contract | Planned | Policy validation |
| Runtime Engine | Planned | Sequential + parallel dispatch |
| Evidence Model | Planned | 7 types, ULIDs, SHA-256 |
| Evidence Receipts | Planned | Ed25519, collector keys |
| Provenance Chain | Planned | Hash-linked, signed |
| Windows Collectors | Planned | 5 collectors (proc, file, reg, logs, net) |
| Linux Collectors | Planned | 5 collectors (proc, file, config, logs, net) |
| Correlation Engine | Planned | 9 deterministic rules |
| Investigation Graph | Planned | petgraph, queries, export |
| Verification Engine | Planned | Receipt + provenance validation |
| Axum API | Planned | REST + WebSocket |
| Frontend Dashboard | Planned | 8 pages, Cytoscape + Chart.js |
| CASE/UCO Export | Planned | Core classes mapped |
| Timesketch Export | Planned | CSV/JSON compatible |
| CLI Tool | Planned | `jockey` binary with subcommands |

---

## 21. Optional Features (Post-MVP)

| Feature | Description | Effort |
|---------|-------------|--------|
| eBPF Collectors (Linux) | Aya-based syscall/socket/file tracing | High |
| Kernel Drivers (Windows) | ETW kernel provider, minifilter | Very High |
| Remote Agent Protocol | gRPC/mTLS, capability negotiation | High |
| Multi-tenancy | Orgs, RBAC, data isolation | Medium |
| AI Assist (Local) | Hypothesis suggestion, timeline summarization | Medium |
| Collaborative Investigation | CRDT-based multi-analyst editing | High |
| Cloud Collection | AWS/Azure/GCP snapshot APIs | Medium |
| Container Forensics | Docker/containerd artifact collection | Medium |
| Mobile Forensics | iOS/Android backup parsing | High |
| Formal Verification | Prove IR safety properties (Prusti/Kani) | Very High |

---

## 22. Future Features (Research)

| Feature | Description |
|---------|-------------|
| WASM Collector Plugins | Sandboxed, portable collector extensions |
| Distributed Execution | Multi-target coordination, result aggregation |
| Query Optimization | IR rewrite rules for collector pushdown |
| Differential Investigation | Compare two runs, highlight delta |
| Evidence Notarization | Timestamping via blockchain/Trusted Timestamping |
| Legal Hold Integration | Automatic preservation workflows |
| Threat Intel Fusion | Real-time IOC enrichment in correlation |

---

## 23. Implementation Status Matrix

| Component | Status | Spec Location | Implementation Location |
|-----------|--------|---------------|------------------------|
| JOCKY DSL Grammar | SPECIFIED | `docs/language/grammar.md` | `crates/jockey-language/src/parser.pest` |
| Lexer/Parser | NOT IMPLEMENTED | — | `crates/jockey-language/src/parser.rs` |
| AST Definition | SPECIFIED | `docs/language/ast.md` | `crates/jockey-language/src/ast.rs` |
| Type Checker | NOT IMPLEMENTED | — | `crates/jockey-language/src/typeck.rs` |
| IR Definition | SPECIFIED | `docs/architecture/system-architecture.md` | `crates/jockey-core/src/ir.rs` |
| IR Compiler | NOT IMPLEMENTED | — | `crates/jockey-core/src/compiler.rs` |
| Execution Contract | SPECIFIED | `docs/architecture/execution-contract.md` | `crates/jockey-core/src/contract.rs` |
| Policy Engine | NOT IMPLEMENTED | — | `crates/jockey-core/src/policy.rs` |
| Runtime Engine | NOT IMPLEMENTED | — | `crates/jockey-runtime/src/executor.rs` |
| Evidence Model | SPECIFIED | `docs/evidence/evidence-model.md` | `crates/jockey-evidence/src/model.rs` |
| Evidence Receipts | SPECIFIED | `docs/evidence/evidence-receipt.md` | `crates/jockey-evidence/src/receipt.rs` |
| Provenance Chain | SPECIFIED | `docs/evidence/provenance.md` | `crates/jockey-evidence/src/provenance.rs` |
| Verification Engine | NOT IMPLEMENTED | — | `crates/jockey-evidence/src/verify.rs` |
| Collector Trait | SPECIFIED | `docs/architecture/collector-architecture.md` | `crates/jockey-collectors/src/trait.rs` |
| Windows Collectors (5) | NOT IMPLEMENTED | `collectors/windows/README.md` | `crates/jockey-collectors/src/windows/` |
| Linux Collectors (4) | NOT IMPLEMENTED | `collectors/linux/README.md` | `crates/jockey-collectors/src/linux/` |
| Correlation Rules | SPECIFIED | `docs/architecture/correlation-architecture.md` | `crates/jockey-correlation/src/rules.rs` |
| Correlation Engine | NOT IMPLEMENTED | — | `crates/jockey-correlation/src/engine.rs` |
| Investigation Graph | SPECIFIED | `docs/architecture/data-flow.md` | `crates/jockey-correlation/src/graph.rs` |
| Axum API Server | NOT IMPLEMENTED | — | `crates/jockey-api/src/server.rs` |
| Frontend Dashboard | NOT IMPLEMENTED | `frontend/README.md` | `frontend/` |
| SQLite Persistence | NOT IMPLEMENTED | — | `crates/jockey-core/src/storage.rs` |
| CLI Tool | NOT IMPLEMENTED | — | `crates/jockey-cli/src/main.rs` |
| CASE/UCO Export | SPECIFIED | `docs/evidence/verification.md` | `crates/jockey-evidence/src/export.rs` |
| Architecture Diagrams | SPECIFIED | `assets/diagrams/` | Mermaid in Markdown |
| Research Documentation | SPECIFIED | `docs/research/` | Markdown |
| Phase 0 Self-Assessment | COMPLETE | `PHASE_0_SELF_ASSESSMENT.md` | — |
| Handoff Document | SPECIFIED | `HANDOFF.md` | — |

---

## 24. Versioning & Compatibility

| Artifact | Versioning | Compatibility |
|----------|------------|---------------|
| DSL Grammar | SemVer (language) | Breaking = major; additive = minor |
| IR Schema | Integer version in IR | Forward compatible within major |
| Evidence Model | Integer version in Evidence | Additive fields only |
| Receipt Format | Integer version in Receipt | Additive fields only |
| Collector Interface | SemVer (crate) | Trait stable; new actions = minor |
| Correlation Rules | Config version | Additive rules only |
| API | SemVer (OpenAPI) | Standard REST versioning |

---

## 25. Testing Requirements

| Test Type | Coverage Target | Tools |
|-----------|----------------|-------|
| Unit Tests | ≥90% per crate | `cargo test`, `proptest` |
| Integration Tests | Full pipeline (script→graph→export) | `cargo test --test integration` |
| Property Tests | Parser roundtrip, hash determinism | `proptest` |
| Fuzzing | Parser, JSON deserialization | `cargo fuzz` |
| Contract Tests | Collector trait compliance | Custom test harness |
| Verification Tests | Tamper detection (positive/negative) | Golden files |

---

**Document Version**: 1.0  
**Last Updated**: October 2026  
**Status**: FROZEN — Changes require architecture review