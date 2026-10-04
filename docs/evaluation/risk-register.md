# Risk Register

**Status**: SPECIFIED

---

## Risk Categories

| ID | Category | Risk | Likelihood | Impact | Mitigation | Owner | Status |
|----|----------|------|------------|--------|------------|-------|--------|
| R1 | Technical | Parser bugs crash demo | Medium | High | Extensive fuzzing, golden tests, panic hooks | Lang Lead | Open |
| R2 | Technical | Collector mock fails | Low | High | Contract tests, integration tests, real collectors early | Collector Lead | Open |
| R3 | Technical | Correlation misses edges | Medium | Medium | Property tests, golden graphs, rule validation | Corr Lead | Open |
| R4 | Technical | Verification false positive | Low | High | Tamper tests (positive + negative), golden chains | Evidence Lead | Open |
| R5 | Technical | Export format invalid | Medium | Medium | Schema validation, round-trip tests, validator tools | Export Lead | Open |
| R6 | Schedule | Demo time exceeds limit | Medium | Medium | Rehearse, optimize slow paths, cut optional steps | PM | Open |
| R7 | Schedule | Feature creep delays MVP | High | High | Strict scope guard, weekly scope review | PM | Open |
| R8 | Technical | Windows API differences | Medium | High | Test on Win10/11, CI on Windows runner | Collector Lead | Open |
| R9 | Technical | Linux kernel compatibility | Medium | High | Test on Ubuntu 22.04/24.04, CI on Linux runner | Collector Lead | Open |
| R10 | Security | Evidence tampering undetected | Low | Critical | Multi-layer verification, tamper tests | Evidence Lead | Open |
| R11 | Security | Collector key compromise | Low | Critical | Embedded keys, supply chain security, capability bounding | Security Lead | Open |
| R12 | Scope | Feature creep from judges | Medium | Medium | Locked scope document, change control board | PM | Open |
| R13 | Resource | Team member unavailable | Low | High | Cross-training, documentation, bus factor > 1 | PM | Open |
| R14 | External | Velociraptor/KAPE API changes | Low | Medium | Version pinning, adapter pattern, tests | Arch Lead | Open |
| R15 | Demo | Live demo fails | Medium | High | Pre-recorded video, screenshots, pre-generated outputs | PM | Open |
| R16 | Technical | SQLite locking issues | Low | Medium | WAL mode, connection pooling, retry logic | Core Lead | Open |
| R17 | Technical | Memory usage exceeds limit | Medium | High | Streaming evidence, pagination, graph pruning | Runtime Lead | Open |
| R18 | Documentation | Inconsistent terminology | Medium | Medium | Glossary, review checklist, automated checks | Doc Lead | Open |
| R19 | Schedule | Research not verified | Medium | Medium | Source verification checklist, peer review | Research Lead | Open |
| R20 | Demo | Judge asks unanticipated question | Medium | Medium | Prep document, role-play, honest "we don't know" | PM | Open |

---

## Risk Matrix

```
Impact
  ^
  |     R10, R11 (Critical)
  |           *
  |     R1, R2, R4, R17 (High)
  |     *   *   *
  |     R3, R5, R8, R9, R16 (Medium)
  |   *   *   *   *   *
  |     R6, R7, R12, R14, R18, R19, R20 (Medium)
  | *   *   *   *   *   *   *
  |     R11, R13, R15 (Low/Medium)
  |   *   *   *
  +-------------------------------------> Likelihood
    Low    Medium    High
```

---

## Top 5 Risks by Score (Likelihood × Impact)

| Rank | Risk | Score | Action |
|------|------|-------|--------|
| 1 | R7: Feature creep delays MVP | High × High = 9 | Strict scope lock, weekly review |
| 2 | R1: Parser bugs crash demo | Medium × High = 6 | Fuzzing, golden tests, panic hooks |
| 3 | R2: Collector mock fails | Low × High = 3 | Contract tests, real collectors early |
| 4 | R6: Demo time exceeds limit | Medium × Medium = 4 | Rehearse, optimize, cut optional |
| 5 | R15: Live demo fails | Medium × High = 6 | Pre-recorded video, screenshots |

---

## Mitigation Tracking

| Risk | Mitigation | Due Date | Status |
|------|------------|----------|--------|
| R1 | Pest fuzzing harness + golden tests | Phase 1 Week 1 | Planned |
| R2 | Collector contract test harness | Phase 4 Week 1 | Planned |
| R4 | Tamper test suite (positive/negative) | Phase 7 Week 1 | Planned |
| R7 | Scope lock document + weekly review | Phase 0 Done | Complete |
| R15 | Pre-recorded demo video | Phase 8 Week 1 | Planned |

---

## Related Documents

- `evaluation-plan.md` — Evaluation plan
- `success-criteria.md` — Success criteria
- `demo-plan.md` — Demo script
- `judge-questions.md` — Judge questions
- `limitations.md` — Limitations