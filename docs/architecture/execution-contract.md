# Execution Contract Specification

**Status**: SPECIFIED

---

## Purpose

The **Execution Contract** is the security boundary between investigator intent (JOCKY script) and action (collector execution). It binds the platform-agnostic IR to a specific target with enforced policy.

---

## Contract Structure

```json
{
  "ir_id": "ULID",
  "run_id": "ULID",
  "target": {
    "type": "live|dead|image|remote",
    "identifier": "HOST-01",
    "os": "windows|linux|darwin",
    "arch": "x86_64|aarch64"
  },
  "steps": [
    {
      "step_id": "ULID",
      "collector": "windows_evt",
      "action": "query",
      "params": { "channel": "Security", "query": "*[System/EventID=4625]" },
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

---

## Field Specifications

### Root Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ir_id` | ULID | ✅ | Source IR module identifier |
| `run_id` | ULID | ✅ | Unique execution identifier |
| `target` | TargetSpec | ✅ | Target specification |
| `steps` | Vec<ExecStep> | ✅ | Ordered, resolved execution steps |
| `policy` | ExecPolicy | ✅ | Execution policy |
| `context` | Value | ❌ | Runtime context (env, credentials) |

### TargetSpec

| Field | Type | Required | Values |
|-------|------|----------|--------|
| `type` | String | ✅ | `live`, `dead`, `image`, `remote` |
| `identifier` | String | ✅ | Hostname, path, IP, etc. |
| `os` | String | ✅ | `windows`, `linux`, `darwin` |
| `arch` | String | ❌ | `x86_64`, `aarch64` |

### ExecStep

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `step_id` | ULID | ✅ | From IRStep.id |
| `collector` | String | ✅ | Collector plugin name |
| `action` | String | ✅ | Action to invoke |
| `params` | Value | ✅ | Resolved parameters (validated) |
| `output_vars` | Vec<String> | ❌ | Variables this step produces |
| `timeout_ms` | u64 | ✅ | Per-step timeout (1000-300000) |
| `retry` | RetryPolicy | ❌ | Retry configuration |

### RetryPolicy

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `max_attempts` | u32 | ✅ | 1 |
| `backoff_ms` | u64 | ✅ | 1000 |

### ExecPolicy

| Field | Type | Required | Default | Constraints |
|-------|------|----------|---------|-------------|
| `parallelism` | u32 | ✅ | 1 | 1-16 |
| `continue_on_error` | bool | ✅ | false | — |
| `evidence_root` | String | ✅ | — | Absolute path, created per run |
| `hash_algorithm` | String | ✅ | `blake3` | `blake3` or `sha256` |

---

## Contract Building Process

```
IR Module + TargetSpec + Policy
         │
         ▼
┌─────────────────────────────────────┐
│ 1. Validate Target OS               │
│    - Check collector availability   │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ 2. Resolve Collector for Each Step  │
│    - Map IR collector → platform    │
│    - e.g., "process" → "proc_windows" │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ 3. Validate Parameters              │
│    - JSON Schema per collector/action│
│    - Reject unknown parameters      │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ 4. Inject Policy                    │
│    - Apply timeouts, retry, root    │
│    - Set hash algorithm             │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ 5. Capability Validation            │
│    - Check declared capabilities    │
│    - Verify against collector manifest│
└─────────────────────────────────────┘
         │
         ▼
Execution Contract (validated)
```

---

## Capability System

### Capability Declaration (Inferred from Script)

```rust
enum Capability {
    ProcessEnumerate,
    ProcessGet,
    ProcessTree,
    FileEnumerate,
    FileHash,
    FileCollect,
    RegistryEnumerate,
    RegistryGet,
    RegistryMonitor,
    LogQuery,
    LogExport,
    LogTail,
    NetworkConnections,
    NetworkListen,
    NetworkCapture,
    NetworkResolve,
}
```

### Capability Inference Rules

| DSL Statement | Inferred Capabilities |
|---------------|----------------------|
| `collect process as x where ...` | `ProcessEnumerate` (+ `ProcessGet` if PID filter) |
| `collect file as x where ...` | `FileEnumerate` (+ `FileHash`, `FileCollect` if collect action) |
| `collect registry as x where ...` | `RegistryEnumerate` (+ `RegistryGet`) |
| `collect logs as x where ...` | `LogQuery` (+ `LogExport`) |
| `collect network as x where ...` | `NetworkConnections` (+ `NetworkListen`) |
| `correlate` | None (local computation) |
| `timeline` | None (local computation) |

### Collector Manifest

Each collector registers its capabilities:

```json
{
  "name": "proc_windows",
  "version": "0.1.0",
  "supported_os": ["windows"],
  "actions": {
    "list": { "capabilities": ["ProcessEnumerate"], "params_schema": {...} },
    "get": { "capabilities": ["ProcessGet"], "params_schema": {...} },
    "tree": { "capabilities": ["ProcessTree"], "params_schema": {...} }
  }
}
```

---

## Policy Validation

### Validation Steps

1. **Target Validation**
   - OS supported by at least one collector per step
   - Architecture compatible

2. **Collector Validation**
   - Each step's collector exists in registry
   - Collector version matches (or compatible)
   - Collector supports required action

3. **Parameter Validation**
   - JSON Schema validation per collector/action
   - No extra parameters allowed
   - Required parameters present

4. **Capability Validation**
   - Script-inferred capabilities ⊆ Collector-declared capabilities
   - No undeclared capabilities used

5. **Policy Bounds Validation**
   - `parallelism` ∈ [1, 16]
   - `timeout_ms` ∈ [1000, 300000]
   - `evidence_root` is absolute path, writable
   - `hash_algorithm` ∈ {`blake3`, `sha256`}

6. **Dependency Validation**
   - `depends_on` references valid step IDs
   - No circular dependencies
   - Topological sort possible

### Validation Errors

| Error Code | Description |
|------------|-------------|
| `UNSUPPORTED_OS` | No collector for target OS |
| `COLLECTOR_NOT_FOUND` | Referenced collector not in registry |
| `ACTION_NOT_SUPPORTED` | Collector doesn't support action |
| `PARAM_VALIDATION_FAILED` | Parameter schema validation failed |
| `CAPABILITY_DENIED` | Required capability not in collector manifest |
| `INVALID_TIMEOUT` | Timeout outside bounds |
| `INVALID_EVIDENCE_ROOT` | Path not absolute or not writable |
| `CYCLIC_DEPENDENCY` | Circular `depends_on` references |
| `HASH_ALGO_UNSUPPORTED` | Algorithm not blake3 or sha256 |

---

## Execution Contract Lifecycle

```
Build Contract (IR + Target + Policy)
         │
         ▼
Validate Contract (Policy Engine)
         │
         ├──── FAIL ────▶ Return PolicyError to Analyst
         │
         ▼ PASS
Store Contract (SQLite)
         │
         ▼
Runtime Execution
         │
         ├──── STEP FAIL ────▶ Retry per policy / Continue on error
         │
         ▼
Persist Evidence + Receipts + Provenance
         │
         ▼
Contract Complete → Mark Investigation Status
```

---

## Related Documents

- `overview.md` — Architecture overview
- `system-architecture.md` — Component architecture
- `data-flow.md` — Evidence pipeline and data flow
- `collector-architecture.md` — Collector trait and implementations
- `correlation-architecture.md` — Correlation rules and graph
- `deployment-model.md` — Deployment topology and models
- `evidence-pipeline.md` — Evidence pipeline details