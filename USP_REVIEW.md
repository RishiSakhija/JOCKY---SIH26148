# USP Review

## Current USP Verdict
**Weak.** JOCKY proposes a *language* + *runtime* for evidence-first forensics. The individual components (collectors, timelines, provenance, graphs, dashboards) all exist in mature tools. The "language" abstraction is the only novel element—but languages for DFIR (Velociraptor VQL, osquery SQL, KAPE targets, Plaso parsers) already exist and are battle-tested. Without a killer feature that *requires* a new language, JOCKY is a wrapper, not a breakthrough.

## Already Common
| Capability | Existing Tools |
|---|---|
| Cross-platform collection | Velociraptor, GRR, osquery |
| Structured evidence model | CASE/UCO, Plaso, Velociraptor artifacts |
| Provenance / hash chains | ProvQL/Provexa, DFIR-ORC, Velociraptor notebooks |
| Timeline + correlation | Plaso, Timesketch, Velociraptor hunt results |
| Investigation graph / link analysis | Timesketch, Graph Explorer, Maltego, KQL |
| Report / dashboard | Velociraptor notebooks, GRR flows, Timesketch, KAPE reports |
| Reproducible workflows | Velociraptor artifacts/VQL, GRR flows, KAPE targets, Ansible/Playbooks |

## Differentiating
1. **Single declarative language** spanning *collection → normalization → correlation → reporting* without switching tools/contexts.
2. **Evidence-first semantics** baked into the IR: every node carries provenance, hash, chain-of-custody, and collection metadata *by default*, not as an afterthought.
3. **Platform-agnostic IR** that compiles to native collectors (Velociraptor VQL, osquery SQL, KAPE modules, Volatility plugins) *and* to a portable evidence bundle (CASE/UCO JSON-LD) for offline analysis.

## Weak Claims
- "First evidence-first language" → False. ProvQL, CASE/UCO, and Velociraptor artifacts already encode evidence semantics.
- "Unified cross-platform collection" → Velociraptor/GRR/osquery do this.
- "Automatic timeline correlation" → Plaso + Timesketch is the standard.
- "Tamper-proof provenance" → ProvQL/Provexa, DFIR-ORC, signed VQL artifacts exist.
- "No vendor lock-in" → CASE/UCO is the open standard; JOCKY adds another format.

## Strongest Combination
**Language-as-IR + Compile-to-Everywhere + Evidence-as-First-Class-Type.**
- Write once → emit Velociraptor VQL *or* osquery SQL *or* KAPE target *or* Volatility plugin *or* CASE bundle.
- Every IR node = (data, provenance, hash, timestamp, collector-metadata) tuple—enforced by type system.
- Deterministic replay: same script + same target → bit-for-bit identical evidence bundle.

## Final USP
**JOCKY is the *only* DFIR language that compiles a single investigation script into native collectors across Velociraptor, osquery, KAPE, and Volatility while emitting a complete, signed CASE/UCO evidence bundle with built-in provenance types—no post-processing required.**

## Judge Question
"Why not just use Velociraptor?"

## Best Answer
"Velociraptor excels at live hunting with VQL, but forces you to write VQL for collection, a different syntax for KAPE/Volatility offline parsing, and another toolchain for CASE/UCO export and court-ready provenance. JOCKY lets investigators author *one* script that *simultaneously* generates a Velociraptor hunt, a KAPE target set, a Volatility plugin invocation, and a signed CASE bundle—guaranteeing the same logic, same provenance, and same evidence integrity across live, dead, and cloud forensics. We don't replace Velociraptor; we make it one of many back-ends for a single, auditable investigation definition."