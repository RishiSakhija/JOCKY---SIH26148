# One-Day Build Plan
## SIH 26148 - JOCKY Forensic Analysis Language

## Must Work
- **DSL parser** that accepts a minimal forensic investigation script (expresses collection + correlation + timeline)
- **AST/IR** representing: evidence nodes (process, file, network, registry), provenance (source, hash, timestamp), and hypothesis links
- **End-to-end replay demo** using controlled sample data: load a PCAP + disk image artifact set → produce a causal graph path → export a narrative
- **Single source file** that compiles to a usable output (Velociraptor VQL fragment + KAPE target list + CASE/UCO-lite bundle)

## Simplify
- **No new query language from scratch** — use a Configurable Expression Language (CEL-lite syntax) or a minimal custom DSL with ~10 statements
- **No custom graph database** — use in-memory NetworkX graph or even plain Python dicts for the demo
- **No custom collectors** — have the compiler emit Velociraptor VQL + KAPE targets; don't build new Windows/Linux collectors
- **No dashboard UI** — CLI-driven; input = script, output = graph dot file + CASE/UCO JSON

## Replay / Sample
- Use **Plaso-parsed data** (already available: EVTX/ MFT parsed into super-timeline CSV) as the investigation source
- Pre-load 2-3 forensic artifacts (process tree, network connection, registry run key) from a known sample case
- Demo sequence: load sample → run one DSL script → show causal chain → export CASE/UCO bundle
- All data lives in memory; no runtime installation required

## Defer
- **Security-resilient execution model** (capability-based sandbox, signed scripts) — defer to Week 2
- **Cross-platform compilation** (compile to osquery SQL, Volatility plugins) — defer; just do VQL + KAPE for now
- **Remote/agent protocol** — local-only replay
- **Web dashboard** — CLI + CSV/JSON output only
- **Multi-user collaboration** — single-analyst workflow
- **Timeline engine from scratch** — leverage Plaso/Timesketch output; just display timestamped events from parsed data

## Core Modules (8 max)
1. **DSL Parser** — custom minimal language (10 statements max): `collect process`, `collect file`, `collect network`, `link hypothesis`, `timeline show`, `graph add`, `export case`
2. **AST Builder** — IR nodes: `EvidenceNode` (type, source, hash, timestamps), `HypothesisEdge` (causal, confidence), `ScriptBlock`
3. **Causal Inference Engine** — deterministic rules: `process_spawned → edge`, `file_written → edge`, `connection_made → edge`; builds NetworkX graph
4. **Velociraptor Compiler** — translates DSL → VQL snippet for artifact hunt (e.g., `select file where name == "*"`)
5. **KAPE Target Generator** — translates DSL → KAPE target config (file globs, registry paths)
6. **CASE/UCO Exporter** — minimal JSON-LD bundle: evidence nodes → cases, hypotheses, with hash chains
7. **Sample Data Loader** — reads Plaso CSV output; pre-seeds evidence nodes from known forensic sample
8. **CLI / Demo Orchestrator** — ties it all together: `jockey compile script.jky → graph.dot → case.json`

## Build Order (critical path → optional)
1. Define DSL syntax (1 hour) — write EBNF, implement minimal parser (Python `pyparsing` or hand-rolled)
2. Build AST/IR types (2 hours) — evidence nodes + provenance fields
3. Implement causal inference rules (3 hours) — process spawn + file write + network = graph edges
4. Write Velociraptor compiler (2 hours) — simple template mapping
5. Write KAPE target generator (2 hours) — file/registry mapping
6. Implement CASE/UCO exporter (2 hours) — JSON-LD skeleton with hash chains
7. Write sample data loader (2 hours) — read 1 Plaso CSV, create 5-6 evidence nodes
8. Wire CLI orchestrator + demo script (3 hours) — end-to-end flow

## Demo Flow (exact 8-step sequence)
1. **Load sample**: `jockey load sample_case/` → reads Plaso CSV, creates in-memory evidence nodes (process creation, DNS query, registry run key, file write)
2. **Show graph**: `jockey graph show` → displays dot output of causal chain (process A → process B → network connection → file write)
3. **Run DSL**: `jockey run investigate.jky` — DSL script that queries the graph:
   - `collect process where name == "svchost.exe"`
   - `link hypothesis "APT29 lateral movement" → confidence 0.7`
   - `timeline show last 30 minutes`
4. **Compile**: `jockey compile investigate.jky` → emits two files:
   - `hunt.vql` (Velociraptor VQL hunt script)
   - `kape_targets.json` (KAPE target configuration)
   - `evidence.case.json` (CASE/UCO bundle with provenance)
5. **Export narrative**: `jockey narrative → evidence_report.md` — one-page causal story with citations
6. **Verify integrity**: display hash chain of evidence nodes; confirm bit-for-bit replay determinism
7. **Q&A**: explain how same script runs on live Velociraptor fleet OR offline disk images
8. **Close** — emphasize: JOCKY = investigation engine, not collector; consumes Velociraptor/KAPE output

## Biggest Implementation Risks
- **DSL scope creep** — resist adding features; keep exactly 10 statements; use timed sprint guard
- **Graph too complex** — in-memory NetworkX may struggle; limit to ≤50 nodes for demo; use adjacency list instead
- **CASE/UCO JSON-LD structure** — easy to get namespaces wrong; use pre-defined template from Plaso/Velociraptor docs
- **Sample data quality** — if Plaso CSV is malformed, demo fails; prepare 2 pre-sanitized CSV examples
- **Compiler output validity** — VQL/KAPE must be syntactically correct; test against actual Velociraptor/KAPE binaries before demo day

## Final Recommendation
**Build a "Language-as-IR + Compile-to-Everywhere" prototype.** JOCKY’s value is letting one investigator write a single forensic script that simultaneously generates Velociraptor VQL, KAPE targets, and a CASE/UCO evidence bundle—all from one source. Use Plaso-parsed sample data for the replay demo. The causal graph + provenance tracking is the differentiator; the language syntax is the vehicle. Keep it to 8 core modules, 8-step demo, and stop. Reliability (deterministic replay, correct hash chains) beats feature count. On demo day: "One script. One graph. Three back-ends. Zero post-processing." That is the strongest, most defensible live demo.