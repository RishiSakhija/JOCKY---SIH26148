# Evidence Pipeline Specification

**Status**: SPECIFIED

---

## Overview

The evidence pipeline transforms raw collector output into tamper-evident, provenance-tracked evidence objects.

---

## Pipeline Stages

```
Collector.execute()
       │
       ▼
Raw Evidence Bytes + Metadata
       │
       ▼
[HASH] blake3(raw_bytes) → payload_hash
       │
       ▼
[SIGN] Ed25519_sign(payload_hash || evidence_id || timestamp, collector_privkey) → signature
       │
       ▼
[BUILD] EvidenceReceipt
       │
       ▼
[BUILD] Evidence Object
       │
       ▼
[PERSIST] Write Evidence File + Receipt (SQLite)
       │
       ▼
[PROVENANCE] Create Root ProvenanceRecord
```

---

## Evidence Object

### Schema

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
  "hash": "a1b2c3d4e5f6...",
  "hash_algo": "blake3",
  "metadata": {
    "event_count": 47,
    "earliest": "2026-10-04T10:00:00Z",
    "latest": "2026-10-04T11:55:00Z"
  },
  "collected_at": "2026-10-04T12:00:05Z"
}
```

### Field Specifications

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | ULID | ✅ | Unique evidence identifier |
| `run_id` | ULID | ✅ | Parent investigation run |
| `step_id` | ULID | ✅ | Producing step identifier |
| `collector` | String | ✅ | Source collector name |
| `type` | String | ✅ | MIME-like type (see below) |
| `name` | String | ✅ | Human-readable name |
| `path` | String | ✅ | Relative path in evidence_root |
| `size` | u64 | ✅ | File size in bytes |
| `hash` | String | ✅ | Hex-encoded hash |
| `hash_algo` | String | ✅ | `blake3` or `sha256` |
| `metadata` | Value | ❌ | Collector-specific metadata |
| `collected_at` | DateTime<Utc> | ✅ | Collection timestamp |

### Evidence Types (MIME-like)

| Category | Types |
|----------|-------|
| **Process** | `artifact/process`, `artifact/process_tree` |
| **File** | `artifact/file`, `artifact/ntfs`, `artifact/mft` |
| **Registry/Config** | `artifact/reg`, `artifact/config` |
| **LogEvent** | `artifact/evt`, `artifact/journald`, `artifact/syslog`, `artifact/audit` |
| **NetworkConnection** | `artifact/connection`, `artifact/netflow`, `artifact/pcap` |
| **Indicator** | `indicator/ioc`, `indicator/yara`, `indicator/sigma` |
| **Finding** | `finding/detection`, `finding/hypothesis` |

---

## Evidence Receipt

### Schema

```json
{
  "id": "ULID",
  "evidence_id": "ULID",
  "run_id": "ULID",
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

### Field Specifications

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | ULID | ✅ | Unique receipt identifier |
| `evidence_id` | ULID | ✅ | References Evidence.id |
| `run_id` | ULID | ✅ | Parent investigation run |
| `collector` | String | ✅ | Collector identity |
| `collector_version` | String | ✅ | Collector semantic version |
| `signature` | String | ✅ | Ed25519 signature (base64) |
| `public_key` | String | ✅ | Collector public key (hex) |
| `payload_hash` | String | ✅ | Hash of evidence payload |
| `payload_hash_algo` | String | ✅ | Hash algorithm used |
| `timestamp` | DateTime<Utc> | ✅ | Receipt creation time |
| `host_info` | HostInfo | ✅ | Collection host details |

### HostInfo

```json
{
  "hostname": "WORKSTATION-01",
  "os": "Windows 10 22H2",
  "kernel": "10.0.19045",
  "collector_pid": 4216
}
```

### Signature Construction

```
signed_data = payload_hash || evidence_id || timestamp (canonical JSON)
signature = Ed25519_sign(signed_data, collector_private_key)
```

### Verification

```rust
fn verify_receipt(receipt: &EvidenceReceipt, evidence: &Evidence) -> Result<(), VerifyError> {
    // 1. Verify signature
    let signed_data = format!("{}{}{}", receipt.payload_hash, receipt.evidence_id, receipt.timestamp);
    ed25519_verify(&receipt.public_key, signed_data, &receipt.signature)?;
    
    // 2. Verify payload hash
    let computed_hash = blake3(evidence_bytes);
    if computed_hash != receipt.payload_hash {
        return Err(VerifyError::HashMismatch);
    }
    
    // 3. Verify timestamp (within 5 min skew)
    let skew = (Utc::now() - receipt.timestamp).abs();
    if skew > Duration::minutes(5) {
        return Err(VerifyError::TimestampSkew);
    }
    
    // 4. Verify collector version
    if receipt.collector_version != registry.get_version(&receipt.collector) {
        return Err(VerifyError::VersionMismatch);
    }
    
    Ok(())
}
```

---

## Provenance Record

### Schema

```json
{
  "id": "ULID",
  "evidence_id": "ULID",
  "receipt_id": "ULID",
  "prev_provenance_id": "ULID|null",
  "action": "collected|transformed|correlated|exported",
  "actor": "windows_evt|correlator|enricher|exporter",
  "actor_version": "0.1.0",
  "input_hashes": ["hash1", "hash2"],
  "output_hashes": ["hash3"],
  "parameters": { "rule": "timestamp_join", "window": 300 },
  "timestamp": "2026-10-04T12:00:05Z",
  "signature": "Ed25519(actor_key, payload)"
}
```

### Field Specifications

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | ULID | ✅ | Unique provenance identifier |
| `evidence_id` | ULID | ✅ | Evidence this provenance tracks |
| `receipt_id` | ULID | ✅ | EvidenceReceipt identifier |
| `prev_provenance_id` | ULID | ❌ | Previous record in chain (null for root) |
| `action` | String | ✅ | `collected`, `transformed`, `correlated`, `exported` |
| `actor` | String | ✅ | Tool/agent identifier |
| `actor_version` | String | ✅ | Tool version |
| `input_hashes` | Vec<String> | ❌ | Input evidence hashes |
| `output_hashes` | Vec<String> | ❌ | Output evidence hashes |
| `parameters` | Value | ❌ | Transformation parameters |
| `timestamp` | DateTime<Utc> | ✅ | Action timestamp |
| `signature` | String | ✅ | Actor signature |

### Chain Properties

- `prev_provenance_id` creates hash-linked chain
- Each transformation produces new provenance record
- Actor signs with its identity key
- Immutable append-only log per evidence object

### Actions

| Action | Actor | Description |
|--------|-------|-------------|
| `collected` | Collector | Initial evidence acquisition |
| `transformed` | Correlator/Enricher | Format conversion, enrichment |
| `correlated` | Correlator | Joining evidence via rules |
| `exported` | Exporter | CASE/UCO, Cytoscape, Timesketch output |

### Verification

```rust
fn verify_provenance_chain(chain: &[ProvenanceRecord]) -> Result<(), VerifyError> {
    let mut prev_output_hashes = Vec::new();
    
    for (i, record) in chain.iter().enumerate() {
        // 1. Verify actor signature
        let payload = canonical_json(&record);
        ed25519_verify(&actor_public_key(&record.actor), payload, &record.signature)?;
        
        // 2. Verify chain link
        if i > 0 {
            if record.prev_provenance_id != chain[i-1].id {
                return Err(VerifyError::ChainBroken);
            }
            if record.input_hashes != prev_output_hashes {
                return Err(VerifyError::HashChainMismatch);
            }
        } else {
            // Root record: input_hashes should be empty
            if !record.input_hashes.is_empty() {
                return Err(VerifyError::RootHasInputs);
            }
        }
        
        // 3. Verify output hashes match actual evidence
        for hash in &record.output_hashes {
            if !evidence_store.has_hash(hash) {
                return Err(VerifyError::OutputHashMissing);
            }
        }
        
        prev_output_hashes = record.output_hashes.clone();
    }
    
    Ok(())
}
```

---

## Evidence Storage

### Filesystem Layout

```
evidence_root/
└── {run_id}/
    ├── evidence/
    │   ├── {evidence_id_1}.jsonl
    │   ├── {evidence_id_2}.jsonl
    │   └── ...
    ├── receipts/
    │   └── (in SQLite)
    └── provenance/
        └── (in SQLite)
```

### SQLite Tables

See `JOCKY_SPEC_SHEET.md` Section 17 for complete schema.

Key tables:
- `evidence` — Evidence objects
- `receipts` — Evidence receipts (1:1 with evidence)
- `provenance` — Provenance chain records

---

## Collector Output Contract

```rust
struct CollectorOutput {
    evidence: Vec<Evidence>,      // Primary evidence objects
    artifacts: Vec<ArtifactRef>,  // Side artifacts (e.g., raw EVTX)
    metrics: CollectorMetrics,    // Duration, bytes, errors
}

struct ArtifactRef {
    path: String,                 // Relative to evidence_root
    type: String,                 // MIME-like type
    description: String,
}

struct CollectorMetrics {
    duration_ms: u64,
    bytes_read: u64,
    bytes_written: u64,
    errors: Vec<String>,
}
```

---

## Related Documents

- `overview.md` — Architecture overview
- `system-architecture.md` — Component architecture
- `data-flow.md` — Evidence pipeline and data flow
- `execution-contract.md` — Execution Contract specification
- `collector-architecture.md` — Collector trait and implementations
- `correlation-architecture.md` — Correlation rules and graph
- `deployment-model.md` — Deployment topology and models