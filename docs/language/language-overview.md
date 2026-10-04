# JOCKY Language Overview

**Status**: SPECIFIED

---

## Design Philosophy

JOCKY is a **small, declarative, forensic DSL** — not a general-purpose programming language.

| Principle | Implementation |
|-----------|----------------|
| **Declarative** | Describe *what* to investigate, not *how* |
| **Hypothesis-Driven** | Explicit hypothesis declaration with MITRE mapping |
| **Evidence-First** | Every `collect` produces typed, provenance-tracked evidence |
| **Deterministic** | No loops, no arbitrary code, no side effects |
| **Cross-Platform** | Same script compiles to Windows/Linux collectors |
| **Composable** | Pipeline: collect → filter → correlate → timeline → export |

---

## Language Grammar (Pest)

Full grammar: `docs/language/grammar.md`

```pest
investigation = { "investigation" ~ identifier ~ "{" ~ statement* ~ "}" }
statement = { hypothesis | collect | filter | match | correlate | timeline | bind | export | verify }
hypothesis = { "hypothesis" ~ string ~ "{" ~ "mitre:" ~ "[" ~ (string ~ ("," ~ string)*)? ~ "]" ~ "}" }
collect = { "collect" ~ evidence_type ~ "as" ~ identifier ~ where_clause? }
where_clause = { "where" ~ expression }
filter = { "filter" ~ identifier ~ where_clause }
match = { "match" ~ identifier ~ where_clause }
correlate = { "correlate" ~ identifier ~ "->" ~ identifier ~ "by" ~ string ~ "as" ~ identifier ~ correlate_params? }
correlate_params = { "window" ~ integer ~ "s" }
timeline = { "timeline" ~ identifier ~ "from" ~ identifier ~ ("," ~ identifier)* }
bind = { "bind" ~ "hypothesis" ~ string ~ "to" ~ identifier ~ ("," ~ identifier)* }
export = { "export" ~ output_type ~ identifier ~ "format" ~ format_spec }
verify = { "verify" ~ identifier }
```

---

## Locked Commands (10 Statements)

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

---

## Evidence Types (for `collect`)

| Type | Description | Windows Collector | Linux Collector |
|------|-------------|-------------------|-----------------|
| `process` | Running processes | `proc_windows` | `proc_linux` |
| `file` | File system artifacts | `file_windows` | `file_linux` |
| `registry` | Windows Registry | `registry_windows` | — |
| `config` | Linux configuration | — | `config_linux` |
| `logs` | Event logs | `logs_windows` | `logs_linux` |
| `network` | Network connections | `net_windows` | `net_linux` |

---

## Example Script

```jocky
investigation "apt29-lateral-movement" {
    // Declare hypothesis with MITRE ATT&CK mapping
    hypothesis "APT29 used WMI for lateral movement" {
        mitre: ["T1047", "T1547.001"]
    }

    // Collection (platform-agnostic)
    collect process as wmi_providers where command_line contains "wmic"
    collect process as spawns where parent_name == "wmiprvse.exe"
    collect registry as run_keys where path matches "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run*"
    collect network as outbound where destination_port in [443, 80, 135, 445]

    // Filtering
    filter wmi_providers where cpu_time > 1000
    match spawns where command_line contains "powershell"

    // Correlation (deterministic, evidence-cited)
    correlate wmi_providers -> spawns by "pid_link" as wmi_chain
    correlate wmi_chain -> run_keys by "timestamp_join" window 300s as persistence
    correlate persistence -> outbound by "process_netlink" as c2_beacon

    // Timeline & Hypothesis Binding
    timeline "lateral_movement" from wmi_chain, persistence, c2_beacon
    bind hypothesis "APT29 used WMI for lateral movement" to wmi_chain, persistence

    // Output
    export case "apt29-case" format case_uco
    export graph "apt29-graph" format cytoscape
    export report "apt29-report" format markdown
}
```

---

## Execution Flow

```
JOCKY Script
       │
       ▼
┌──────────────────┐
│  pest Parser     │ ◀── Grammar
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
... (evidence pipeline, correlation, graph, verification, export)
```

---

## Capability Inference

The type checker infers required capabilities from `collect` statements:

| DSL Pattern | Inferred Capabilities |
|-------------|----------------------|
| `collect process as x where ...` | `ProcessEnumerate` |
| `collect process as x where pid == 123` | `ProcessEnumerate`, `ProcessGet` |
| `collect file as x where ...` | `FileEnumerate` |
| `collect file as x where path == "..."` | `FileEnumerate`, `FileHash`, `FileCollect` |
| `collect registry as x where ...` | `RegistryEnumerate` |
| `collect logs as x where ...` | `LogQuery` |
| `collect network as x where ...` | `NetworkConnections` |

These capabilities are validated against collector manifests in the Execution Contract.

---

## Expression Language

Used in `where` clauses and `match` conditions:

```pest
expression = { comparison ~ (("and" | "or") ~ comparison)* }
comparison = { identifier ~ operator ~ value }
operator = { "==" | "!=" | ">" | "<" | ">=" | "<=" | "contains" | "matches" | "in" }
value = { string | integer | float | boolean | "[" ~ value ~ ("," ~ value)* ~ "]" }
identifier = { ALPHA ~ (ALPHA | DIGIT | "_")* }
```

### Supported Operators

| Operator | Types | Example |
|----------|-------|---------|
| `==`, `!=` | All | `name == "svchost.exe"` |
| `>`, `<`, `>=`, `<=` | Numeric | `cpu > 50` |
| `contains` | String | `command_line contains "powershell"` |
| `matches` | String (regex) | `path matches "HKLM\\.*Run.*"` |
| `in` | Array | `destination_port in [443, 80]` |

---

## Output Formats (for `export`)

| Format | Description | Output |
|--------|-------------|--------|
| `case_uco` | CASE/UCO JSON-LD | `.case.json` |
| `cytoscape` | Cytoscape.js JSON | `.graph.json` |
| `timesketch` | Timesketch CSV | `.timeline.csv` |
| `markdown` | Narrative report | `.report.md` |
| `graphml` | GraphML for Gephi | `.graphml` |
| `dot` | GraphViz DOT | `.dot` |

---

## Related Documents

- `grammar.md` — Complete Pest grammar
- `ast.md` — AST node definitions
- `ir.md` — IR specification
- `examples.md` — Example scripts