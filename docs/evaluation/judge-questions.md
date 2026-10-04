# Anticipated Judge Questions

**Status**: SPECIFIED

---

## Technical Architecture

### Q: "Why not just use Velociraptor?"
**A:** Velociraptor excels at live hunting with VQL, but forces you to write VQL for collection, a different syntax for KAPE/Volatility offline parsing, and another toolchain for CASE/UCO export and court-ready provenance. JOCKY lets investigators author **one script that simultaneously generates a Velociraptor hunt, a KAPE target set, a Volatility plugin invocation, and a signed CASE bundle** — guaranteeing the same logic, same provenance, and same evidence integrity across live, dead, and cloud forensics. We don't replace Velociraptor; we make it one of many back-ends for a single, auditable investigation definition.

### Q: "How is this different from osquery?"
**A:** osquery is a **snapshot query engine** (SQL on OS state). It has no procedural collection, no timeline engine, no hypothesis tracking, no evidence receipts, no provenance chains, and no compilation to other backends. JOCKY adds: hypothesis-driven collection, deterministic correlation with evidence citations, cryptographic receipts at collection time, hash-linked provenance, and compilation to Velociraptor/KAPE/Volatility/CASE.

### Q: "What about KAPE? It's the gold standard for Windows triage."
**A:** KAPE is excellent for **Windows triage speed** (targets/modules, VSS, compound targets). But it's Windows-only, config-driven (no query language), no live/remote capability, and no cross-platform story. JOCKY adopts KAPE's target/module pattern but makes it query-driven, cross-platform, centrally deployable via GitOps, and with cryptographic evidence receipts and provenance. JOCKY compiles TO KAPE targets.

### Q: "How does this handle memory forensics?"
**A:** JOCKY doesn't reimplement memory forensics — Volatility 3 owns that space. JOCKY **compiles to Volatility plugins** for memory analysis, and adds live memory acquisition via WinPMEM/LiME integration (future). The value is correlating memory findings (Volatility) with host artifacts (process, file, registry) and network evidence in a single causal graph with shared provenance.

### Q: "What about eBPF? That's the future of Linux forensics."
**A:** We agree — eBPF (via Aya) is documented as our **extensible architecture direction** for Linux collectors (proc_ebpf, file_ebpf, net_ebpf). It's not in MVP because: kernel version compatibility (5.10+ for BTF/CO-RE), verifier complexity, and we want graceful fallback to userspace collectors. The architecture supports dynamic eBPF loading with automatic unload and capability bounding.

### Q: "Is the correlation engine using AI/ML?"
**A:** **No.** JOCKY uses **deterministic evidence correlation** — rule-based joins on observable keys (PID/PPID, timestamps, paths, IPs, hashes) with explicit confidence scores and method names. Every edge cites supporting evidence IDs. No opaque ML scoring. This is by design for court defensibility.

---

## Evidence & Provenance

### Q: "How do you prove evidence wasn't tampered with?"
**A:** Three-layer integrity:
1. **Evidence Receipt** — Ed25519 signature by collector over `blake3(payload) || evidence_id || timestamp` at collection time
2. **Provenance Chain** — Hash-linked records for every transformation (collected -> transformed -> correlated -> exported), each signed by actor
3. **Verification Engine** — Re-validates all signatures, recomputes all hashes, checks chain continuity. Any modification -> immediate FAIL with specific location.

### Q: "Are your receipts legally admissible?"
**A:** We **do not claim automatic court admissibility**. Legal admissibility depends on jurisdiction, procedure, and expert testimony. Our receipts provide: cryptographic integrity (Ed25519), collector identity (public key), collection context (host info, timestamp), and tamper detection. These are **technical foundations** for admissibility arguments; the legal determination is outside our scope.

### Q: "What if the collector binary is compromised?"
**A:** If the collector binary is compromised, its private key could sign malicious receipts. Mitigations:
- Collector binaries signed and verified at load time
- Keys embedded at build time, not runtime configurable
- Capability bounding limits what compromised collector can access
- Multi-collector consensus for critical evidence (future)
- Supply chain security: reproducible builds, SBOM

### Q: "How do you handle clock skew between systems?"
**A:** Timestamps are UTC (RFC3339). Verification allows ±5 minute skew tolerance. For high-precision correlation, we use monotonic timestamps where available (e.g., `monotonic_timestamp` in journald) and logical ordering via provenance chain.

### Q: "Can investigators add manual annotations to evidence?"
**A:** Yes — Findings support analyst-added evidence citations. Hypotheses track supporting/refuting evidence. Future: collaborative annotation layer (CRDT-based) for multi-analyst investigations.

