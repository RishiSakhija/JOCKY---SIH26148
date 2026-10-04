# JOCKY IR (Intermediate Representation)

**Status**: SPECIFIED

---

## Purpose

The **Forensic IR** is a platform-agnostic execution plan derived from the typed AST. It contains fully resolved collector/action/params with explicit dependencies for parallel execution.

---

## IR Module Structure

```json
{
  "version": 1,
  "id": "ULID",
  "metadata": {
    "name": "apt29-lateral-movement",
    "description": "Detect APT29 WMI lateral movement",
    "author": "analyst@org",
    "created_at": "2026-10-04T12:00:00Z",
    "tags": ["windows", "wmi", "lateral-movement", "apt29"]
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

---

## Field Specifications

### IRModule

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | u32 | ✅ | IR schema version (currently 1) |
| `id` | ULID | ✅ | Unique module identifier |
| `metadata` | IRMetadata | ✅ | Module metadata |
| `steps` | Vec<IRStep> | ✅ | Ordered execution steps |
| `outputs` | Vec<IROutput> | ❌ | Declared output bindings |

### IRMetadata

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Human-readable name |
| `description` | String | ❌ | Free text description |
| `author` | String | ❌ | Author identifier |
| `created_at` | DateTime<Utc> | ✅ | Creation timestamp |
| `tags` | Vec<String> | ❌ | Searchable tags |

### IRStep

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | ULID | ✅ | Unique step identifier (within module) |
| `collector` | String | ✅ | Collector plugin name (platform-specific) |
| `action` | String | ✅ | Collector action to invoke |
| `params` | Value | ✅ | Resolved action parameters |
| `binds` | Vec<String> | ❌ | Output variable names |
| `depends_on` | Vec<String> | ❌ | Step IDs this depends on |
| `condition` | Option<String> | ❌ | Optional guard expression |

### IROutput

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Output variable name |
| `from_step` | String | ✅ | Step ID producing this output |
| `field` | String | ❌ | Specific field to extract |

---

## Key Properties

1. **ULIDs Everywhere** — Monotonic, sortable, no coordination needed
2. **Explicit Dependencies** — `depends_on` enables topological sort for parallel execution
3. **Platform-Specific Collectors** — IR contains resolved collector names (e.g., `windows_evt` not generic `logs`)
4. **Fully Resolved Params** — No DSL references; all values concrete
5. **Output Bindings** — Declare data flow for downstream consumers

---

## IR Lowering (AST → IR)

### Collect → IRStep

| DSL | IR Collector | IR Action | Params |
|-----|--------------|-----------|--------|
| `collect process as x where name == "svchost"` | `proc_windows` / `proc_linux` | `list` | `{ "filter": { "name_contains": "svchost" } }` |
| `collect file as x where path == "C:\\x"` | `file_windows` / `file_linux` | `collect` | `{ "source": "C:\\x" }` |
| `collect registry as x where path == "HKLM\\Run"` | `registry_windows` | `get` | `{ "hive": "HKLM", "path": "Run" }` |
| `collect logs as x where EventID == 4625` | `logs_windows` / `logs_linux` | `query` | `{ "channel": "Security", "query": "..." }` |
| `collect network as x where port == 443` | `net_windows` / `net_linux` | `connections` | `{ "filter": { "remote_port": 443 } }` |

### Correlate → IRStep

| DSL | IR Collector | IR Action | Params |
|-----|--------------|-----------|--------|
| `correlate A -> B by "pid_link" as C` | `correlator` | `pid_link` | `{ "left": "A", "right": "B" }` |
| `correlate A -> B by "timestamp_join" window 300s as C` | `correlator` | `timestamp_join` | `{ "left": "A", "right": "B", "window_seconds": 300 }` |

### Timeline → IRStep

| DSL | IR Collector | IR Action | Params |
|-----|--------------|-----------|--------|
| `timeline "name" from A, B` | `correlator` | `build_timeline` | `{ "sources": ["A", "B"] }` |

---

## Dependency Resolution

### Topological Sort

```rust
fn topological_sort(steps: &[IRStep]) -> Result<Vec<&IRStep>, CycleError> {
    let mut graph = DiGraphMap::new();
    let mut node_map = HashMap::new();
    
    // Add nodes
    for step in steps {
        let idx = graph.add_node(step);
        node_map.insert(step.id.clone(), idx);
    }
    
    // Add edges
    for step in steps {
        let from = node_map[&step.id];
        for dep_id in &step.depends_on {
            let to = node_map[dep_id];
            graph.add_edge(to, from, ());  // dep → step
        }
    }
    
    // Topological sort
    toposort(&graph, None).map_err(|_| CycleError)
}
```

### Parallel Execution Groups

```rust
fn execution_groups(steps: &[IRStep], parallelism: usize) -> Vec<Vec<&IRStep>> {
    let sorted = topological_sort(steps)?;
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    let mut completed = HashSet::new();
    
    for step in sorted {
        if step.depends_on.iter().all(|d| completed.contains(d)) {
            current_group.push(step);
            if current_group.len() == parallelism {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
            groups.push(vec![step]);
        }
        completed.insert(step.id.clone());
    }
    
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    
    groups
}
```

---

## IR Validation

### Validation Rules

| Rule | Check |
|------|-------|
| **Unique Step IDs** | All `step.id` unique within module |
| **Valid Dependencies** | All `depends_on` reference existing step IDs |
| **No Cycles** | Dependency graph is a DAG |
| **Collector Exists** | Each `collector` registered in registry |
| **Action Supported** | Each `action` in collector's `supported_actions` |
| **Params Valid** | `params` matches collector's JSON Schema |
| **Output Bindings** | All `binds` unique; `outputs` reference valid steps |

---

## IR Versioning

| Version | Changes |
|---------|---------|
| 1 | Initial specification |

- Forward compatible within major version
- Additive fields only (optional)
- Breaking changes = major version bump

---

## Rust Types

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IrModule {
    pub version: u32,
    pub id: String,  // ULID
    pub metadata: IRMetadata,
    pub steps: Vec<IRStep>,
    pub outputs: Vec<IROutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IRMetadata {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IRStep {
    pub id: String,  // ULID
    pub collector: String,
    pub action: String,
    pub params: Value,
    pub binds: Vec<String>,
    pub depends_on: Vec<String>,
    pub condition: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IROutput {
    pub name: String,
    pub from_step: String,
    pub field: Option<String>,
}
```

---

## Related Documents

- `language-overview.md` — Language overview
- `grammar.md` — Pest grammar
- `ast.md` — AST node definitions
- `examples.md` — Example scripts
- `../architecture/system-architecture.md` — System architecture
- `../architecture/execution-contract.md` — Execution Contract