# JOCKY AST Specification

**Status**: SPECIFIED

---

## AST Module

```rust
pub struct AstModule {
    pub name: String,
    pub statements: Vec<Statement>,
    pub span: Span,  // Source location
}
```

---

## Statement Nodes

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
```

### HypothesisDecl

```rust
pub struct HypothesisDecl {
    pub description: String,
    pub mitre_tags: Vec<String>,
    pub span: Span,
}
```

### CollectDecl

```rust
pub struct CollectDecl {
    pub evidence_type: EvidenceType,
    pub binding: String,
    pub filter: Option<Expression>,
    pub span: Span,
}

pub enum EvidenceType {
    Process,
    File,
    Registry,
    Config,
    Logs,
    Network,
}
```

### FilterDecl

```rust
pub struct FilterDecl {
    pub source: String,  // Binding name
    pub filter: Expression,
    pub span: Span,
}
```

### MatchDecl

```rust
pub struct MatchDecl {
    pub source: String,  // Binding name
    pub pattern: Expression,
    pub span: Span,
}
```

### CorrelateDecl

```rust
pub struct CorrelateDecl {
    pub source: String,      // Source binding
    pub target: String,      // Target binding
    pub relation: String,    // Correlation method name
    pub binding: String,     // Output binding name
    pub params: CorrelationParams,
    pub span: Span,
}

pub struct CorrelationParams {
    pub window: Option<Duration>,  // For timestamp_join
}
```

### TimelineDecl

```rust
pub struct TimelineDecl {
    pub name: String,
    pub sources: Vec<String>,  // Binding names
    pub span: Span,
}
```

### BindDecl

```rust
pub struct BindDecl {
    pub hypothesis: String,     // Hypothesis description
    pub evidence_bindings: Vec<String>,  // Binding names
    pub span: Span,
}
```

### ExportDecl

```rust
pub struct ExportDecl {
    pub output_type: OutputType,
    pub name: String,
    pub format: ExportFormat,
    pub span: Span,
}

pub enum OutputType {
    Case,
    Graph,
    Timeline,
    Report,
}

pub enum ExportFormat {
    CaseUco,
    Cytoscape,
    Timesketch,
    Markdown,
    Graphml,
    Dot,
}
```

### VerifyDecl

```rust
pub struct VerifyDecl {
    pub target: String,  // Binding name or "all"
    pub span: Span,
}
```

---

## Expression Nodes

```rust
pub enum Expression {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
    Identifier(String),
}

pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: BinaryOp,
    pub right: Box<Expression>,
    pub span: Span,
}

pub enum BinaryOp {
    Eq,      // ==
    Neq,     // !=
    Gt,      // >
    Lt,      // <
    Gte,     // >=
    Lte,     // <=
    Contains,
    Matches, // regex
    In,      // array membership
    And,
    Or,
}

pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub operand: Box<Expression>,
    pub span: Span,
}

pub enum UnaryOp {
    Not,
    Neg,
}

pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Literal>),
}
```

---

## Typed AST (Post Type Checking)

```rust
pub struct TypedAst {
    pub module: AstModule,
    pub capabilities: Vec<Capability>,
    pub symbol_table: SymbolTable,
}

pub struct SymbolTable {
    bindings: HashMap<String, BindingInfo>,
    hypotheses: HashMap<String, HypothesisInfo>,
}

pub struct BindingInfo {
    pub name: String,
    pub evidence_type: EvidenceType,
    pub inferred_capabilities: Vec<Capability>,
}

pub struct HypothesisInfo {
    pub description: String,
    pub mitre_tags: Vec<String>,
    pub bound_evidence: Vec<String>,
}
```

---

## Capability Inference Rules

| AST Pattern | Inferred Capabilities |
|-------------|----------------------|
| `CollectDecl { evidence_type: Process, filter: None }` | `[ProcessEnumerate]` |
| `CollectDecl { evidence_type: Process, filter: Some(pid_eq) }` | `[ProcessEnumerate, ProcessGet]` |
| `CollectDecl { evidence_type: File, filter: None }` | `[FileEnumerate]` |
| `CollectDecl { evidence_type: File, filter: Some(path_eq) }` | `[FileEnumerate, FileHash, FileCollect]` |
| `CollectDecl { evidence_type: Registry, .. }` | `[RegistryEnumerate]` |
| `CollectDecl { evidence_type: Config, .. }` | `[ConfigEnumerate]` |
| `CollectDecl { evidence_type: Logs, .. }` | `[LogQuery]` |
| `CollectDecl { evidence_type: Network, .. }` | `[NetworkConnections]` |

---

## Type Checking Passes

1. **Scope Resolution** — All bindings referenced must be declared
2. **Type Compatibility** — Filter/match source must be collect binding
3. **Correlation Validity** — Source/target bindings must exist and be compatible
4. **Capability Inference** — Collect declarations → capability set
5. **Export Validity** — Export sources must exist
6. **Hypothesis Binding** — Bound evidence must exist

---

## Error Reporting

```rust
pub struct TypeError {
    pub message: String,
    pub span: Span,
    pub code: TypeErrorCode,
}

pub enum TypeErrorCode {
    UndefinedBinding,
    TypeMismatch,
    InvalidCorrelation,
    MissingCapability,
    CircularDependency,
    DuplicateBinding,
}
```

---

## Related Documents

- `language-overview.md` — Language overview
- `grammar.md` — Pest grammar
- `ir.md` — IR specification
- `examples.md` — Example scripts