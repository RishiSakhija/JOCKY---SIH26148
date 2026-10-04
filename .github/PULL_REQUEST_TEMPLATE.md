# Pull Request Template

## Description
Brief description of what this PR does.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Refactoring
- [ ] Performance improvement
- [ ] Test addition
- [ ] CI/Build change

## Related Issues
Closes #(issue number)

## Changes Made
- [ ] List key changes
- [ ] Reference specific files/modules

## Testing
- [ ] Unit tests pass (`cargo test`)
- [ ] Integration tests pass (if applicable)
- [ ] Added new tests for new functionality
- [ ] Manual testing performed (describe)

## Code Quality
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] No `unwrap()`/`expect()`/panic in production code paths
- [ ] Error handling uses `thiserror`/`anyhow` appropriately

## Documentation
- [ ] Updated relevant documentation in `docs/`
- [ ] Updated README if user-facing change
- [ ] Updated CHANGELOG.md
- [ ] Added/updated code comments

## Security
- [ ] No secrets, keys, or credentials in code
- [ ] No unsafe code without justification
- [ ] Input validation on all external interfaces
- [ ] Follows SECURITY.md guidelines

## Breaking Changes
If this introduces breaking changes, describe:
1. What breaks
2. Migration path
3. Version bump needed (major/minor/patch)

## Checklist
- [ ] Self-review completed
- [ ] Code follows JOCKY architecture (see SPEC_SHEET.md)
- [ ] Terminology consistent with JOCKY_SPEC_SHEET.md
- [ ] No unsupported claims ("first ever", "court admissible", "100% secure", etc.)