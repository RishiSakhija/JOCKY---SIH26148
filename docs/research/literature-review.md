# Literature Review

**Status**: SPECIFIED

---

## Purpose

Verified academic and industry research sources relevant to JOCKY's architecture. Sources verified against primary publications where available.

---

## Forensic Domain-Specific Languages

### Nugget: A Digital Forensics Language
- **Authors**: Stelly, C., Roussev, V.
- **Year**: 2018
- **Venue**: Digital Investigation
- **DOI**: 10.1016/j.diin.2018.01.006
- **URL**: https://doi.org/10.1016/j.diin.2018.01.006
- **Contribution**: DSL for forensic analysis with evidence types, provenance, and reproducibility
- **Overlap with JOCKY**: Evidence-first semantics, provenance tracking, reproducible execution
- **What it does NOT solve**: Cross-platform compilation, execution contracts, deterministic correlation engine, evidence receipts
- **Design Implication**: Proves DSL approach viable; JOCKY differentiates via compilation targets and execution contracts

### VQL (Velociraptor Query Language)
- **Source**: Velociraptor Documentation
- **URL**: https://docs.velociraptor.app/docs/vql/
- **Contribution**: SQL-like functional query language with artifacts, plugins, event queries
- **Overlap with JOCKY**: Forensic query language, artifact system, live hunting
- **What it does NOT solve**: Compilation to other backends (KAPE, Volatility, CASE), execution contracts, evidence receipts, provenance chains
- **Design Implication**: JOCKY compiles TO VQL; VQL is a backend target

### KQL (Kusto Query Language)
- **Source**: Microsoft Azure Data Explorer
- **URL**: https://learn.microsoft.com/en-us/azure/data-explorer/kusto/query/
- **Contribution**: Time-series optimized query language for log analytics
- **Overlap with JOCKY**: Time-series queries, joins, aggregations
- **What it does NOT solve**: Forensic evidence model, provenance, compilation to collectors
- **Design Implication**: Study KQL time-series operators for JOCKY timeline queries

### ES|QL (Elasticsearch Query Language)
- **Source**: Elastic
- **URL**: https://www.elastic.co/guide/en/elasticsearch/reference/current/esql.html
- **Contribution**: Piped, SQL-like query language for Elasticsearch
- **Overlap with JOCKY**: Piped syntax, time-series, security analytics
- **What it does NOT solve**: Forensic evidence types, compilation, execution contracts
- **Design Implication**: Consider piped syntax for JOCKY correlation expressions

---

## Provenance & Evidence Integrity

### Provexa / ProvQL
- **Authors**: Tsegai, S.A., Yang, X., Liu, H., Gao, P.
- **Year**: 2025
- **Venue**: Proceedings of the VLDB Endowment (PVLDB)
- **Volume**: 18, Issue 11
- **Pages**: 3771-3783
- **DOI**: 10.14778/3749646.3749653
- **Contribution**: DSL + provenance graph for investigation, evidence tracking
- **Overlap with JOCKY**: DSL + provenance graph, investigation tracking
- **What it does NOT solve**: Cross-platform compilation, execution contracts, deterministic correlation rules, evidence receipts
- **Design Implication**: Proves DSL+provenance combination; JOCKY adds compilation targets and execution contracts

### DFIR-ORC (Open Resilient Collection)
- **Source**: GitHub
- **URL**: https://github.com/ORC-Project/ORC
- **Contribution**: Evidence collection format with integrity verification
- **Overlap with JOCKY**: Evidence integrity, collection standardization
- **What it does NOT solve**: Query language, correlation, investigation graph, hypothesis tracking
- **Design Implication**: Consider ORC as export format option

### CASE / UCO (Unified Cyber Ontology)
- **Source**: CASE Community
- **URL**: https://caseontology.org/
- **Contribution**: Ontology for cyber-investigation data exchange (JSON-LD)
- **Overlap with JOCKY**: Evidence modeling, interoperability, verification-backed export
- **What it does NOT solve**: Query language, execution runtime, collectors, correlation engine
- **Design Implication**: JOCKY exports to CASE/UCO; not a replacement

---

## Memory Forensics

### Volatility 3
- **Source**: Volatility Foundation
- **URL**: https://github.com/volatilityfoundation/volatility3
- **Contribution**: Memory forensics framework with symbol-backed correctness
- **Overlap with JOCKY**: Memory analysis, plugin architecture
- **What it does NOT solve**: Query language, live collection, cross-platform compilation, timeline correlation
- **Design Implication**: JOCKY compiles TO Volatility plugins; Volatility is a backend target

