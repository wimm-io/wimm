# Test Coverage Documentation

This document explains the test coverage setup for the Wimm task manager project and how to maintain and improve coverage.

## Overview

We maintain a **minimum test coverage threshold of 60%** across the codebase to ensure code quality and reliability. Our current coverage is **~63%** with comprehensive unit and integration tests.

## Quick Start

```bash
# Check coverage with detailed report
make coverage-open

# Run coverage check with threshold enforcement
./scripts/check-coverage.sh

# Run full development workflow (tests + lint + coverage)
make dev
```

## Coverage Tools

### 1. Cargo Tarpaulin

We use [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) for coverage analysis:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Basic coverage
cargo tarpaulin

# With configuration file
cargo tarpaulin --config tarpaulin.toml
```

### 2. Coverage Script

The `scripts/check-coverage.sh` script provides enhanced coverage checking:

```bash
# Basic usage
./scripts/check-coverage.sh

# With custom threshold
./scripts/check-coverage.sh --threshold 70

# Generate and open HTML report
./scripts/check-coverage.sh --open

# Clean run with minimal output
./scripts/check-coverage.sh --clean --quiet
```

### 3. Makefile Commands

```bash
make coverage          # Basic coverage check
make coverage-html     # Generate HTML report
make coverage-open     # Generate and open HTML report
make coverage-clean    # Clean coverage artifacts
```

## Configuration

### Tarpaulin Configuration

Coverage settings are defined in `tarpaulin.toml`:

- **Threshold**: 60% minimum coverage
- **Exclusions**: `src/main.rs`, test files, examples
- **Formats**: HTML, XML, JSON reports
- **Timeout**: 120 seconds for tests

### GitHub Actions

The CI workflow (`.github/workflows/rust.yml`) includes:

- **Coverage Enforcement**: Fails if below 60%
- **PR Comments**: Automatic coverage reports on pull requests
- **Artifact Upload**: Coverage reports saved as CI artifacts
- **Multiple Integrations**: Codecov, Coveralls support

### Pre-commit Hooks

Coverage checks run automatically before commits:

```bash
# Install pre-commit
pip install pre-commit
pre-commit install

# Manual run
pre-commit run --all-files
```

## Current Coverage Breakdown

| Module | Coverage | Lines Covered | Status |
|--------|----------|---------------|---------|
| `storage/` | 92.30% | 36/38 | ✅ Excellent |
| `types.rs` | 100% | 7/7 | ✅ Complete |
| `time_tracking/` | 100% | 14/14 | ✅ Complete |
| `input/` | 100% | 4/4 | ✅ Complete |
| `ui/events.rs` | 84.71% | 72/85 | ✅ Good |
| `ui/help_panel.rs` | 100% | 85/85 | ✅ Complete |
| `ui/layout.rs` | 100% | 20/20 | ✅ Complete |
| `ui/app.rs` | 73.36% | 168/229 | ⚠️ Needs improvement |
| `ui/mod.rs` | 15.03% | 29/193 | ❌ Low coverage |
| `main.rs` | 0% | 0/20 | ⚠️ Excluded |

## Test Types

### Unit Tests

Located in each module with `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

**Coverage**: Individual functions and methods

### Integration Tests

Located in `tests/` directory:

```rust
// tests/integration_tests.rs
use wimm::*;

#[test]
fn test_full_workflow() {
    // End-to-end testing
}
```

**Coverage**: Cross-module interactions and workflows

## Improving Coverage

### 1. Identify Uncovered Code

```bash
# Generate HTML report
make coverage-open

# Look for red highlighted lines
# Focus on:
# - Error handling paths
# - Edge cases
# - Complex conditional logic
```

### 2. Add Missing Tests

**Priority Areas:**
1. **Error Handling**: Test failure scenarios
2. **Edge Cases**: Boundary conditions, empty inputs
3. **Integration**: Module interactions
4. **User Workflows**: Complete user journeys

**Example Test Addition:**
```rust
#[test]
fn test_error_handling() {
    let mut storage = MemoryStorage::new(HashMap::new());

    // Test error case
    let result = storage.delete_task("nonexistent");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DbError::NotFound(_)));
}
```

