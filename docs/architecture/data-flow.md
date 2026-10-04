# Data Flow Architecture

**Status**: SPECIFIED

---

## End-to-End Data Flow

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

## Evidence Pipeline

### Collection → Receipt → Provenance

```
Collector.execute()
       │
       ▼
Raw Evidence Bytes + Metadata
       │
       ▼
blake3(raw_bytes) → payload_hash
       │
       ▼
Ed25519_sign(payload_hash || evidence_id || timestamp, collector_privkey) → signature
       │
       ▼
Build EvidenceReceipt {
    id, evidence_id, run_id,
    collector, collector_version,
    signature, public_key,
    payload_hash, payload_hash_algo,
    timestamp, host_info
}
       │
       ▼
Build Evidence {
    id, run_id, step_id, collector,
    type, name, path, size,
    hash, hash_algo, metadata,
    collected_at
}
       │
       ▼
Persist Evidence (filesystem) + Receipt (SQLite)
       │
       ▼
Create Root ProvenanceRecord {
    id, evidence_id, receipt_id,
    prev_provenance_id: null,
    action: "collected",
    actor: collector,
    actor_version,
    input_hashes: [],
    output_hashes: [payload_hash],
    parameters: {},
    timestamp,
    signature: Ed25519(actor_key, payload)
}
```

### Provenance Chain Extension

Each transformation appends to chain:

```
Previous ProvenanceRecord
       │
       ▼
Transformation (correlate, enrich, export)
       │
       ▼
New ProvenanceRecord {
    id: ULID,
    evidence_id: ULID,
    receipt_id: ULID,
    prev_provenance_id: previous.id,
    action: "transformed|correlated|exported",
    actor: "correlator|enricher|exporter",
    actor_version: "0.1.0",
    input_hashes: [prev.output_hashes...],
    output_hashes: [new_evidence_hashes...],
    parameters: { "rule": "...", "window": 300 },
    timestamp,
    signature: Ed25519(actor_key, canonical_json)
}
```

---

## Correlation Data Flow

```
Evidence[] (from store)
       │
       ▼
Load Correlation Rules (config)
       │
       ▼
For Each Rule:
    Filter source evidence by source_type
    Filter target evidence by target_type
    Apply condition (e.g., source.pid == target.ppid)
    For each match:
        Create CorrelationEdge {
            id: ULID,
            run_id,
            src_evidence_id,
            dst_evidence_id,
            relation: rule.relation,
            confidence: rule.confidence,
            method: rule.name,
            details: rule.params,
            created_at: now(),
            created_by: "correlator"
        }
       │
       ▼
Append CorrelationEdge to store
       │
       ▼
Create ProvenanceRecord for correlation:
    action: "correlated"
    actor: "correlator"
    input_hashes: [src.hash, dst.hash]
    output_hashes: [edge_hash]
```

---

## Investigation Graph Construction

```
CorrelationEdge[] (from store)
       │
       ▼
petgraph::Graph<EvidenceNode, CorrelationEdge, Directed>
       │
       ▼
For each Evidence:
    Add node: EvidenceNode {
        evidence_id,
        evidence_type,
        timestamp,
        summary: human_readable(evidence)
    }
       │
       ▼
For each CorrelationEdge:
    Add edge: src_node → dst_node with CorrelationEdge data
       │
       ▼
Apply Hypothesis Overlay:
    For each hypothesis:
        Mark supporting nodes (border: gold)
        Mark refuting nodes (border: red)
        Update hypothesis.confidence
```

### Graph Queries

| Query | Implementation |
|-------|----------------|
| `path(from, to, max_hops)` | `petgraph::algo::astar` or `all_simple_paths` |
| `backward_from(node, window)` | Reverse DFS with timestamp filter |
| `forward_from(node, window)` | DFS with timestamp filter |
| `hypothesis_support(id)` | Filter nodes by hypothesis.supporting_evidence_ids |
| `subgraph(evidence_ids)` | `graph.filter_map` with node filter |

---

## Verification Data Flow

```
Verification Request
       │
       ▼
Load All Evidence for Run
       │
       ▼
For Each Evidence:
    1. Load Receipt
    2. Verify Ed25519 signature
    3. Recompute blake3(evidence_file) == receipt.payload_hash
    4. Verify timestamp within skew
    5. Verify collector_version matches registry
       │
       ▼
Load Provenance Chain for Evidence
       │
       ▼
For Each ProvenanceRecord in Chain:
    1. Verify actor signature
    2. Verify prev_provenance_id links to previous record
    3. Verify input_hashes match previous output_hashes
    4. Verify output_hashes match actual evidence hashes
       │
       ▼
Aggregate Results:
    PASS: All checks pass
    FAIL: Any check fails with details
```

---

## Export Data Flow

```
Verified Investigation Graph
       │
       ├──────────────────┐
       ▼                  ▼
CASE/UCO Export      Cytoscape Export
       │                  │
       ▼                  ▼
JSON-LD with         JSON with
provenance refs      styles + metadata
       │                  │
       └────────┬───────┘
                ▼
         Timesketch CSV
                │
                ▼
         Markdown Report
                │
                ▼
         Filesystem + API Response
```

---

## Related Documents

- `overview.md` — Architecture overview
- `system-architecture.md` — Component architecture
- `execution-contract.md` — Execution Contract specification
- `collector-architecture.md` — Collector trait and implementations
- `correlation-architecture.md` — Correlation rules and graph
- `deployment-model.md` — Deployment topology and models
- `evidence-pipeline.md` — Evidence pipeline details