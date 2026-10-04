# Competitor Analysis

**Status**: SPECIFIED

---

## Comparison Matrix

| System | Query/DSL | Collection | Cross-Platform | Provenance | Correlation | Graph | What JOCKY Adds |
|--------|-----------|------------|----------------|------------|-------------|-------|-----------------|
| **Velociraptor** | VQL (SQL-like, functional) | Artifacts, VFS, offline | Win/Lin/Mac | Artifact signing, notebooks | Hunt results, notebooks | Notebooks, timelines | Compilation to VQL + KAPE + Volatility + CASE; execution contracts; evidence receipts; deterministic correlation |
| **osquery** | Standard SQL | Tables only (snapshot) | Win/Lin/Mac/BSD | Limited (FIM) | Packs only | No native | Compilation to SQL; procedural collection; timeline; evidence receipts; provenance |
| **GRR** | Python flows + YAML artifacts | Files, memory, artifacts | Win/Lin/Mac | Signed binaries, TLS | Flow-based | No native | Native query language (not Python); execution contracts; evidence receipts; CASE export |
| **KAPE** | None (config: targets/modules) | Best-in-class Windows | Windows only | VHDX containers, logs | Via modules | No native | Cross-platform; query-driven; compilation to KAPE targets; provenance |
| **Volatility 3** | Python API + plugins | Memory only | Analysis: Win/Lin/Mac | Plugin output | Plugin-based | Timeliner plugin | Compilation to Volatility plugins; live memory acquisition; correlation with host/network |
| **Sysmon** | None (XML config) | Event logging only | Win + Lin (sep) | Tamper-protected driver | Rule filtering | No | Consume Sysmon events; query language; correlation; CASE export |
| **Wazuh** | Rules (XML) + WQL (API) | Logs, FIM, vuln, config | Win/Lin/Mac/Unix | FIM, SCA | 5000+ rules, MITRE | Kibana dashboards | Forensic query language; deep artifact collection; evidence receipts; CASE export |
| **Fleet** | SQL + YAML + Lua | osquery tables | All + mobile/cloud | osquery FIM | CIS, vuln | Fleet UI | Forensic depth (memory, MFT, VSS); compilation to Fleet packs; evidence receipts |
| **Timesketch** | ES DSL / Lucene + Python | None (ingestion) | Web (data any) | Analyzer plugins | Analyzers (Sigma, ML) | Collaborative timelines | Collection + query language; compilation; evidence receipts; provenance |
| **Plaso** | None (CLI + Python API) | Parsing 1000+ formats | Runs anywhere | Analysis plugins | Analysis plugins | Super timelines | Query language; live collection; correlation; CASE export |
| **Provexa/ProvQL** | DSL + provenance | Investigation tracking | Platform-agnostic | Provenance graph | Provenance-based | Provenance graph | Execution contracts; compilation targets; evidence receipts; deterministic correlation |
| **CASE/UCO** | Ontology (JSON-LD) | None (exchange format) | Platform-agnostic | Provenance support | Via ontology | Via ontology | Query language; execution runtime; collectors; compilation; deterministic correlation |

---

## Detailed Comparison

### Velociraptor
| Aspect | Velociraptor | JOCKY |
|--------|--------------|-------|
| **Query Language** | VQL (mature, 500+ artifacts) | JOCKY DSL (compiles to VQL) |
| **Collection** | Live hunting, artifacts, VFS | Same + compiles to KAPE/Volatility |
| **Cross-Platform** | Win/Lin/Mac | Win/Lin (Mac future) |
| **Provenance** | Artifact signing, notebooks | Evidence receipts + hash-linked provenance |
| **Correlation** | Hunt results, notebooks | Deterministic rules + investigation graph |
| **Export** | VQL, JSON, Timesketch | VQL, KAPE, Volatility, CASE, Cytoscape |
| **JOCKY Advantage** | Single script → multiple backends; execution contracts; evidence receipts |

### osquery
| Aspect | osquery | JOCKY |
|--------|---------|-------|
| **Query Language** | Standard SQL | JOCKY DSL (compiles to SQL) |
| **Collection** | Snapshot tables only | Procedural + timeline + correlation |
| **Cross-Platform** | Win/Lin/Mac/BSD | Win/Lin |
| **Provenance** | FIM only | Full evidence receipts + provenance chain |
| **Correlation** | Packs only | Deterministic rules + graph |
| **Export** | SQL, JSON, Fleet | SQL, KAPE, Volatility, CASE |
| **JOCKY Advantage** | Procedural collection; timeline; provenance; compilation to osquery |

### GRR
| Aspect | GRR | JOCKY |
|--------|-----|-------|
| **Query Language** | Python flows + YAML | JOCKY DSL (compiles to GRR artifacts) |
| **Collection** | Files, memory, artifacts | Same + compilation to GRR artifacts |
| **Cross-Platform** | Win/Lin/Mac | Win/Lin |
| **Provenance** | Signed binaries, TLS | Evidence receipts + hash-linked provenance |
| **Correlation** | Flow-based | Deterministic rules + graph |
| **Export** | Timesketch, BigQuery | GRR artifacts, CASE, Cytoscape |
| **JOCKY Advantage** | Native DSL (not Python); execution contracts; compilation to GRR |

### KAPE
| Aspect | KAPE | JOCKY |
|--------|------|-------|
| **Query Language** | None (targets/modules) | JOCKY DSL (compiles to KAPE targets) |
| **Collection** | Windows triage (best-in-class) | Cross-platform + compiles to KAPE |
| **Cross-Platform** | Windows only | Win/Lin |
| **Provenance** | VHDX, logs | Evidence receipts + provenance chain |
| **Correlation** | Via modules | Deterministic rules + graph |
| **Export** | Timeline Explorer, CSV | KAPE targets, CASE, Cytoscape |
| **JOCKY Advantage** | Query-driven; cross-platform; provenance; compilation to KAPE |

### Volatility 3
| Aspect | Volatility 3 | JOCKY |
|--------|--------------|-------|
| **Query Language** | Python API + plugins | JOCKY DSL (compiles to Volatility plugins) |
| **Collection** | Memory only | Live memory + host + network |
| **Cross-Platform** | Analysis: Win/Lin/Mac | Win/Lin (collection) |
| **Provenance** | Plugin output | Evidence receipts + provenance |
| **Correlation** | Timeliner plugin | Deterministic rules + graph |
| **Export** | JSON, Timesketch | Volatility plugins, CASE |
| **JOCKY Advantage** | Live memory acquisition; cross-source correlation; compilation to Volatility |

---

## JOCKY's Unique Position

| Dimension | JOCKY Position |
|-----------|----------------|
| **Primary Innovation** | Compilation target unification + execution contracts + evidence receipts |
| **Not Competing With** | Velociraptor, osquery, KAPE, Volatility, Timesketch |
| **Complements** | All above — JOCKY compiles TO them |
| **Unique Value** | One script → VQL + SQL + KAPE + Volatility + CASE + Cytoscape + Timesketch |
| **Security Model** | Execution contracts (capability-based) + evidence receipts (Ed25519) |
| **Correlation** | Deterministic, explainable, evidence-cited |
| **Provenance** | Hash-linked chain from collection to export |

---

## Related Documents

- `research-overview.md` — Research categories
- `literature-review.md` — Verified research sources
- `novelty-analysis.md` — Differentiation
- `research-to-design.md` — Research to architecture mapping
- `sources.md` — Bibliography