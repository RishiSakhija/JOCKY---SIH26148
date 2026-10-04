# RED-TEAM PRODUCT CRITIC ANALYSIS - SIH 26148 JOCKY

## CLAIM-BY-CLAIM ATTACK

### Claim 1: "We have a new programming language."
**Novel?** ❌ NO
**Who already does it:**
- Velociraptor: VQL (full DSL with artifacts, plugins, eventing, monitoring, remediation)
- osquery: SQL (100+ tables, extensions API, universal skill set)
- GRR: Artifact system (YAML), VFS abstraction, Python API
- Wazuh: Rule/decoder system (XML), osquery integration
- KAPE: Target/Module YAML definitions
- Volatility 3: Plugin framework (Python), symbol tables
- Elastic/Splunk: KQL, SPL, EQL, ES|QL

**Judge Challenge:** "VQL already solves 'forensic query language' with 500+ artifacts. SQL is universal. New language = new learning curve, zero ecosystem, no artifact library. Why not write VQL artifacts or osquery extensions?"

**Strengthen:** Don't build a language. Build a COMPILATION TARGET for existing languages. If language is essential: make it a TRANSPILER to VQL/SQL/Elastic/Sigma/YARA. Focus on SEMANTIC INTEROPERABILITY not syntax.

---

### Claim 2: "We support Windows and Linux."
**Novel?** ❌ NO
**Who already does it:** Velociraptor, osquery, GRR, Wazuh (Win/Lin/macOS/Solaris/AIX/HP-UX), Volatility 3, Elastic, Splunk
**Judge Challenge:** "Table stakes. Every serious platform does this. 'We support both' = 'We do the minimum.'"
**Strengthen:** Cross-platform ARTIFACT PARITY (same artifact name works identically), UNIFIED TIMELINE across OS in single view, CROSS-OS CORRELATION (lateral movement detection).

---

### Claim 3: "We have a central dashboard."
**Novel?** ❌ NO
**Who already does it:** Velociraptor (Web UI, notebooks, hunts), GRR (AngularJS, REST API), Wazuh (Kibana, 50+ dashboards), Elastic (Timeline, Visual Event Analyzer, Session View, Cases), Splunk ES (Workbench, Risk Timeline, Asset/Identity Investigator)
**Judge Challenge:** "Dashboards are commoditized. What can your dashboard DO that Elastic/Splunk/Velociraptor cannot?"
**Strengthen:** Dashboard as INVESTIGATION WORKBENCH not visualization. LIVE COLLABORATIVE TIMELINE with hypothesis tracking. ONE-CLICK PIVOT from finding → hunt query → deployment.

---

### Claim 4: "We automate forensic collection."
**Novel?** ❌ NO
**Who already does it:** Velociraptor (artifacts, offline collectors, scheduled hunts, API), KAPE (targets/modules, compound targets, VHDX, cloud upload - GOLD STANDARD for triage), GRR (artifacts, hunts, cron, Python API), Wazuh (Syscollector, active response), osquery (scheduled packs, FIM), Elastic (Osquery Manager, curated packs, detection→investigation loop)
**Judge Challenge:** "KAPE does triage in minutes. Velociraptor does fleet-wide with resource controls. 'Automation' without INTELLIGENT PRIORITIZATION is just scripting."
**Strengthen:** EVIDENCE-DRIVEN COLLECTION (collect ONLY what answers current hypotheses), ADAPTIVE COLLECTION (results from host A change plan for host B), FORENSIC SOUNDNESS by default (hash chains, write-blocking simulation).

---

