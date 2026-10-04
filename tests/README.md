# Tests

**Status**: SPECIFIED — Test infrastructure not yet implemented

---

## Purpose

Test organization and conventions for JOCKY.

---

## Test Structure

```
tests/
├── contract/           # Collector trait compliance tests
├── golden/             # Serialization roundtrip golden files
├── proptest/           # Property-based test definitions
├── e2e/                # End-to-end integration tests
├── fixtures/           # Shared test fixtures
└── README.md           # This file
```

---

## Test Conventions

### Naming
- Unit tests: `#[test]` in `src/**/*.rs`
- Integration tests: `tests/**/*.rs` with descriptive names
- Golden files: `tests/golden/<crate>_<type>.json`

### Running
```bash
# All tests
cargo test --workspace

# Specific test categories
cargo test --test contract
cargo test --test golden
cargo test --test e2e

# Property tests (may be slow)
cargo test --test proptest

# With output
cargo test -- --nocapture
```

### Golden File Updates
```bash
# Update golden files after intentional changes
cargo test --test golden -- --update
```

---

## Test Categories

| Category | Coverage | Tools |
|----------|----------|-------|
| Parser | Grammar, AST, error recovery | `pest`, `proptest` |
| Type Checker | Capability inference, scope, types | `proptest` |
| IR Compiler | Lowering, dependency resolution | Golden files |
| Contract Builder | Policy injection, validation | Contract tests |
| Runtime | Dispatch, timeout, retry, errors | Integration tests |
| Evidence | Serialization, hashing, receipts | Golden files |
| Provenance | Chain linking, signing, verification | Property tests |
| Collectors | Trait compliance, output format | Contract tests |
| Correlation | Rule matching, graph building | Golden files |
| Graph | Queries, path finding, subgraphs | Property tests |
| API | REST endpoints, WebSocket, auth | Integration tests |
| Verification | Tamper detection, false positives | Golden files |
| Export | CASE/UCO, Cytoscape, Timesketch | Golden files |

---

## CI Requirements

All tests must pass in CI:
- Ubuntu (Linux collectors)
- Windows (Windows collectors)
- macOS (cross-platform)

---

## Test Data

- Synthetic data generated at runtime (deterministic seeds)
- Golden files committed to `tests/golden/`
- Large fixtures downloaded as CI artifacts (not committed)

---

## Current State

Test infrastructure not yet implemented. This directory exists for future organization.