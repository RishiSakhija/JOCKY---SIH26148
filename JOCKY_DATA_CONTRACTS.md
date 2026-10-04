# JOCKY Data Contracts

## 1. IR (Intermediate Representation)

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `version` | `u32` | ✓ | IR schema version |
| `id` | `String` | ✓ | ULID of this IR module |
| `metadata` | `IRMetadata` | ✓ | Module metadata |
| `steps` | `Vec<IRStep>` | ✓ | Ordered investigation steps |
| `outputs` | `Vec<IROutput>` | | Declared output bindings |

**IRMetadata**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `name` | `String` | ✓ | Human-readable name |
| `description` | `String` | | Free text |
| `author` | `String` | | Author identifier |
| `created_at` | `DateTime<Utc>` | ✓ | Creation timestamp |
| `tags` | `Vec<String>` | | Searchable tags |

**IRStep**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `id` | `String` | ✓ | ULID, unique in module |
| `collector` | `String` | ✓ | Collector plugin name |
| `action` | `String` | ✓ | Collector action/method |
| `params` | `serde_json::Value` | ✓ | Action parameters |
| `binds` | `Vec<String>` | | Output variable names |
| `depends_on` | `Vec<String>` | | Step IDs this depends on |
| `condition` | `Option<String>` | | Optional guard expression |

**IROutput**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `name` | `String` | ✓ | Output variable name |
| `from_step` | `String` | ✓ | Step ID producing this output |
| `field` | `String` | | Specific field to extract |

---

## 2. Execution Contract

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `ir_id` | `String` | ✓ | Source IR module ULID |
| `run_id` | `String` | ✓ | ULID for this execution |
| `target` | `TargetSpec` | ✓ | Target specification |
| `steps` | `Vec<ExecStep>` | ✓ | Resolved, ordered steps |
| `policy` | `ExecPolicy` | ✓ | Execution policy |
| `context` | `serde_json::Value` | | Runtime context (env, creds) |

**TargetSpec**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `type` | `String` | ✓ | `live`, `dead`, `image`, `remote` |
| `identifier` | `String` | ✓ | Hostname, path, IP, etc. |
| `os` | `String` | ✓ | `windows`, `linux`, `darwin` |
| `arch` | `String` | | `x86_64`, `aarch64` |

**ExecStep**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `step_id` | `String` | ✓ | From IRStep.id |
| `collector` | `String` | ✓ | Collector plugin name |
| `action` | `String` | ✓ | Action to invoke |
| `params` | `serde_json::Value` | ✓ | Resolved parameters |
| `output_vars` | `Vec<String>` | | Variables this step produces |
| `timeout_ms` | `u64` | ✓ | Per-step timeout |
| `retry` | `RetryPolicy` | | Retry configuration |

**RetryPolicy**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `max_attempts` | `u32` | ✓ | Default 1 |
| `backoff_ms` | `u64` | ✓ | Base backoff |

**ExecPolicy**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `parallelism` | `u32` | ✓ | Max concurrent steps |
| `continue_on_error` | `bool` | ✓ | Default false |
| `evidence_root` | `String` | ✓ | Output directory |
| `hash_algorithm` | `String` | ✓ | `sha256`, `blake3` |

---

## 3. Evidence Object

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `id` | `String` | ✓ | ULID |
| `run_id` | `String` | ✓ | Parent execution |
| `step_id` | `String` | ✓ | Producing step |
| `collector` | `String` | ✓ | Source collector |
| `type` | `String` | ✓ | MIME-like: `artifact/reg`, `file/ntfs`, `memory/process` |
| `name` | `String` | ✓ | Human name |
| `path` | `String` | ✓ | Local filesystem path |
| `size` | `u64` | ✓ | Bytes |
| `hash` | `String` | ✓ | Hex-encoded hash |
| `hash_algo` | `String` | ✓ | Algorithm used |
| `metadata` | `serde_json::Value` | | Collector-specific metadata |
| `collected_at` | `DateTime<Utc>` | ✓ | Collection timestamp |

---

## 4. Evidence Receipt

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `id` | `String` | ✓ | ULID |
| `evidence_id` | `String` | ✓ | Evidence.object.id |
| `run_id` | `String` | ✓ | Parent execution |
| `collector` | `String` | ✓ | Collector identity |
| `collector_version` | `String` | ✓ | Collector semver |
| `signature` | `String` | ✓ | Ed25519 signature over receipt |
| `public_key` | `String` | ✓ | Collector public key (hex) |
| `payload_hash` | `String` | ✓ | Hash of evidence payload |
| `payload_hash_algo` | `String` | ✓ | Hash algorithm |
| `timestamp` | `DateTime<Utc>` | ✓ | Receipt creation time |
| `host_info` | `HostInfo` | ✓ | Collector host details |

**HostInfo**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `hostname` | `String` | ✓ | |
| `os` | `String` | ✓ | |
| `kernel` | `String` | | |
| `collector_pid` | `u32` | ✓ | |

---