### Claim 5: "We use AI."
**Novel?** ❌ NO
**Who already does it:** Elastic (Attack Discovery, AI Chat, Entity Analytics ML), Splunk (Risk-based alerting, SOAR, MLTK), Wazuh (ML anomaly detection), DFIR Companion (Local LLM for timeline synthesis, findings, reports), Clouseau (Multi-agent LLM investigation), ForenSift (GenAI + LangChain), InquestiQ (ML risk scoring), SANS FOR563 (teaching local LLM deployment)
**Judge Challenge:** "'AI' is table stakes. Show me a finding your AI made that a skilled analyst would miss. Hallucination risk = evidence inadmissibility. Black-box AI = cannot testify in court."
**Strengthen:** DETERMINISTIC AI (every conclusion has clickable evidence citations), LOCAL/FREE MODEL ONLY (no data leaves environment), AI as SECOND PAIR OF EYES not primary analyst, EXPLAINABLE (show exact forensic events behind each AI finding).

---

### Claim 6: "We generate forensic timelines."
**Novel?** ❌ NO
**Who already does it:** Velociraptor (notebooks, Timesketch export), KAPE (mini_timeline - MFT+event logs+registry→CSV), Elastic (Timeline, Visual Event Analyzer), Splunk (Risk Timeline, swimlanes), GRR (Timesketch/BigQuery export), Volatility 3 (automatic timeline integration), DFIR Companion (algorithmic burst grouping + ATT&CK phase labeling), Plaso/log2timeline (INDUSTRY STANDARD super timeline engine)
**Judge Challenge:** "Plaso/log2timeline is the open source standard. KAPE does it in minutes. Elastic/Splunk do it at fleet scale. 'We make timelines' = 'We parse timestamps.'"
**Strengthen:** CAUSAL TIMELINE (not just 'when' but 'why connected' - process tree + file lineage + network), HYPOTHESIS-BOUND TIMELINE (filter to events relevant to current theory), COLLABORATIVE (multiple analysts annotate, merge conflicts resolved).

---

### Claim 7: "We have evidence integrity."
**Novel?** ❌ NO
**Who already does it:** Velociraptor (encrypted collections, audit logging, TLS, signed artifacts), GRR (signed binaries, TLS, Fleetspeak, audit logs), Wazuh (FIM, SCA, signed agents), KAPE (preserves timestamps, metadata logs, VHDX containers), DFIR Companion (hash-chained capture log, signed manifest, SHA-256 per artifact), Elastic/Splunk (immutable indices, audit trails, compliance)
**Judge Challenge:** "Everyone claims this. It's a BASELINE REQUIREMENT not a differentiator. Show me your chain of custody implementation. If you can't explain HOW it survives court challenge, it's marketing."
**Strengthen:** CRYPTOGRAPHIC EVIDENCE CHAIN (every transformation logged and verifiable), REPRODUCIBLE COLLECTION (same input → same output, verifiable by third party), COURT-READY EXPORT (package with verification manifest, tool versions, hashes).

---

### Claim 8: "We have an IR/AST/compiler."
**Novel?** ❌ NO
**Who already does it:** Velociraptor (VQL parser, AST, compiler, plugins), osquery (SQLite SQL parser, virtual tables, extension SDK), GRR (Artifact parser, VFS abstraction, Python engine), Volatility 3 (Symbol table parser, plugin framework, automagic), Elastic (ES|QL parser, EQL sequence engine), Wazuh (Rule/decoder compiler, correlation engine)
**Judge Challenge:** "Compiler/IR/AST is INFRASTRUCTURE not a product feature. What does your IR enable that VQL/SQL doesn't? If it doesn't enable a USER-VISIBLE CAPABILITY, it's academic."
**Strengthen:** CROSS-PLATFORM QUERY COMPILATION (write once → VQL/SQL/KQL/ES|QL/Sigma/YARA), FORMAL VERIFICATION (prove query correctness, resource bounds, no side effects), OPTIMIZATION (automatically rewrite queries for target platform performance).

---

## 5 DEFENSIBLE USP COMBINATIONS

### USP 1: Hypothesis-Driven Adaptive Collection Engine
**Description:** Analyst states a hypothesis (e.g., "attacker used living-off-the-land for lateral movement"). System automatically: (1) derives minimal evidence set needed, (2) generates cross-platform collection plan, (3) executes with forensic soundness, (4) returns ONLY evidence that confirms/denies hypothesis, (5) updates hypothesis confidence, (6) suggests next hypothesis. Collection adapts in real-time based on findings from each host.

