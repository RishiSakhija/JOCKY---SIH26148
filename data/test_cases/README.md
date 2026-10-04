# Test Cases

**Status**: PLACEHOLDER — Test infrastructure not yet implemented

---

## Purpose

This directory defines the **test case specifications** for JOCKY. Actual test implementations live in each crate's `tests/` directory or as `#[test]` functions.

---

## Test Categories

| Category | Description | Location |
|----------|-------------|----------|
| Unit tests | Per-crate internal logic | `crates/*/src/**/*.rs` |
| Integration tests | Cross-crate pipelines | `crates/*/tests/*.rs` |
| Contract tests | Collector trait compliance | `tests/contract/` |
| Golden tests | Serialization/parsing roundtrips | `tests/golden/` |
| Property tests | Proptest-based invariants | `tests/proptest/` |
| E2E tests | Full investigation runs | `tests/e2e/` |

---

## Test Case Specification Format

Each test case is defined as a TOML file:

```toml
# test_cases/integration/basic_investigation.toml
name = "basic_investigation"
description = "End-to-end: script → IR → contract → execution → graph → export"

[input]
script = """
investigation "test" {
    collect process as procs where name == "init"
    correlate procs -> procs by "pid_link" as tree
    export case "test" format case_uco
}
"""
target = { type = "dead", identifier = "sample_case/", os = "linux" }

[expected]
steps = 2
evidence_min = 1
correlations_min = 1
exports = ["case_uco"]
verification = "pass"
```

---

## Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p jockey-language

# Integration tests
cargo test --test integration

# With output
cargo test -- --nocapture
```

---

## CI Integration

Tests run in GitHub Actions on:
- Ubuntu latest (Linux collectors)
- Windows latest (Windows collectors)
- macOS latest (cross-platform validation)

---

## Test Data

Test data is sourced from:
- `data/samples/` (synthetic, committed)
- Generated at runtime (deterministic)
- Downloaded CI artifacts (large datasets)

No production/case data in tests.