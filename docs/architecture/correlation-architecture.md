# Correlation Architecture

**Status**: SPECIFIED

---

## Overview

**DETERMINISTIC EVIDENCE CORRELATION** — Not ML, not heuristic scoring. Every edge is derived from observable keys with explicit evidence citations.

---

## Core Correlation Model

### Correlation Edge

```json
{
  "id": "ULID",
  "run_id": "ULID",
  "src_evidence_id": "ULID",
  "dst_evidence_id": "ULID",
  "relation": "spawned|wrote|read|connected|resolved_to|contains|indicates|supported_by|timeline_adjacent",
  "confidence": 0.95,
  "method": "pid_link|file_write|net_link|ioc_match|timestamp_join|custom",
  "details": { "window_seconds": 300 },
  "created_at": "2026-10-04T12:00:10Z",
  "created_by": "correlator"
}
```

### Field Specifications

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | ULID | ✅ | Unique correlation identifier |
| `run_id` | ULID | ✅ | Parent investigation run |
| `src_evidence_id` | ULID | ✅ | Source evidence |
| `dst_evidence_id` | ULID | ✅ | Destination evidence |
| `relation` | String | ✅ | Relation type (see below) |
| `confidence` | f32 | ✅ | 0.0 - 1.0 |
| `method` | String | ✅ | Correlation method name |
| `details` | Value | ❌ | Method-specific details |
| `created_at` | DateTime<Utc> | ✅ | Creation timestamp |
| `created_by` | String | ✅ | Engine identifier |

---

## Core Relations

| Relation | Source Type | Target Type | Method | Confidence | Observable Keys |
|----------|-------------|-------------|--------|------------|-----------------|
| `spawned` | Process | Process | `pid_link` | 1.0 | `pid`, `ppid` |
| `wrote` | Process | File | `file_write` | 0.95 | `pid`, `writer_pid`, `timestamp` |
| `read` | Process | File | `file_read` | 0.9 | `pid`, `reader_pid`, `timestamp` |
| `connected` | Process | NetworkConnection | `net_link` | 0.95 | `pid`, `4-tuple` |
| `resolved_to` | NetworkConnection | Indicator | `ioc_match` | 1.0 | `remote_ip`, `remote_domain`, `ioc_value` |
| `contains` | File | Indicator | `yara_match`/`sigma_match` | 0.9 | `file_hash`, `file_path`, `ioc_pattern` |
| `indicates` | Evidence[] | Finding | `rule_match` | 0.8-1.0 | Rule + evidence IDs |
| `supported_by` | Finding | Evidence[] | `citation` | 1.0 | Explicit evidence IDs |
| `timeline_adjacent` | Evidence | Evidence | `timestamp_join` | 0.7 | `timestamp`, `window` |

---