### Rusting Volatility
- **Source**: Community effort
- **URL**: https://github.com/volatilityfoundation/volatility3 (Rust plugins)
- **Contribution**: Rust implementations of Volatility plugins for performance/safety
- **Overlap with JOCKY**: Rust in memory forensics
- **What it does NOT solve**: DSL, provenance, correlation, compilation targets
- **Design Implication**: Supports Rust architectural direction; consider Rust plugins

### LEMON: eBPF-based Volatile Memory Acquisition
- **Authors**: Oliveri, A., Cavenati, M., De Rosa, S., Lakshmi Narasimhan, S., Balzarotti, D.
- **Year**: 2026
- **Venue**: Digital Investigation
- **DOI**: 10.1016/j.fsidi.2026.302045
- **URL**: https://doi.org/10.1016/j.fsidi.2026.302045
- **Contribution**: eBPF-based volatile memory acquisition for Android and hardened Linux
- **Overlap with JOCKY**: eBPF for forensics, kernel visibility
- **What it does NOT solve**: Query language, provenance, correlation, CASE export
- **Design Implication**: Validates eBPF approach for Linux collectors; JOCKY documents as future extension

---

## eBPF for Forensics

### Aya: Rust eBPF Library
- **Source**: Aya Project
- **URL**: https://aya-rs.dev/
- **Contribution**: Pure Rust eBPF development with BTF/CO-RE support
- **Overlap with JOCKY**: Rust eBPF for Linux collectors
- **What it does NOT solve**: Forensic DSL, evidence model, correlation, Windows collectors
- **Design Implication**: Technical foundation for Linux eBPF collectors (future)

### bpftrace
- **Source**: bpftrace Project
- **URL**: https://github.com/bpftrace/bpftrace
- **Contribution**: High-level tracing language for eBPF
- **Overlap with JOCKY**: eBPF programmability
- **What it does NOT solve**: Forensic evidence model, provenance, cross-platform, CASE export
- **Design Implication**: Study syntax for eBPF collector configuration

---

## Timeline & Graph Analysis

### Plaso / log2timeline
- **Source**: log2timeline Project
- **URL**: https://github.com/log2timeline/plaso
- **Contribution**: 1000+ parser formats, super timeline generation
- **Overlap with JOCKY**: Timeline generation, parser breadth
- **What it does NOT solve**: Query language, live collection, correlation, hypothesis tracking, provenance
- **Design Implication**: JOCKY consumes Plaso output; not a replacement

### Timesketch
- **Source**: Google
- **URL**: https://github.com/google/timesketch
- **Contribution**: Collaborative timeline analysis, sketch-based investigation
- **Overlap with JOCKY**: Timeline analysis, collaboration, hypothesis/sketch
- **What it does NOT solve**: Collection, query language, provenance, deterministic correlation, compilation
- **Design Implication**: JOCKY exports to Timesketch; Timesketch is analysis backend

### Maltego
- **Source**: Paterva
- **URL**: https://www.maltego.com/
- **Contribution**: Link analysis, graph visualization, transform framework
- **Overlap with JOCKY**: Investigation graph, link analysis
- **What it does NOT solve**: Forensic collection, query language, provenance, execution contracts
- **Design Implication**: Graph visualization inspiration; JOCKY uses Cytoscape.js

---

## Standards & Interoperability

### NIST SP 800-86 (Guide to Integrating Forensic Techniques)
- **Source**: NIST
- **URL**: https://csrc.nist.gov/publications/detail/sp/800-86/final
- **Contribution**: Forensic methodology, evidence handling
- **Overlap with JOCKY**: Evidence integrity, chain of custody
- **What it does NOT solve**: Technical architecture, DSL, correlation engine
- **Design Implication**: Align evidence handling with NIST guidelines

### STIX / TAXII
- **Source**: OASIS
- **URL**: https://oasis-open.github.io/cti-documentation/
- **Contribution**: Threat intelligence sharing, structured indicators
- **Overlap with JOCKY**: Indicator evidence type, threat intel integration
- **What it does NOT solve**: Forensic query language, collection, provenance, CASE export
- **Design Implication**: Support STIX indicators in JOCKY indicator evidence type

---

## Related Documents

- `research-overview.md` — Research categories
- `competitor-analysis.md` — Tool comparison
- `novelty-analysis.md` — Differentiation
- `research-to-design.md` — Research to architecture mapping
- `sources.md` — Bibliography