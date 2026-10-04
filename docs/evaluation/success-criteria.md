# Success Criteria

**Status**: SPECIFIED

---

## MVP Success Criteria

### Functional Completeness

| Criterion | Metric | Target | Validation |
|-----------|--------|--------|------------|
| DSL Parser | Grammar coverage | 100% of locked 10 statements | Grammar tests, example scripts parse |
| Type Checker | Capability inference | All collect statements infer capabilities | Type check tests |
| IR Compiler | Lowering completeness | All typed AST nodes lower to IR | IR golden tests |
| Execution Contract | Policy enforcement | All 7 enforcement points validated | Contract validation tests |
| Runtime | Dispatch correctness | Sequential + parallel execution works | Integration tests |
| Windows Collectors | 5 collectors implemented | proc, file, registry, logs, net | Collector contract tests |
| Linux Collectors | 5 collectors implemented | proc, file, config, logs, net | Collector contract tests |
| Evidence Model | 7 types supported | All types serialize/deserialize | Serialization tests |
| Evidence Receipts | Ed25519 + blake3 | Sign/verify works, hash matches | Crypto tests |
| Provenance | Hash-linked chain | Chain verification passes | Provenance tests |
| Correlation | 9 rules implemented | All rules produce correct edges | Correlation tests |
| Investigation Graph | petgraph + queries | 5 query types work | Graph query tests |
| Verification | Receipt + provenance | Tamper detection works | Tamper tests |
| API | REST + WebSocket | All endpoints respond | API tests |
| Frontend | 8 pages functional | Graph, timeline, evidence, findings, verify, report, export, dashboard | E2E tests |
| Exports | 4 formats | CASE/UCO, Cytoscape, Timesketch, Markdown | Export validation |
| CLI | 5 commands | run, compile, graph, verify, export | CLI tests |

### Quality Gates

| Gate | Threshold | Tool |
|------|-----------|------|
| Unit test coverage | >= 90% per crate | cargo tarpaulin |
| Integration test pass | 100% | cargo test --test integration |
| Clippy warnings | 0 (deny) | cargo clippy -- -D warnings |
| Format compliance | 0 diffs | cargo fmt --check |
| Security audit | 0 critical/high | cargo audit |
| Documentation coverage | All public items | cargo doc |

---

## Demo Success Criteria

### Technical Demo

| Step | Success Indicator |
|------|-------------------|
| 1. Load Sample | jockey load sample_case/ completes, shows event count |
| 2. Show Graph | jockey graph show outputs valid DOT/Cytoscape |
| 3. Run DSL | jockey run investigate.jky completes with findings |
| 4. Compile | jockey compile investigate.jky generates 3 valid files |
| 5. Export Narrative | jockey narrative produces readable Markdown with citations |
| 6. Verify Integrity | jockey verify reports PASS with details |
| 7. Tamper Demo | Modify file -> jockey verify reports FAIL with specifics |
| 8. Restore Demo | Restore -> jockey verify reports PASS |

### Demo Quality

| Criterion | Target |
|-----------|--------|
| Total time | < 8 minutes |
| Zero crashes | No panics, unhandled errors |
| Clear output | Human-readable at each step |
| Judge comprehension | Verbal explanation matches output |
| Visual aids | Graph, timeline, report visible in browser |

---

## Interoperability Success Criteria

| Format | Validator | Success |
|--------|-----------|---------|
| CASE/UCO JSON-LD | case-validator tool / manual review | Valid JSON-LD, required classes present |
| Cytoscape.js | Load in Cytoscape.js demo | Nodes/edges render, styles apply |
| Timesketch CSV | Import in Timesketch | Events appear, timestamps correct |
| Markdown Report | Render in browser | Citations clickable, evidence links work |

---

## Performance Success Criteria

| Metric | Target | Measurement |
|--------|--------|-------------|
| Collection throughput | > 1000 events/sec | proc_windows list on 10k processes |
| Correlation latency | < 100ms for 1000 evidence | 9 rules, full cross-product |
| Verification time | < 30s for 1000 evidence | Full receipt + provenance check |
| Graph query | < 50ms for 5000 nodes | path(), backward_from() |
| Export generation | < 10s for full case | CASE/UCO + Cytoscape + Timesketch |
| Memory usage | < 512 MB | Full investigation in memory |
| Binary size | < 50 MB | Release build |

---

## Security Success Criteria

| Check | Requirement |
|-------|-------------|
| No secrets in code | cargo audit + manual review |
| No unsafe without justification | cargo clippy + unsafe audit |
| Input validation on all APIs | All handlers validate params |
| Capability enforcement | Contract validation rejects unauthorized |
| Evidence root isolation | Path traversal attempts blocked |
| Tamper detection | Modified evidence -> verification FAIL |

---

## Documentation Success Criteria

| Document | Complete | Accurate | Linked |
|----------|----------|----------|--------|
| README.md | Yes | Yes | Yes |
| JOCKY_SPEC_SHEET.md | Yes | Yes | Yes |
| All docs/architecture/*.md | Yes | Yes | Yes |
| All docs/language/*.md | Yes | Yes | Yes |
| All docs/evidence/*.md | Yes | Yes | Yes |
| All docs/research/*.md | Yes | Yes | Yes |
| All docs/evaluation/*.md | Yes | Yes | Yes |
| COMPLETE_AUDIT_REPORT.md | Yes | Yes | Yes |
| HANDOFF.md | Yes | Yes | Yes |
| ROADMAP.md | Yes | Yes | Yes |
| CONTRIBUTING.md | Yes | Yes | Yes |
| SECURITY.md | Yes | Yes | Yes |

---

## Related Documents

- evaluation-plan.md — Evaluation plan
- demo-plan.md — Demo script
- judge-questions.md — Judge questions
- limitations.md — Limitations
- risk-register.md — Risk register