### 3. Test Coverage Guidelines

**Good Test Coverage Includes:**
- ✅ Happy path scenarios
- ✅ Error conditions and edge cases
- ✅ Boundary value testing
- ✅ State transitions
- ✅ Integration between modules
- ✅ User interaction flows

**What to Test:**
```rust
// ✅ Test all public functions
#[test] fn test_public_api() { /* ... */ }

// ✅ Test error conditions
#[test] fn test_invalid_input() { /* ... */ }

// ✅ Test edge cases
#[test] fn test_empty_data() { /* ... */ }
#[test] fn test_large_data() { /* ... */ }

// ✅ Test state changes
#[test] fn test_state_transition() { /* ... */ }
```

## Coverage in CI/CD

### Pull Requests

1. **Automatic Coverage Check**: Runs on every PR
2. **Threshold Enforcement**: PR fails if coverage drops below 60%
3. **Coverage Comments**: Bot posts coverage report on PR
4. **Diff Coverage**: Shows coverage impact of changes

### Branch Protection

Configure branch protection rules:

```yaml
# .github/branch-protection.yml
protection_rules:
  main:
    required_status_checks:
      - "test-coverage"
    enforce_admins: true
```

### Coverage Badges

Add to README.md:

```markdown
[![Coverage Status](https://codecov.io/gh/username/wimm/branch/main/graph/badge.svg)](https://codecov.io/gh/username/wimm)
```

## Troubleshooting

### Common Issues

**1. Coverage Below Threshold**
```bash
# Check what's missing
make coverage-open

# Focus on red highlighted lines
# Add tests for uncovered paths
```

**2. Flaky Coverage Numbers**
```bash
# Clean and re-run
make coverage-clean
make coverage
```

**3. Tests Timeout**
```bash
# Increase timeout in tarpaulin.toml
timeout = 180  # 3 minutes
```

**4. Excluded Files Not Working**
```bash
# Check tarpaulin.toml exclude patterns
exclude-files = ["src/main.rs", "tests/*"]
```

### Performance Issues

**Slow Coverage Runs:**
1. Use `--skip-clean` for incremental runs
2. Exclude large test files if needed
3. Run coverage only on changed files during development

**Memory Issues:**
1. Increase timeout for complex tests
2. Split large test suites
3. Use `--bin` to target specific binaries

## Best Practices

### 1. Write Tests First

```bash
# TDD workflow
cargo test          # Write failing test
# Implement feature
cargo test          # Verify test passes
make coverage       # Check coverage impact
```

### 2. Regular Coverage Monitoring

```bash
# Daily development workflow
make dev            # test + lint + coverage

# Before committing
pre-commit run      # Includes coverage check

# Before pushing
make ci             # Full CI simulation
```

### 3. Coverage-Driven Development

1. **Identify Low Coverage**: Use HTML report to find gaps
2. **Write Tests**: Focus on uncovered lines
3. **Verify Impact**: Re-run coverage to confirm improvement
4. **Iterate**: Repeat until threshold met

### 4. Code Review Checklist

- [ ] New code includes tests
- [ ] Coverage doesn't decrease
- [ ] Error paths are tested
- [ ] Integration scenarios covered
- [ ] Edge cases handled

## Maintenance

### Monthly Tasks

1. **Review Coverage Trends**: Track coverage over time
2. **Update Thresholds**: Consider increasing minimum coverage
3. **Audit Test Quality**: Ensure tests are meaningful
4. **Clean Technical Debt**: Improve low-coverage areas

### Tool Updates

```bash
# Update tarpaulin
cargo install cargo-tarpaulin --force

# Update pre-commit hooks
pre-commit autoupdate

# Update CI dependencies in .github/workflows/
```

## Resources

- [Cargo Tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Codecov Documentation](https://docs.codecov.io/)
- [Pre-commit Hooks](https://pre-commit.com/)

## Support

For questions about coverage setup:

1. Check this documentation
2. Review existing tests for examples
3. Run `make help` for available commands
4. Create an issue with the `coverage` label
