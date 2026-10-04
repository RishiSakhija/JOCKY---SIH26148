# Run and Evaluate Guide

**Status**: SPECIFIED

---

## Purpose

Instructions for building, running, and evaluating JOCKY at each phase.

---

## Prerequisites

### System Requirements
- **Rust**: 1.78+ (managed by `rust-toolchain.toml`)
- **OS**: Windows 10/11, Ubuntu 22.04/24.04, macOS 13+
- **Memory**: 8 GB+ RAM (for compilation)
- **Disk**: 10 GB+ free space

### Windows-Specific
- Visual Studio 2022 Build Tools (for `windows-sys` crate)
- Windows 10 SDK (10.0.19041+)

### Linux-Specific
- `build-essential`, `pkg-config`
- `libssl-dev`, `libclang-dev` (for some dependencies)

---

## Build Commands

### Standard Build
```bash
# Debug build (default)
cargo build --workspace

# Release build (optimized)
cargo build --workspace --release

# Specific crate
cargo build -p jockey-language
cargo build -p jockey-core --release
```

### Cross-Compilation (Future)
```bash
# Windows from Linux
cargo build --workspace --release --target x86_64-pc-windows-msvc

# Linux from Windows (via WSL)
cargo build --workspace --release --target x86_64-unknown-linux-gnu
```

---

## Run Commands

### Phase 1+ — CLI Tool
```bash
# Run investigation script
cargo run --bin jockey -- run investigate.jky --target live --host WORKSTATION-01 --os windows

# Compile to backends
cargo run --bin jockey -- compile investigate.jky --output-dir ./out

# Show graph
cargo run --bin jockey -- graph show --run-id RUN_ID --format cytoscape

# Verify integrity
cargo run --bin jockey -- verify --run-id RUN_ID

# Export
cargo run --bin jockey -- export --run-id RUN_ID --format case_uco

# Load sample case
cargo run --bin jockey -- load sample_case/

# Generate narrative report
cargo run --bin jockey -- narrative --run-id RUN_ID

# Start API server
cargo run --bin jockey -- serve --addr 0.0.0.0:8080 --evidence-root ./evidence
```

### Release Binary
```bash
# Build release
cargo build --workspace --release

# Run from target/release/
./target/release/jockey run investigate.jky ...
./target/release/jockey serve --addr 0.0.0.0:8080
```

---

## API Server

### Start Server
```bash
# Debug
cargo run --bin jockey -- serve --addr 0.0.0.0:8080 --evidence-root ./evidence

# Release
./target/release/jockey serve --addr 0.0.0.0:8080 --evidence-root ./evidence

# With config file
./target/release/jockey serve --config jockey.toml
```

### API Endpoints
```bash
# Create investigation
curl -X POST http://localhost:8080/api/v1/investigations \
  -H "Content-Type: application/json" \
  -d '{"script": "investigation \"test\" { collect process as p }", "target": {"type": "live", "identifier": "localhost", "os": "linux"}}'

# Get investigation status
curl http://localhost:8080/api/v1/investigations/{id}

# Get graph (Cytoscape JSON)
curl http://localhost:8080/api/v1/investigations/{id}/graph

# Get timeline
curl http://localhost:8080/api/v1/investigations/{id}/timeline

# List evidence
curl http://localhost:8080/api/v1/investigations/{id}/evidence

# Get evidence with receipt/provenance
curl http://localhost:8080/api/v1/investigations/{id}/evidence/{eid}

# Trigger verification
curl -X POST http://localhost:8080/api/v1/investigations/{id}/verify

# Export
curl http://localhost:8080/api/v1/investigations/{id}/export/case_uco
```

### WebSocket (Live Updates)
```javascript
const ws = new WebSocket("ws://localhost:8080/api/v1/investigations/{id}/stream");
ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    console.log(msg.type, msg.data);
};
```

---

## Frontend Dashboard

### Access
```bash
# Start server (serves frontend at /)
cargo run --bin jockey -- serve --addr 0.0.0.0:8080

# Open browser
open http://localhost:8080
# or
xdg-open http://localhost:8080
```

### Pages
| URL | Page |
|-----|------|
| `/` | Dashboard overview |
| `/investigation.html?id={id}` | Investigation detail |
| `/graph.html?id={id}` | Graph view |
| `/timeline.html?id={id}` | Timeline view |
| `/evidence.html?id={id}` | Evidence browser |
| `/findings.html?id={id}` | Findings board |
| `/verify.html?id={id}` | Verification report |
| `/report.html?id={id}` | Generated report |

---

## Testing

### Unit Tests
```bash
# All crates
cargo test --workspace --lib

# Specific crate
cargo test -p jockey-language
cargo test -p jockey-core
cargo test -p jockey-evidence
```