---

## Correlation & Graph

### Q: "How do you correlate across Windows and Linux?"
**A:** **Semantic evidence types** abstract platform differences:
- `collect process` -> `proc_windows` (ETW/WMI) or `proc_linux` (/proc/auditd)
- `collect file` -> `file_windows` (NTFS/USN/VSS) or `file_linux` (ext4/fanotify)
- `collect registry` -> `registry_windows` or `config_linux` (systemd/journald)
Correlation rules operate on **common semantic fields** (pid, path, hash, timestamp, IP) regardless of source OS.

### Q: "What correlation methods do you support?"
**A:** 9 deterministic rules in MVP:
| Method | Relation | Confidence |
|--------|----------|------------|
| pid_link | spawned | 1.0 |
| file_write | wrote | 0.95 |
| file_read | read | 0.9 |
| net_link | connected | 0.95 |
| ioc_match | resolved_to | 1.0 |
| yara_match | contains | 0.9 |
| sigma_match | contains | 0.9 |
| rule_match | indicates | 0.8 |
| timestamp_join | timeline_adjacent | 0.7 (configurable window) |

### Q: "Can analysts define custom correlation rules?"
**A:** Yes — rules are JSON configuration loaded at runtime. Analysts can add rules with custom conditions (expression language) and methods. Future: rule marketplace with versioning.

### Q: "How does hypothesis tracking work?"
**A:** Hypotheses are first-class DSL constructs with MITRE tags. Analysts `bind hypothesis "desc" to evidence_bindings`. The graph overlay shows gold borders on supporting nodes, red on refuting. Confidence auto-updates based on evidence count; analysts can manually adjust. Export includes hypothesis tree with evidence citations.

---

## Implementation & Deployment

### Q: "What's the current implementation status?"
**A:** **Phase 0 Complete** — Specifications, architecture, research, documentation, diagrams, CI/CD, Cargo workspace. **No implementation code exists yet.** Ready for Phase 1 (DSL parser, AST, IR).

### Q: "Why Rust? Isn't Go better for this?"
**A:** Rust provides: memory safety (critical for forensic correctness), performance (par with C/C++), WASM target (future collector plugins), growing forensics ecosystem (Volatility 3 Rust plugins, Aya eBPF), and strong type system for our capability-based security model. Go's GC and runtime are less suitable for evidence integrity guarantees.

### Q: "How do you deploy this?"
**A:** **MVP: Single binary** — `jockey` contains API server, runtime, collectors, SQLite, frontend assets. Run `jockey serve` for API + dashboard, or `jockey run script.jky` for CLI. Future: server + agents via gRPC/mTLS for fleet management.

### Q: "What about remote/agent-based collection?"
**A:** **MVP is local execution only.** Remote via SSH/WinRM orchestration is documented for post-MVP. Full agent protocol (gRPC/mTLS, capability negotiation, enrollment) is Phase 2+.

### Q: "What's the binary size?"
**A:** Target < 50 MB release (striped, LTO). Static linking where possible. Frontend assets embedded.

### Q: "How do you handle Windows kernel drivers?"
**A:** **MVP uses userspace collectors only** (Windows APIs: ETW, WMI, PSAPI, Registry, EVTX). Kernel drivers (ETW kernel provider, minifilter) are documented for post-MVP. Avoids driver signing complexity and kernel crash risk in MVP.

---

## Scope & Vision

### Q: "What's NOT in scope?"
**A:** Explicitly out of scope for MVP:
- AI/ML correlation (deterministic only)
- Multi-tenancy/RBAC (single analyst)
- Remote agent protocol (local only)
- Kernel drivers (userspace only)
- Mobile/cloud/container forensics
- Real-time streaming dashboard
- Custom graph database (petgraph in-memory)
- Legal hold/preservation workflows

### Q: "What's the long-term vision?"
**A:** **Evidence-Contract Forensics Platform** — unified workflow from intent to verified report, compiling to all major DFIR backends, with cryptographic evidence integrity, deterministic correlation, and court-ready export. Become the "Git for investigations" + "compiler for forensics."

### Q: "How do you sustain this after SIH?"
**A:** Open source (MIT), community-driven, plugin architecture for collectors/rules/exports. Seek academic/industry partnerships for formal verification (Prusti/Kani) and standards alignment (CASE/UCO, NIST).

---

## Related Documents

- `evaluation-plan.md` — Evaluation plan
- `success-criteria.md` — Success criteria
- `demo-plan.md` — Demo script
- `limitations.md` — Limitations
- `risk-register.md` — Risk register