## Correlation Rules (Configuration)

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
      "name": "file_write",
      "relation": "wrote",
      "source_type": "artifact/process",
      "target_type": "artifact/file",
      "condition": "source.pid == target.writer_pid && abs(source.timestamp - target.timestamp) < 5",
      "confidence": 0.95
    },
    {
      "name": "file_read",
      "relation": "read",
      "source_type": "artifact/process",
      "target_type": "artifact/file",
      "condition": "source.pid == target.reader_pid && abs(source.timestamp - target.timestamp) < 5",
      "confidence": 0.9
    },
    {
      "name": "net_link",
      "relation": "connected",
      "source_type": "artifact/process",
      "target_type": "artifact/connection",
      "condition": "source.pid == target.pid",
      "confidence": 0.95
    },
    {
      "name": "ioc_match",
      "relation": "resolved_to",
      "source_type": "artifact/connection",
      "target_type": "indicator/ioc",
      "condition": "target.value in [source.remote_ip, source.remote_domain]",
      "confidence": 1.0
    },
    {
      "name": "yara_match",
      "relation": "contains",
      "source_type": "artifact/file",
      "target_type": "indicator/yara",
      "condition": "yara_scan(source.path, target.rule)",
      "confidence": 0.9
    },
    {
      "name": "sigma_match",
      "relation": "contains",
      "source_type": "artifact/evt",
      "target_type": "indicator/sigma",
      "condition": "sigma_match(source.parsed, target.rule)",
      "confidence": 0.9
    },
    {
      "name": "rule_match",
      "relation": "indicates",
      "source_type": "*",
      "target_type": "finding/detection",
      "condition": "detection_rule.match(source_evidence)",
      "confidence": 0.8
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

## Correlation Engine

### Algorithm

```rust
pub fn correlate(evidence: &[Evidence], rules: &[CorrelationRule]) -> Vec<CorrelationEdge> {
    let mut edges = Vec::new();
    
    // Index evidence by type for efficient lookup
    let by_type: HashMap<String, Vec<&Evidence>> = evidence.iter()
        .fold(HashMap::new(), |mut acc, ev| {
            acc.entry(ev.type.clone()).or_default().push(ev);
            acc
        });
    
    for rule in rules {
        let sources = by_type.get(&rule.source_type).unwrap_or(&vec![]);
        let targets = if rule.target_type == "*" {
            evidence.iter().collect()
        } else {
            by_type.get(&rule.target_type).unwrap_or(&vec![]).iter().collect()
        };
        
        for src in sources {
            for dst in targets {
                if rule.condition.evaluate(src, dst) {
                    edges.push(CorrelationEdge {
                        id: Ulid::new(),
                        run_id: src.run_id.clone(),
                        src_evidence_id: src.id.clone(),
                        dst_evidence_id: dst.id.clone(),
                        relation: rule.relation.clone(),
                        confidence: rule.confidence,
                        method: rule.name.clone(),
                        details: rule.params.clone(),
                        created_at: Utc::now(),
                        created_by: "correlator".to_string(),
                    });
                }
            }
        }
    }
    
    edges
}
```

### Condition Evaluation

Conditions are evaluated as expressions against evidence metadata:

| Condition Type | Example | Implementation |
|----------------|---------|----------------|
| Field equality | `source.pid == target.ppid` | Direct field access |
| Temporal | `abs(source.timestamp - target.timestamp) < 5` | `chrono` duration |
| Set membership | `target.value in [source.remote_ip, source.remote_domain]` | HashSet lookup |
| Function call | `yara_scan(source.path, target.rule)` | External function registry |
| Regex | `source.path matches target.pattern` | `regex` crate |

---

## Investigation Graph

### Data Structure

```rust
use petgraph::graph::Graph;
use petgraph::Direction;

type InvestigationGraph = Graph<EvidenceNode, CorrelationEdge, Directed>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceNode {
    pub evidence_id: String,
    pub evidence_type: String,
    pub timestamp: DateTime<Utc>,
    pub summary: String,
    pub metadata: Value,
}
```

### Node Types & Visual Encoding

| Evidence Type | Shape | Color | Summary Example |
|---------------|-------|-------|-----------------|
| `artifact/process` | ellipse | `#3b82f6` | "Process: powershell.exe (PID 200)" |
| `artifact/file` | rectangle | `#10b981` | "File: C:\\Temp\\script.ps1" |
| `artifact/reg` / `artifact/config` | hexagon | `#f59e0b` | "Registry: HKLM\\Run\\Malware" |
| `artifact/evt` / `artifact/journald` | diamond | `#8b5cf6` | "Event: 4625 Logon Failure" |
| `artifact/connection` | triangle | `#ef4444` | "TCP: 192.168.1.50:49234 → 1.1.1.1:443" |
| `indicator/*` | star | `#ec4899` | "IOC: domain=evil.com" |
| `finding/*` | pentagon | `#f59e0b` | "Finding: T1059.001 PowerShell" |

### Edge Visual Encoding

| Relation | Color | Width (confidence) | Arrow |
|----------|-------|-------------------|-------|
| `spawned` | `#3b82f6` | 1.0 = 5px | ▸ |
| `wrote` | `#10b981` | 0.95 = 4.75px | ▸ |
| `read` | `#10b981` | 0.9 = 4.5px | ▸ |
| `connected` | `#ef4444` | 0.95 = 4.75px | ▸ |
| `resolved_to` | `#ec4899` | 1.0 = 5px | ▸ |
| `contains` | `#8b5cf6` | 0.9 = 4.5px | ▸ |
| `indicates` | `#f59e0b` | 0.8-1.0 | ▸ |
| `supported_by` | `#f59e0b` | 1.0 = 5px | ◂ (reverse) |
| `timeline_adjacent` | `#64748b` | 0.7 = 3.5px | — (undirected) |

---

## Graph Queries

```rust
impl InvestigationGraph {
    // Causal chain between two evidence nodes
    pub fn causal_path(&self, from: NodeIndex, to: NodeIndex, max_hops: usize) -> Vec<Path> {
        // Use petgraph::algo::all_simple_paths with hop limit
    }

    // All precursors within time window
    pub fn backward_from(&self, node: NodeIndex, window: Duration) -> Vec<EvidenceNode> {
        // Reverse DFS with timestamp filter
    }

    // All consequences within time window
    pub fn forward_from(&self, node: NodeIndex, window: Duration) -> Vec<EvidenceNode> {
        // DFS with timestamp filter
    }

    // Evidence supporting a hypothesis
    pub fn hypothesis_support(&self, hypothesis_id: &str) -> Vec<EvidenceNode> {
        // Filter nodes by hypothesis.supporting_evidence_ids
    }

    // Subgraph for export (e.g., Cytoscape JSON)
    pub fn induced_subgraph(&self, evidence_ids: &[String]) -> InvestigationGraph {
        // graph.filter_map with node filter
    }

    // Time-travel: graph state at specific timestamp
    pub fn graph_at(&self, timestamp: DateTime<Utc>) -> InvestigationGraph {
        // Filter edges/nodes by timestamp
    }
}
```

---

## Hypothesis Overlay

Hypotheses are a **separate layer** on the graph (not modifying evidence):

```rust
struct Hypothesis {
    id: String,
    description: String,
    mitre_tags: Vec<String>,
    confidence: f32,           // Analyst-assigned / auto-updated
    status: HypothesisStatus,  // Open, Confirmed, Refuted
    supporting_evidence_ids: Vec<String>,
    refuting_evidence_ids: Vec<String>,
}
```

**Visual**: Gold dashed border on nodes supporting hypothesis; red dashed on refuting.

### Hypothesis Confidence Update

```rust
fn update_hypothesis_confidence(hypothesis: &mut Hypothesis, graph: &InvestigationGraph) {
    let supporting = hypothesis.supporting_evidence_ids.iter()
        .filter_map(|id| graph.find_node(id))
        .count();
    let refuting = hypothesis.refuting_evidence_ids.iter()
        .filter_map(|id| graph.find_node(id))
        .count();
    
    // Simple heuristic: more support = higher confidence
    hypothesis.confidence = (supporting as f32) / ((supporting + refuting) as f32).max(1.0);
}
```

---

## Export Formats

| Format | Use Case | Content |
|--------|----------|---------|
| **Cytoscape.js JSON** | Frontend graph view | Nodes + edges + styles |
| **CASE/UCO JSON-LD** | Interop, archival | Full semantic graph |
| **Timesketch CSV** | Timeline analysis | Flattened events |
| **GraphML** | External tools (Gephi, etc.) | Standard graph format |
| **DOT/GraphViz** | Static diagrams | Renderable graphs |

---

## Related Documents

- `overview.md` — Architecture overview
- `system-architecture.md` — Component architecture
- `data-flow.md` — Evidence pipeline and data flow
- `execution-contract.md` — Execution Contract specification
- `collector-architecture.md` — Collector trait and implementations
- `deployment-model.md` — Deployment topology and models
- `evidence-pipeline.md` — Evidence pipeline details