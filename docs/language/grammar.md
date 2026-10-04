# JOCKY Grammar (Pest)

**Status**: SPECIFIED

---

## Complete Pest Grammar

```pest
// ============================================================================
// JOCKY Grammar - Pest PEG Parser
// ============================================================================

// --- Top Level ---
investigation = { "investigation" ~ identifier ~ "{" ~ statement* ~ "}" }
statement = { hypothesis | collect | filter | match | correlate | timeline | bind | export | verify }

// --- Hypothesis ---
hypothesis = { "hypothesis" ~ string ~ "{" ~ "mitre:" ~ "[" ~ mitre_list? ~ "]" ~ "}" }
mitre_list = { string ~ ("," ~ string)* }

// --- Collection ---
collect = { "collect" ~ evidence_type ~ "as" ~ identifier ~ where_clause? }
evidence_type = { "process" | "file" | "registry" | "config" | "logs" | "network" }
where_clause = { "where" ~ expression }

// --- Filtering ---
filter = { "filter" ~ identifier ~ where_clause }
match = { "match" ~ identifier ~ where_clause }

// --- Correlation ---
correlate = { "correlate" ~ identifier ~ "->" ~ identifier ~ "by" ~ string ~ "as" ~ identifier ~ correlate_params? }
correlate_params = { "window" ~ integer ~ ("s" | "m" | "h") }

// --- Timeline ---
timeline = { "timeline" ~ identifier ~ "from" ~ identifier ~ ("," ~ identifier)* }

// --- Hypothesis Binding ---
bind = { "bind" ~ "hypothesis" ~ string ~ "to" ~ identifier ~ ("," ~ identifier)* }

// --- Export ---
export = { "export" ~ output_type ~ identifier ~ "format" ~ format_spec }
output_type = { "case" | "graph" | "timeline" | "report" }
format_spec = { "case_uco" | "cytoscape" | "timesketch" | "markdown" | "graphml" | "dot" }

// --- Verification ---
verify = { "verify" ~ identifier }

// --- Expressions ---
expression = { comparison ~ (("and" | "or") ~ comparison)* }
comparison = { identifier ~ operator ~ value }
operator = { "==" | "!=" | ">" | "<" | ">=" | "<=" | "contains" | "matches" | "in" }
value = { string | integer | float | boolean | array }
array = { "[" ~ value ~ ("," ~ value)* ~ "]" }

// --- Lexical ---
identifier = { ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
string = { "\"" ~ (CHAR - "\"")* ~ "\"" }
integer = { ["-"]? ~ ASCII_DIGIT+ }
float = { ["-"]? ~ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
boolean = { "true" | "false" }

// --- Whitespace & Comments ---
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* ~ ("\n" | EOF) }

// --- Rules for Pest ---
// These are silent rules (prefixed with _)
// They're used for tokenization but don't appear in the CST
```

---

## Grammar Notes

### Precedence
- `and` binds tighter than `or`
- Comparison operators have equal precedence
- Parentheses not supported in MVP (add if needed)

### String Escaping
- Standard JSON escaping in strings: `\"`, `\\`, `\n`, `\t`, `\r`, `\uXXXX`

### Identifiers
- Must start with letter
- Can contain letters, digits, underscore
- Case-sensitive

### Evidence Types
- Fixed set: `process`, `file`, `registry`, `config`, `logs`, `network`
- Maps to platform-specific collectors at compile time

### Correlation Methods
- String identifier matching registered correlation rules
- Current: `pid_link`, `file_write`, `file_read`, `net_link`, `ioc_match`, `yara_match`, `sigma_match`, `rule_match`, `timestamp_join`

### Time Units
- `s` = seconds
- `m` = minutes  
- `h` = hours
- Default: seconds if omitted

---

## Example Parse Tree

For:
```jocky
investigation "test" {
    collect process as procs where name == "svchost.exe"
    correlate procs -> procs by "pid_link" as chain
}
```

CST Structure:
```
investigation
├── identifier: "test"
└── statement (x2)
    ├── collect
    │   ├── evidence_type: "process"
    │   ├── identifier: "procs"
    │   └── where_clause
    │       └── expression
    │           └── comparison
    │               ├── identifier: "name"
    │               ├── operator: "=="
    │               └── value: "svchost.exe"
    └── correlate
        ├── identifier: "procs"
        ├── identifier: "procs"
        ├── string: "pid_link"
        └── identifier: "chain"
```

---

## Pest Integration (Rust)

```rust
// build.rs
fn main() {
    pest_generator::generate_grammar(
        std::path::Path::new("src/parser.pest")
    ).unwrap();
}

// src/lib.rs
#[derive(pest_derive::Parser)]
#[grammar = "parser.pest"]
pub struct JockeyParser;
```

---

## Related Documents

- `language-overview.md` — Language overview
- `ast.md` — AST node definitions
- `ir.md` — IR specification
- `examples.md` — Example scripts