## 5. Provenance Record

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `id` | `String` | ✓ | ULID |
| `evidence_id` | `String` | ✓ | Evidence.object.id |
| `receipt_id` | `String` | ✓ | EvidenceReceipt.id |
| `prev_provenance_id` | `Option<String>` | | Chain link |
| `action` | `String` | ✓ | `collected`, `transformed`, `correlated`, `exported` |
| `actor` | `String` | ✓ | Tool/agent identifier |
| `actor_version` | `String` | ✓ | Tool version |
| `input_hashes` | `Vec<String>` | | Input evidence hashes |
| `output_hashes` | `Vec<String>` | | Output evidence hashes |
| `parameters` | `serde_json::Value` | | Transformation parameters |
| `timestamp` | `DateTime<Utc>` | ✓ | Action timestamp |
| `signature` | `String` | ✓ | Actor signature |

---

## 6. Collector Interface

```rust
#[async_trait]
trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn supported_actions(&self) -> Vec<&'static str>;
    fn supported_os(&self) -> Vec<&'static str>;
    async fn execute(
        &self,
        action: &str,
        params: serde_json::Value,
        context: &ExecutionContext,
    ) -> Result<CollectorOutput, CollectorError>;
}
```

**ExecutionContext**
| Field | Type | Purpose |
|-------|------|---------|
| `run_id` | `String` | |
| `evidence_root` | `PathBuf` | |
| `target` | `TargetSpec` | |
| `temp_dir` | `PathBuf` | |
| `credentials` | `Option<serde_json::Value>` | |

**CollectorOutput**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `evidence` | `Vec<Evidence>` | ✓ | Produced evidence objects |
| `artifacts` | `Vec<ArtifactRef>` | | References to side artifacts |
| `metrics` | `CollectorMetrics` | | Timing, bytes, errors |

**ArtifactRef**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `path` | `String` | ✓ | Relative to evidence_root |
| `type` | `String` | ✓ | MIME-like type |
| `description` | `String` | | |

**CollectorMetrics**
| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `duration_ms` | `u64` | ✓ | |
| `bytes_read` | `u64` | | |
| `bytes_written` | `u64` | | |
| `errors` | `Vec<String>` | | |

---

## 7. Correlation Edge

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `id` | `String` | ✓ | ULID |
| `run_id` | `String` | ✓ | Parent execution |
| `src_evidence_id` | `String` | ✓ | Source evidence |
| `dst_evidence_id` | `String` | ✓ | Destination evidence |
| `relation` | `String` | ✓ | `parent_of`, `contains`, `references`, `timeline_adjacent`, `network_flow` |
| `confidence` | `f32` | ✓ | 0.0–1.0 |
| `method` | `String` | ✓ | `timestamp_join`, `pid_link`, `path_match`, `custom` |
| `details` | `serde_json::Value` | | Method-specific detail |
| `created_at` | `DateTime<Utc>` | ✓ | |
| `created_by` | `String` | ✓ | Correlation engine identifier |

---

## 8. Finding Object

| Field | Type | Req | Purpose |
|-------|------|-----|---------|
| `id` | `String` | ✓ | ULID |
| `run_id` | `String` | ✓ | Parent execution |
| `rule_id` | `String` | ✓ | Detection rule identifier |
| `title` | `String` | ✓ | Short title |
| `description` | `String` | | Detail |
| `severity` | `String` | ✓ | `critical`, `high`, `medium`, `low`, `info` |
| `status` | `String` | ✓ | `open`, `triaged`, `false_positive`, `resolved` |
| `evidence_ids` | `Vec<String>` | ✓ | Supporting evidence |
| `correlation_ids` | `Vec<String>` | | Supporting correlation edges |
| `tags` | `Vec<String>` | | MITRE ATT&CK, etc. |
| `metadata` | `serde_json::Value` | | Rule-specific data |
| `created_at` | `DateTime<Utc>` | ✓ | |
| `updated_at` | `DateTime<Utc>` | ✓ | |

---

## 9. End-to-End JSON Example

### JOCKY Command (DSL)
```jocky
investigation "suspicious-logon" {
    collect windows.security_log as logon_events where EventID == 4625
    collect windows.registry as hkcu_run where path == "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"
    correlate logon_events -> hkcu_run by "timestamp_join" window 300s
    emit finding "brute_force_persistence" severity high
}
```

### IR
```json
{
  "version": 1,
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "metadata": {
    "name": "suspicious-logon",
    "description": "Detect failed logons followed by persistence",
    "author": "analyst@org",
    "created_at": "2026-10-04T12:00:00Z",
    "tags": ["windows", "logon", "persistence"]
  },
  "steps": [
    {
      "id": "01ARZ3NDEKTSV4RRFFQ69G5FAW",
      "collector": "windows_evt",
      "action": "query",
      "params": { "channel": "Security", "query": "*[System/EventID=4625]" },
      "binds": ["logon_events"],
      "depends_on": [],
      "condition": null
    },
    {
      "id": "01ARZ3NDEKTSV4RRFFQ69G5FAX",
      "collector": "windows_reg",
      "action": "enumerate",
      "params": { "hive": "HKCU", "path": "Software\\Microsoft\\Windows\\CurrentVersion\\Run" },
      "binds": ["hkcu_run"],
      "depends_on": [],
      "condition": null
    },
    {
      "id": "01ARZ3NDEKTSV4RRFFQ69G5FAY",
      "collector": "correlator",
      "action": "timestamp_join",
      "params": { "left": "logon_events", "right": "hkcu_run", "window_seconds": 300 },
      "binds": ["correlated"],
      "depends_on": ["01ARZ3NDEKTSV4RRFFQ69G5FAW", "01ARZ3NDEKTSV4RRFFQ69G5FAX"],
      "condition": null
    }
  ],
  "outputs": [
    { "name": "findings", "from_step": "01ARZ3NDEKTSV4RRFFQ69G5FAY", "field": "findings" }
  ]
}
```