**Components:** Hypothesis formalization, Evidence requirement derivation, Cross-platform collection planner, Adaptive execution engine, Confidence scoring with citations, Forensic soundness by default

| Dimension | Score |
|-----------|-------|
| Novelty | 8 |
| SIH Relevance | 9 |
| Feasibility Today | 7 |
| Demo Impact | 9 |
| Technical Depth | 8 |
| Judge Defensibility | 8 |
| **TOTAL** | **49/60** |

---

### USP 2: Unified Cross-Platform Forensic Semantic Layer
**Description:** Single artifact definition (e.g., "persistence.mechanisms") compiles to: Velociraptor VQL, osquery pack, KAPE target, GRR artifact, Elastic Osquery query, Sigma rule, YARA rule. Analyst writes ONCE in high-level semantic DSL. System handles: OS differences, artifact location differences, parser differences, output normalization. Enables true "write once, hunt everywhere" across live endpoints, disk images, memory dumps, cloud snapshots.

**Components:** High-level forensic semantic DSL, Multi-target compiler (VQL, SQL, KAPE, GRR, Sigma, YARA, ES|QL), Artifact registry with versioning, Cross-platform parity validation (CI on Win/Lin/macOS), Runtime engine for live/offline, Evidence normalization to ECS/OTel

| Dimension | Score |
|-----------|-------|
| Novelty | 7 |
| SIH Relevance | 8 |
| Feasibility Today | 6 |
| Demo Impact | 8 |
| Technical Depth | 9 |
| Judge Defensibility | 7 |
| **TOTAL** | **45/60** |

---

### USP 3: Causal Attack Graph with Evidence Provenance ⭐ WINNER
**Description:** Not a timeline. A GRAPH. Nodes = forensic events (process creation, file write, network connection, registry change, memory injection). Edges = CAUSAL relationships (spawned, wrote, read, injected, connected-to, loaded-by). Every edge has: evidence source (which artifact, which tool, which timestamp), confidence score, analyst verification status. Auto-constructed from ANY input (Velociraptor, KAPE, Volatility, osquery, EVTX, MFT, PCAP). Enables: "Show me the full causal chain from phishing email to domain controller compromise" with ONE CLICK. Export to court-ready narrative.

**Components:** Multi-source evidence ingest (live, disk, memory, network, cloud), Causal relationship inference engine (deterministic rules + optional AI assist), Graph database backend with time-travel queries, Evidence provenance tracking per node/edge (hash, tool, version, analyst), Interactive graph investigation UI, One-click narrative report with citations, Hypothesis overlay (mark nodes as supports/refutes hypothesis X)

| Dimension | Score |
|-----------|-------|
| Novelty | 9 |
| SIH Relevance | 10 |
| Feasibility Today | 6 |
| Demo Impact | 10 |
| Technical Depth | 9 |
| Judge Defensibility | 9 |
| **TOTAL** | **53/60** |

---

### USP 4: Live-Offline Parity Forensic Workbench
**Description:** SAME query, SAME artifact definition, SAME UI runs against: (1) Live fleet via Velociraptor/osquery/GRR, (2) Disk images (E01, raw, VHDX) via KAPE/Plaso/TSK, (3) Memory dumps via Volatility 3, (4) Cloud snapshots (AWS/Azure/GCP), (5) PCAP/Zeek logs. No rewriting queries. No learning new tools. Analyst develops investigation on live endpoint, then applies IDENTICAL logic to 500 offline images for scope determination. Results normalized to single timeline/graph.

**Components:** Unified query interface, Pluggable backends (live agents, disk mounters, memory analyzers, cloud APIs), Artifact implementation per backend, Result normalization to common schema, Unified investigation UI (timeline + graph + evidence viewer), Progressive disclosure (live triage → full image analysis)

