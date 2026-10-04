# Deployment Model

**Status**: SPECIFIED

---

## Deployment Models

| Model | Description | Status | Use Case |
|-------|-------------|--------|----------|
| **Single Binary (MVP)** | One `jockey` binary runs API + Runtime + Collectors | SPECIFIED | Local investigation, SIH demo |
| **Server + Agents (Future)** | Central server manages remote collectors via gRPC/mTLS | PLANNED | Fleet investigation |
| **Air-Gapped (Future)** | Offline binary + pre-compiled collector bundles | PLANNED | Classified environments |

---

## MVP: Single Binary Deployment

### Build

```bash
cargo build --release --workspace
```

### Binary Structure

```
jockey
├── API Server (Axum)
├── Runtime Engine
├── Collector Registry
│   ├── proc_windows
│   ├── file_windows
│   ├── registry_windows
│   ├── logs_windows
│   ├── net_windows
│   ├── proc_linux
│   ├── file_linux
│   ├── config_linux
│   ├── logs_linux
│   └── net_linux
├── SQLite Storage (embedded)
└── Frontend Assets (embedded)
```

### Run Server

```bash
./target/release/jockey serve \
  --addr 0.0.0.0:8080 \
  --evidence-root ./evidence \
  --db-path ./evidence/jockey.db
```

### Run Investigation (CLI)

```bash
# Local execution
./target/release/jockey run investigate.jky \
  --target live \
  --host WORKSTATION-01 \
  --os windows

# Via API
curl -X POST http://localhost:8080/api/v1/investigations \
  -H "Content-Type: application/json" \
  -d '{
    "script": "investigation \"test\" { collect process as p }",
    "target": { "type": "live", "identifier": "HOST-01", "os": "windows" }
  }'
```

---

## Components

| Component | Technology | Port | Data |
|-----------|------------|------|------|
| **API Server** | Axum | 8080 (HTTP) | REST + WebSocket |
| **Runtime** | Embedded in binary | — | In-memory execution |
| **Collector Registry** | Embedded | — | Trait objects |
| **SQLite DB** | rusqlite | — | `jockey.db` (in evidence_root) |
| **Evidence Files** | Filesystem | — | `evidence_root/run_id/` |
| **Frontend** | Static files | Served by Axum | `frontend/` |

---

## Target Access Methods

| Target Type | Access Method | Collectors |
|-------------|---------------|------------|
| Windows Live | WinRM / SSH / Local | All 5 Windows collectors |
| Linux Live | SSH / Local | All 5 Linux collectors |
| Windows Disk Image | Mount (read-only) | `file_windows` (MFT, USN, VSS) |
| Linux Disk Image | Mount (read-only) | `file_linux` |
| Memory Dump | File read | Future: Volatility integration |
| Network PCAP | File read | Future: Zeek/tshark integration |

---

## Security Boundaries

```
┌─────────────────────────────────────────────────────────┐
│                    JOCKY PROCESS                         │
├─────────────────────────────────────────────────────────┤
│  API (untrusted input) → Validation → Runtime (trusted) │
│                                    │                     │
│                                    ▼                     │
│                          Collector Execution            │
│                          (capability-scoped)            │
│                                    │                     │
│                                    ▼                     │
│                          Evidence Root (chrooted)       │
└─────────────────────────────────────────────────────────┘
```

- **API**: Validates all input (script, target, params)
- **Runtime**: Executes validated contracts only
- **Collectors**: Run with minimal privileges; no arbitrary code
- **Evidence Root**: Isolated per-run directory; no path traversal
- **Database**: Local SQLite; no network exposure

---

## Evidence Root Isolation

```
evidence_root/
├── run_01ARZ.../
│   ├── evidence/
│   │   ├── ULID1.jsonl
│   │   └── ULID2.jsonl
│   ├── receipts/ (in SQLite)
│   └── provenance/ (in SQLite)
└── run_01ARZ.../
    └── ...
```

- Each run gets unique subdirectory (ULID-based)
- Collectors receive `evidence_root` in `ExecutionContext`
- Path traversal prevented: all paths relative to `evidence_root`
- `chroot`-style enforcement via runtime validation

---

## Configuration

### Config File (TOML)

```toml
# jockey.toml
[server]
addr = "0.0.0.0:8080"
evidence_root = "./evidence"
db_path = "./evidence/jockey.db"

[policy]
default_parallelism = 2
default_timeout_ms = 30000
default_hash_algorithm = "blake3"
max_parallelism = 16
max_timeout_ms = 300000

[collectors]
# Collector-specific config
proc_windows = { enabled = true }
file_windows = { enabled = true }
# ...

[logging]
level = "info"
format = "json"
output = "stdout"
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `JOCKY_EVIDENCE_ROOT` | Evidence root directory | `./evidence` |
| `JOCKY_DB_PATH` | SQLite database path | `./evidence/jockey.db` |
| `JOCKY_SERVER_ADDR` | API server address | `0.0.0.0:8080` |
| `JOCKY_LOG_LEVEL` | Logging level | `info` |
| `JOCKY_CONFIG` | Config file path | `./jockey.toml` |

---

## Future: Distributed Deployment

```
┌─────────────┐     gRPC/mTLS      ┌──────────────────┐
│ JOCKY       │ ◀─────────────────▶ │ Collector Agent  │
│ Controller  │   Capability Negotiation │ (Windows/Linux) │
└─────────────┘                     └──────────────────┘
       │                                      │
       ▼                                      ▼
┌─────────────┐                     ┌──────────────────┐
│ SQLite/     │                     │ Local Evidence   │
│ PostgreSQL  │                     │ Cache            │
└─────────────┘                     └──────────────────┘
```

### Controller Responsibilities

- Manage agent fleet (enrollment, health, capabilities)
- Validate and distribute execution contracts
- Aggregate evidence from agents
- Provide unified API for analysts

### Agent Responsibilities

- Register capabilities with controller
- Execute assigned steps
- Stream evidence/receipts to controller
- Local evidence cache for resilience

### Communication

- **Protocol**: gRPC with mTLS
- **Authentication**: Certificate-based (controller CA + agent certs)
- **Authorization**: Capability-based (agent declares, controller validates)
- **Evidence Streaming**: Protobuf over gRPC streaming

---

## Air-Gapped Deployment (Future)

### Bundle Structure

```
jockey-airgapped/
├── jockey                    # Main binary
├── collectors/
│   ├── windows/
│   │   ├── proc_windows.dll
│   │   ├── file_windows.dll
│   │   └── ...
│   └── linux/
│       ├── proc_linux.so
│       ├── file_linux.so
│       └── ...
├── frontend/                 # Static assets
├── jockey.toml              # Config
└── collector-manifests/     # JSON manifests with public keys
```

### Workflow

1. Build bundle on connected build machine
2. Transfer bundle to air-gapped environment (USB, DVD)
3. Run `jockey` directly on target or via local server
4. Export evidence bundles for transfer back

---

## Related Documents

- `overview.md` — Architecture overview
- `system-architecture.md` — Component architecture
- `data-flow.md` — Evidence pipeline and data flow
- `execution-contract.md` — Execution Contract specification
- `collector-architecture.md` — Collector trait and implementations
- `correlation-architecture.md` — Correlation rules and graph
- `evidence-pipeline.md` — Evidence pipeline details