### Execution Contract
```json
{
  "ir_id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "run_id": "01ARZ3NDEKTSV4RRFFQ69G5FAZ",
  "target": {
    "type": "live",
    "identifier": "WORKSTATION-01",
    "os": "windows",
    "arch": "x86_64"
  },
  "steps": [
    {
      "step_id": "01ARZ3NDEKTSV4RRFFQ69G5FAW",
      "collector": "windows_evt",
      "action": "query",
      "params": { "channel": "Security", "query": "*[System/EventID=4625]" },
      "output_vars": ["logon_events"],
      "timeout_ms": 30000,
      "retry": { "max_attempts": 2, "backoff_ms": 1000 }
    },
    {
      "step_id": "01ARZ3NDEKTSV4RRFFQ69G5FAX",
      "collector": "windows_reg",
      "action": "enumerate",
      "params": { "hive": "HKCU", "path": "Software\\Microsoft\\Windows\\CurrentVersion\\Run" },
      "output_vars": ["hkcu_run"],
      "timeout_ms": 10000,
      "retry": { "max_attempts": 1, "backoff_ms": 500 }
    },
    {
      "step_id": "01ARZ3NDEKTSV4RRFFQ69G5FAY",
      "collector": "correlator",
      "action": "timestamp_join",
      "params": { "left": "logon_events", "right": "hkcu_run", "window_seconds": 300 },
      "output_vars": ["correlated"],
      "timeout_ms": 5000,
      "retry": { "max_attempts": 1, "backoff_ms": 100 }
    }
  ],
  "policy": {
    "parallelism": 2,
    "continue_on_error": false,
    "evidence_root": "C:\\jocky\\evidence\\01ARZ3NDEKTSV4RRFFQ69G5FAZ",
    "hash_algorithm": "blake3"
  },
  "context": {}
}
```

### Evidence (first piece)
```json
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FB0",
  "run_id": "01ARZ3NDEKTSV4RRFFQ69G5FAZ",
  "step_id": "01ARZ3NDEKTSV4RRFFQ69G5FAW",
  "collector": "windows_evt",
  "type": "artifact/evt",
  "name": "Security-4625-events",
  "path": "evidence/01ARZ3NDEKTSV4RRFFQ69G5FB0.jsonl",
  "size": 24576,
  "hash": "a1b2c3d4e5f6...",
  "hash_algo": "blake3",
  "metadata": { "event_count": 47, "earliest": "2026-10-04T10:00:00Z", "latest": "2026-10-04T11:55:00Z" },
  "collected_at": "2026-10-04T12:00:05Z"
}
```

### Evidence Receipt
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
  "host_info": {
    "hostname": "WORKSTATION-01",
    "os": "Windows 10 22H2",
    "kernel": "10.0.19045",
    "collector_pid": 4216
  }
}
```

### Finding
```json
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FB2",
  "run_id": "01ARZ3NDEKTSV4RRFFQ69G5FAZ",
  "rule_id": "brute_force_persistence",
  "title": "Failed logon burst followed by Run key modification",
  "description": "47 failed logons (EventID 4625) in 115 minutes, followed by new Run key value within 5 minutes",
  "severity": "high",
  "status": "open",
  "evidence_ids": ["01ARZ3NDEKTSV4RRFFQ69G5FB0", "01ARZ3NDEKTSV4RRFFQ69G5FB3"],
  "correlation_ids": ["01ARZ3NDEKTSV4RRFFQ69G5FB4"],
  "tags": ["T1110", "T1547.001"],
  "metadata": { "failed_logon_count": 47, "time_window_minutes": 115, "persistence_keys": ["malware.exe"] },
  "created_at": "2026-10-04T12:00:15Z",
  "updated_at": "2026-10-04T12:00:15Z"
}
```

---

## 10. Final Recommendation

**Use ULIDs everywhere**—monotonic, sortable, no coordination. **blake3 for hashing**—fast, parallel, 256-bit. **Ed25519 for signatures**—small keys, fast verify. **serde_json::Value for all extensible fields**—no schema migrations for collector-specific data. **SQLite tables mirror structs 1:1** with `json` columns for `metadata`, `params`, `details`. **Zero enterprise fields**—no tenant, org, RBAC, workflow IDs. Add later via wrapper tables if needed.