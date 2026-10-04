# Evaluation Plan

**Status**: SPECIFIED

---

## Purpose

Define how JOCKY will be evaluated for correctness, performance, and SIH demo readiness.

---

## Evaluation Dimensions

| Dimension | Metrics | Target |
|-----------|---------|--------|
| **Correctness** | Unit test coverage, integration test pass rate, property test pass rate | >= 90% coverage, 100% critical path |
| **Completeness** | MVP feature coverage (spec vs implementation) | 100% of MVP spec |
| **Performance** | Collection throughput, correlation latency, verification time | < 30s for 1000 evidence |
| **Security** | Audit findings, capability enforcement, tamper detection | 0 critical, 0 high |
| **Usability** | Demo script success rate, judge comprehension | 100% demo steps pass |
| **Interoperability** | Export format validation (CASE/UCO, Timesketch, Cytoscape) | Schema valid, round-trip |

---

## Test Categories

### 1. Unit Tests (Per Crate)

| Crate | Focus Areas | Tools |
|-------|-------------|-------|
| `jockey-language` | Parser roundtrip, AST construction, type checking, capability inference | `cargo test`, `proptest` |
| `jockey-core` | IR construction, contract building, policy validation, storage | `cargo test`, `proptest` |
| `jockey-runtime` | Dispatcher, timeout/retry, evidence root isolation, collector execution | `cargo test`, mock collectors |
| `jockey-evidence` | Evidence/receipt/provenance serialization, hashing, signing, verification | `cargo test`, golden files |
| `jockey-collectors` | Trait compliance, Windows/Linux mock execution, output format | `cargo test`, contract tests |
| `jockey-correlation` | Rule matching, graph construction, query accuracy | `cargo test`, golden graphs |
| `jockey-api` | REST endpoints, WebSocket, OpenAPI spec compliance | `cargo test`, `reqwest` |

### 2. Integration Tests

| Test | Description | Success Criteria |
|------|-------------|------------------|
| `full_pipeline` | Script -> AST -> IR -> Contract -> Runtime -> Evidence -> Correlation -> Graph -> Verify -> Export | All stages pass, outputs valid |
| `cross_platform` | Same script on Windows + Linux targets produces equivalent evidence types | Evidence types match, correlations equivalent |
| `verification_tamper` | Modify evidence file -> verify detects FAIL -> restore -> verify PASS | Tamper detected, restoration works |
| `compilation_targets` | Script compiles to VQL, SQL, KAPE, Volatility, CASE, Cytoscape, Timesketch | All outputs syntactically valid |
| `hypothesis_tracking` | Bind hypothesis -> correlate -> graph -> confidence update | Confidence reflects evidence |

### 3. Property Tests

| Property | Description |
|----------|-------------|
| `parser_roundtrip` | `parse(format(ast)) == ast` for all valid ASTs |
| `hash_determinism` | `blake3(bytes)` always produces same output |
| `signature_verification` | Valid signature always verifies; invalid always fails |
| `provenance_chain` | Chain verification passes for unmodified chains; fails for any mutation |
| `graph_queries` | `path(a,b)` == reverse of `path(b,a)` for undirected relations |

### 4. Fuzzing

| Target | Strategy |
|--------|----------|
| Parser | Random valid/invalid JOCKY scripts |
| JSON deserialization | Malformed evidence/receipt/provenance JSON |
| Parameter validation | Random collector parameters |

### 5. Contract Tests

| Contract | Implementation |
|----------|----------------|
| Collector trait | All collectors implement `execute` with correct signature |
| Parameter schemas | All collector actions validate against declared JSON Schema |
| Evidence format | All evidence objects serialize/deserialize correctly |

---

## Test Data

### Synthetic Data

- Generated via `scripts/generate_test_data.rs` (to be implemented)
- Deterministic seeds for reproducibility
- Covers all 7 evidence types with known correlations

### Golden Files

- `tests/golden/` — Expected outputs for parser, serializer, graph, exports
- Updated via `cargo test --test golden -- --update`

### Public Domain Datasets (CI Only)

| Dataset | Source | Use Case |
|---------|--------|----------|
| M57 Patents | NIST CFReDS | Full investigation replay |
| NIST Hacking Case | NIST CFReDS | Attack reconstruction |
| DFIR Training Samples | Various | Component testing |

---

## CI/CD Integration

```yaml
# .github/workflows/ci.yml (excerpt)
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Unit tests
        run: cargo test --workspace --lib
      - name: Integration tests
        run: cargo test --workspace --test integration
      - name: Property tests
        run: cargo test --workspace --test proptest
      - name: Golden tests
        run: cargo test --workspace --test golden
```

---

## Performance Benchmarks

| Benchmark | Target | Measurement |
|-----------|--------|-------------|
| **Collection throughput** | > 1000 events/sec | `proc_windows list` on 10k processes |
| **Correlation latency** | < 100ms for 1000 evidence | 9 rules, full cross-product |
| **Verification time** | < 30s for 1000 evidence | Full receipt + provenance check |
| **Graph query** | < 50ms for 5000 nodes | `path()`, `backward_from()` |
| **Export generation** | < 10s for full case | CASE/UCO + Cytoscape + Timesketch |
| **Memory usage** | < 512 MB | Full investigation in memory |

---

## SIH Demo Evaluation

### Demo Script Validation

| Step | Command | Expected Outcome |
|------|---------|------------------|
| 1 | `jockey load sample_case/` | Loads 47 synthetic events |
| 2 | `jockey graph show` | Displays causal chain DOT |
| 3 | `jockey run investigate.jky` | Completes with 3 findings, 12 correlations |
| 4 | `jockey compile investigate.jky` | Generates hunt.vql, kape_targets.json, evidence.case.json |
| 5 | `jockey narrative` | Produces evidence_report.md with citations |
| 6 | `jockey verify` | PASS: all receipts valid, chain intact |
| 7 | Tamper + `jockey verify` | FAIL: hash mismatch detected |
| 8 | Restore + `jockey verify` | PASS: integrity restored |

### Demo Success Criteria

- [ ] All 8 steps complete without errors
- [ ] Total demo time < 8 minutes
- [ ] Judge can follow causal chain in graph
- [ ] Tamper detection visibly works (FAIL -> PASS)
- [ ] Exports open in standard tools (CASE/UCO viewer, Cytoscape, Timesketch)
- [ ] No crashes, panics, or unhandled errors

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Parser bugs crash demo | Medium | High | Extensive fuzzing, golden tests |
| Collector mock fails | Low | High | Contract tests, integration tests |
| Correlation misses edge | Medium | Medium | Property tests, golden graphs |
| Verification false positive | Low | High | Tamper tests (positive + negative) |
| Export format invalid | Medium | Medium | Schema validation, round-trip tests |
| Demo time exceeds limit | Medium | Medium | Rehearse, optimize slow paths |

---

## Related Documents

- `success-criteria.md` — Detailed success criteria
- `demo-plan.md` — Demo script and flow
- `judge-questions.md` — Anticipated judge questions
- `limitations.md` — Known limitations
- `risk-register.md` — Risk register