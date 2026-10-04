# Contributing to JOCKY

Thank you for your interest in contributing to JOCKY! This document outlines the process and standards for contributions.

---

## Code of Conduct

This project follows the **Rust Community Code of Conduct**. By participating, you agree to uphold this code. Please report unacceptable behavior to the project maintainers.

---

## Getting Started

### Prerequisites
- Rust 1.78+ (see `rust-toolchain.toml`)
- Git
- Familiarity with Rust async programming (tokio)

### Development Setup
```bash
# Clone repository
git clone https://github.com/RishiSakhija/JOKEY---SIH26148.git
cd JOKEY---SIH26148

# Install Rust toolchain (via rustup)
rustup toolchain install stable --component rustfmt,clippy

# Verify setup
cargo fmt --all --check
cargo check --workspace
cargo test --workspace --lib
```

---

## Contribution Workflow

### 1. Find or Create an Issue
- Check existing issues for bugs, features, or documentation improvements
- Create a new issue if needed (use templates)

### 2. Fork and Branch
```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/JOKEY---SIH26148.git
cd JOKEY---SIH26148

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make Changes
- Follow coding standards (see below)
- Write tests for new functionality
- Update documentation as needed
- Keep commits atomic and focused

### 4. Validate Changes
```bash
# Format check
cargo fmt --all --check

# Compilation check
cargo check --workspace --all-targets

# Linting
cargo clippy --workspace --all-targets -- -D warnings

# Tests
cargo test --workspace --all-targets

# Documentation
cargo doc --workspace --no-deps --document-private-items
```

### 5. Commit and Push
```bash
# Stage changes
git add .

# Commit with conventional commit message
git commit -m "feat: add new collector capability"
# or
git commit -m "fix: handle edge case in parser"
# or
git commit -m "docs: update architecture diagram"

# Push to your fork
git push origin feature/your-feature-name
```

### 6. Create Pull Request
- Use the PR template
- Link related issues
- Ensure CI passes
- Request review from maintainers

---

## Coding Standards

### Rust Style
- **Format**: `cargo fmt --all` (enforced by CI)
- **Linting**: `cargo clippy -- -D warnings` (enforced by CI)
- **Edition**: 2021
- **Async**: Use `tokio` — no blocking in async contexts

### Error Handling
```rust
// ✅ Good: Proper error propagation
fn process_evidence(data: &[u8]) -> Result<Evidence, EvidenceError> {
    let hash = blake3::hash(data);
    let receipt = sign_receipt(hash)?;
    Ok(Evidence::new(data, hash, receipt))
}

// ❌ Bad: Panic in production code
fn process_evidence(data: &[u8]) -> Evidence {
    let hash = blake3::hash(data);
    let receipt = sign_receipt(hash).unwrap(); // PANIC!
    Evidence::new(data, hash, receipt)
}
```

### Unsafe Code
- **Avoid** `unsafe` unless absolutely necessary
- If required: Document invariants in comments
- **Audit required** for any new `unsafe` blocks
- Prefer safe abstractions (`nix`, `windows-sys` with safe wrappers)

### Error Types
- Use `thiserror` for library errors
- Use `anyhow` for application errors
- Define specific error variants (not `Box<dyn Error>`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum CollectorError {
    #[error("collector not found: {0}")]
    NotFound(String),
    
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("timeout after {0}ms")]
    Timeout(u64),
    
    #[error("internal error: {0}")]
    Internal(String),
}
```

### Async Code
- Use `tokio` for async runtime
- No blocking calls in async functions (use `tokio::task::spawn_blocking`)
- Prefer `async fn` over `fn() -> impl Future`
- Handle cancellation gracefully

### Testing
- **Unit tests**: In `src/**/*.rs` as `#[cfg(test)] mod tests`
- **Integration tests**: In `tests/**/*.rs`
- **Property tests**: Use `proptest` for invariants
- **Golden tests**: In `tests/golden/` for serialization/parsing
- **Contract tests**: For trait compliance (collectors)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_evidence_serialization_roundtrip() {
        let evidence = create_test_evidence();
        let json = serde_json::to_string(&evidence).unwrap();
        let decoded: Evidence = serde_json::from_str(&json).unwrap();
        assert_eq!(evidence, decoded);
    }
    
    proptest! {
        #[test]
        fn hash_determinism(bytes in any::<Vec<u8>>()) {
            let h1 = blake3::hash(&bytes);
            let h2 = blake3::hash(&bytes);
            prop_assert_eq!(h1, h2);
        }
    }
}
```

---

## Documentation Standards

### Code Documentation
- All public items must have `///` doc comments
- Include examples for non-trivial functions
- Document invariants and preconditions

### Architecture Documentation
- Update `docs/architecture/` for architectural changes
- Update `assets/diagrams/*.mmd` for diagram changes
- Keep `JOCKY_SPEC_SHEET.md` as single source of truth

### Changelog
- Update `CHANGELOG.md` for every PR
- Follow [Keep a Changelog](https://keepachangelog.com/) format

---

## Pull Request Checklist

Before submitting, ensure:

- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace --all-targets` passes
- [ ] `cargo doc --workspace --no-deps --document-private-items` builds
- [ ] New code has tests
- [ ] Documentation updated (code + architecture + changelog)
- [ ] No `unwrap()`/`expect()`/panic in production paths
- [ ] No new `unsafe` without documented invariants
- [ ] No secrets, keys, or credentials in code
- [ ] PR description explains *what* and *why*
- [ ] Related issues linked

---

## Review Process

1. **Automated checks** must pass (CI)
2. **Maintainer review** for architecture/design
3. **Code review** for correctness/style
4. **Security review** for sensitive changes
5. **Documentation review** for public-facing changes
6. **Approval** from at least one maintainer
5. **Merge** (squash and merge preferred)

---

## Release Process

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml` (workspace)
3. Create release PR
5. Merge to `main`
6. Tag release: `git tag v0.x.0`
7. GitHub Actions builds and publishes release
8. Update `CHANGELOG.md` for next version

---

## Security

See [SECURITY.md](SECURITY.md) for security policy and vulnerability reporting.

---

## Questions?

- Open a discussion on GitHub
- Check existing issues/PRs
- Read `HANDOFF.md` for implementation context

---

Thank you for contributing to JOCKY! 🦀