| Dimension | Score |
|-----------|-------|
| Novelty | 7 |
| SIH Relevance | 9 |
| Feasibility Today | 5 |
| Demo Impact | 9 |
| Technical Depth | 8 |
| Judge Defensibility | 8 |
| **TOTAL** | **46/60** |

---

### USP 5: Collaborative Hypothesis-Tracking Investigation Platform
**Description:** Investigation = structured hypothesis tree, not query history. Root: "APT29 compromised domain controller." Children: "Initial access via phishing", "Lateral movement via WMI", "Credential theft via LSASS". Each hypothesis: required evidence, collected evidence, confidence, assigned analyst, status (open/confirmed/ruled-out). Platform: (1) Suggests evidence to collect per hypothesis, (2) Tracks collection progress across team, (3) Auto-updates confidence as evidence arrives, (4) Detects contradictory evidence, (5) Generates handoff-ready state for shift change, (6) Exports full investigation logic for court/peer review. Think: "Git for investigations" + "Jira for hypotheses".

**Components:** Hypothesis tree data model (structured, versioned, mergeable), Evidence-to-hypothesis mapping (many-to-many with confidence), Real-time collaborative editing (CRDT), Collection task generation from hypothesis gaps, Shift handoff export/import, Court export (hypothesis tree + evidence citations + analyst attestations), Integration with Velociraptor/KAPE/Volatility

| Dimension | Score |
|-----------|-------|
| Novelty | 8 |
| SIH Relevance | 9 |
| Feasibility Today | 7 |
| Demo Impact | 8 |
| Technical Depth | 7 |
| Judge Defensibility | 8 |
| **TOTAL** | **47/60** |

---

## WINNER: USP 3 - Causal Attack Graph with Evidence Provenance

**Score: 53/60** (Highest across all dimensions)

### WHY THIS WINS

1. **BUILDABLE TODAY:** Core graph engine + hypothesis model + 2-3 backend integrations (Velociraptor, KAPE, Volatility 3) achievable in hackathon timeframe. Graph DB (Neo4j/JanusGraph) + Python backend + React frontend = standard stack.

2. **DEMOABLE LIVE:** Load a sample case (memory dump + disk image + PCAP) → show auto-constructed graph → add hypothesis "APT29 lateral movement" → watch confidence update as evidence maps → one-click court-ready narrative export.

3. **CLEAR TECHNICAL EXPLANATION:** 
   - Causal inference from forensic artifacts (deterministic rules: process A spawned B → edge "spawned" with evidence citation)
   - Provenance tracking (every node/edge stores: source tool, artifact, hash, timestamp, analyst)
   - Graph database enables time-travel queries ("show graph at T+2hours")
   - Hypothesis overlay is a separate layer on the graph (not modifying evidence)

4. **NO OFFENSIVE DEPENDENCY:** Pure defensive analysis. No evasion, no malware implementation, no weaponization. Only parsing, correlation, and reconstruction.

5. **ANSWERS "WHY NOT VELCIRAPTOR/OSQUERY?":** 
   - Velociraptor = **COLLECTION ENGINE** (excellent at gathering artifacts from live endpoints)
   - osquery = **QUERY ENGINE** (excellent at SQL-based endpoint state querying)
   - KAPE = **TRIAGE ENGINE** (excellent at fast artifact extraction from disk)
   - Volatility = **MEMORY ENGINE** (excellent at RAM analysis)
   - **JOCKY = INVESTIGATION ENGINE** (correlates ACROSS all sources into CAUSAL GRAPH with HYPOTHESIS TRACKING)
   
   We don't replace them. We CONSUME their output and solve the "now what?" problem: turning 50,000 artifacts from 5 tools into a defensible attack narrative.

---

## OUR RECOMMENDED USP:

**Causal Attack Graph with Evidence Provenance:** A causal attack graph platform that auto-constructs evidence-backed attack narratives from ANY forensic source (live, disk, memory, cloud), tracks investigator hypotheses with confidence scoring, and exports court-ready investigation logic — turning fragmented artifact collection into structured, defensible incident reconstruction.