### Integration Tests
```bash
# All integration tests
cargo test --workspace --test integration

# Specific test
cargo test --test integration full_pipeline
```

### Property Tests
```bash
cargo test --workspace --test proptest
```

### Golden Tests
```bash
# Run (fails if output differs)
cargo test --test golden

# Update golden files (after intentional changes)
cargo test --test golden -- --update
```

### Fuzzing (Optional)
```bash
cargo install cargo-fuzz
cargo fuzz run parser_fuzz
```

### Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```

---

## Demo Execution

### Prerequisites
```bash
# 1. Build release
cargo build --workspace --release

# 2. Generate demo sample case
cargo run --bin generate_test_data -- --case apt29 --output sample_case/

# 3. Start API server (terminal 1)
./target/release/jockey serve --addr 0.0.0.0:8080 --evidence-root ./evidence &
```

### Demo Script (Run in Terminal 2)
```bash
# 1. Load sample
./target/release/jockey load sample_case/

# 2. Show graph
./target/release/jockey graph show

# 3. Run DSL
./target/release/jockey run investigate.jky

# 4. Compile
./target/release/jockey compile investigate.jky

# 5. Export narrative
./target/release/jockey narrative

# 6. Verify integrity
./target/release/jockey verify

# 7. Tamper demo
echo "TAMPER" >> evidence/01ARZ3NDEKTSV4RRFFQ69G5FB0.jsonl
./target/release/jockey verify

# 8. Restore and verify
git checkout evidence/01ARZ3NDEKTSV4RRFFQ69G5FB0.jsonl
./target/release/jockey verify
```

### Dashboard Demo
```bash
# Open browser
open http://localhost:8080/investigation.html?id=RUN_ID

# Navigate tabs: Graph, Timeline, Evidence, Findings, Verify, Export
```

---

## Evaluation Checklist

### Functional
- [ ] All 10 DSL statements parse and execute
- [ ] Type checker infers capabilities correctly
- [ ] IR lowering produces valid execution plan
- [ ] Policy engine enforces all 7 rules
- [ ] Runtime executes steps with correct parallelism
- [ ] All 10 collectors produce valid evidence
- [ ] Receipts sign/verify correctly
- [ ] Provenance chains build and verify
- [ ] All 9 correlation rules produce edges
- [ ] Graph queries return expected results
- [ ] Hypothesis overlay works
- [ ] Verification detects tampering
- [ ] All 4 export formats valid
- [ ] CLI commands work end-to-end
- [ ] API endpoints respond correctly
- [ ] Frontend loads and interacts

### Performance
- [ ] Collection throughput > 1000 events/sec
- [ ] Correlation latency < 100ms for 1000 evidence
- [ ] Verification < 30s for 1000 evidence
- [ ] Graph query < 50ms for 5000 nodes
- [ ] Export generation < 10s
- [ ] Memory < 512 MB
- [ ] Binary size < 50 MB

### Security
- [ ] No secrets in code
- [ ] No unsafe without justification
- [ ] Input validation on all APIs
- [ ] Capability enforcement works
- [ ] Evidence root isolation works
- [ ] Tamper detection works

### Demo
- [ ] All 8 demo steps work
- [ ] Total time < 8 minutes
- [ ] Tamper demo shows FAIL -> PASS
- [ ] Dashboard shows all tabs
- [ ] Exports open in standard tools

---

## CI/CD Validation

```bash
# Run all CI checks locally
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps --document-private-items
cargo audit --workspace
cargo deny check licenses
```

---

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| `link.exe` not found (Windows) | Install Visual Studio Build Tools + Windows SDK |
| `clang` not found (Linux) | `apt install libclang-dev` |
| `cargo build` fails on `windows-sys` | Ensure Windows 10 SDK installed |
| `cargo test` hangs | Check for deadlocks in async code; increase timeout |
| `cargo audit` finds vulnerabilities | Update dependencies in `Cargo.toml` |
| Binary too large | Enable LTO, strip symbols: `cargo build --release --target=x86_64-pc-windows-msvc -Z trim-paths` |

### Debug Logging
```bash
# Enable debug logging
RUST_LOG=debug ./target/release/jockey run investigate.jky

# Specific crate
RUST_LOG=jockey_runtime=debug,jockey_collectors=trace ./target/release/jockey run investigate.jky
```

---

## Related Documents

- `DATASETS.md` — Dataset details
- `docs/evaluation/evaluation-plan.md` — Test plan
- `docs/evaluation/success-criteria.md` — Success criteria
- `docs/evaluation/demo-plan.md` — Demo script
- `HANDOFF.md` — Implementation guide
- `ROADMAP.md` — Phase timeline