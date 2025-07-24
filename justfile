# Wimm Task Manager - Just Command Runner
# =======================================

# Configuration
cargo := "cargo"
min_coverage := "60"
coverage_script := "./scripts/check-coverage.sh"

# Colors for output
bold := '\033[1m'
green := '\033[32m'
yellow := '\033[33m'
red := '\033[31m'
blue := '\033[34m'
nc := '\033[0m'

# Default recipe
default: test lint coverage

# Display help
help:
    @echo "{{bold}}Wimm Task Manager - Development Commands{{nc}}"
    @echo "=========================================="
    @echo ""
    @echo "{{bold}}Building:{{nc}}"
    @echo "  build           Build the project in debug mode"
    @echo "  build-release   Build the project in release mode"
    @echo "  clean           Clean build artifacts"
    @echo ""
    @echo "{{bold}}Testing:{{nc}}"
    @echo "  test            Run all tests"
    @echo "  test-unit       Run unit tests only"
    @echo "  test-integration Run integration tests only"
    @echo "  test-watch      Run tests in watch mode"
    @echo ""
    @echo "{{bold}}Coverage:{{nc}}"
    @echo "  coverage        Generate coverage report (min: {{min_coverage}}%)"
    @echo "  coverage-html   Generate HTML coverage report"
    @echo "  coverage-open   Generate and open HTML coverage report"
    @echo "  coverage-clean  Clean coverage artifacts"
    @echo ""
    @echo "{{bold}}Code Quality:{{nc}}"
    @echo "  lint            Run all linting (fmt + clippy)"
    @echo "  fmt             Format code with rustfmt"
    @echo "  fmt-check       Check code formatting"
    @echo "  clippy          Run clippy lints"
    @echo "  clippy-fix      Fix clippy warnings automatically"
    @echo ""
    @echo "{{bold}}Security & Compatibility:{{nc}}"
    @echo "  audit           Run security audit"
    @echo "  msrv            Check minimum supported Rust version"
    @echo ""
    @echo "{{bold}}Development:{{nc}}"
    @echo "  install-tools   Install required development tools"
    @echo "  install-just    Check Just installation and show install instructions"
    @echo "  setup           Complete development environment setup"
    @echo "  dev             Full development check (test + lint + coverage)"
    @echo "  watch           Watch for changes and run tests"
    @echo "  run             Run the application"
    @echo ""
    @echo "{{bold}}Installation:{{nc}}"
    @echo "  If Just is not installed, run one of these first:"
    @echo "    brew install just     # macOS/Linux with Homebrew"
    @echo "    cargo install just    # Any platform with Rust"
    @echo ""
    @echo "{{bold}}Environment Variables:{{nc}}"
    @echo "  MIN_COVERAGE    Minimum coverage percentage (default: {{min_coverage}})"
    @echo ""

# Building
build:
    @echo "{{blue}}Building project...{{nc}}"
    {{cargo}} build

build-release:
    @echo "{{blue}}Building project (release mode)...{{nc}}"
    {{cargo}} build --release

clean:
    @echo "{{blue}}Cleaning build artifacts...{{nc}}"
    {{cargo}} clean
    @echo "{{blue}}Cleaning coverage reports...{{nc}}"
    rm -f cobertura.xml tarpaulin-report.html tarpaulin-report.json lcov.info
    rm -rf target/tarpaulin

# Testing
test:
    @echo "{{blue}}Running all tests...{{nc}}"
    {{cargo}} test --verbose

test-unit:
    @echo "{{blue}}Running unit tests...{{nc}}"
    {{cargo}} test --lib --verbose

test-integration:
    @echo "{{blue}}Running integration tests...{{nc}}"
    {{cargo}} test --test '*' --verbose

test-watch:
    @echo "{{blue}}Running tests in watch mode...{{nc}}"
    {{cargo}} watch -x test

# Coverage
coverage:
    @echo "{{blue}}Generating coverage report (minimum: {{min_coverage}}%)...{{nc}}"
    @if [ -x "{{coverage_script}}" ]; then \
        MIN_COVERAGE={{min_coverage}} {{coverage_script}} --quiet; \
    else \
        echo "{{yellow}}Coverage script not found, falling back to basic tarpaulin...{{nc}}"; \
        {{cargo}} tarpaulin --exclude-files "src/main.rs" --fail-under {{min_coverage}}; \
    fi

coverage-html:
    @echo "{{blue}}Generating HTML coverage report...{{nc}}"
    @if [ -x "{{coverage_script}}" ]; then \
        MIN_COVERAGE={{min_coverage}} {{coverage_script}} --quiet; \
    else \
        {{cargo}} tarpaulin --exclude-files "src/main.rs" --out Html; \
    fi

coverage-open:
    @echo "{{blue}}Generating and opening HTML coverage report...{{nc}}"
    @if [ -x "{{coverage_script}}" ]; then \
        MIN_COVERAGE={{min_coverage}} {{coverage_script}} --open; \
    else \
        {{cargo}} tarpaulin --exclude-files "src/main.rs" --out Html; \
        if [ -f "tarpaulin-report.html" ]; then \
            if command -v xdg-open >/dev/null 2>&1; then \
                xdg-open tarpaulin-report.html; \
            elif command -v open >/dev/null 2>&1; then \
                open tarpaulin-report.html; \
            else \
                echo "{{yellow}}HTML report generated: tarpaulin-report.html{{nc}}"; \
            fi; \
        fi; \
    fi

coverage-clean:
    @echo "{{blue}}Cleaning coverage artifacts...{{nc}}"
    rm -rf target/tarpaulin
    rm -f tarpaulin-report.html cobertura.xml tarpaulin-report.json

# Code quality
lint: fmt-check clippy

fmt:
    @echo "{{blue}}Formatting code...{{nc}}"
    {{cargo}} fmt

fmt-check:
    @echo "{{blue}}Checking code formatting...{{nc}}"
    {{cargo}} fmt --all -- --check

clippy:
    @echo "{{blue}}Running clippy lints...{{nc}}"
    {{cargo}} clippy --all-targets --all-features -- -D warnings

clippy-fix:
    @echo "{{blue}}Fixing clippy warnings...{{nc}}"
    {{cargo}} clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

# Security and compatibility
audit:
    @echo "{{blue}}Running security audit...{{nc}}"
    @if ! command -v cargo-audit >/dev/null 2>&1; then \
        echo "{{yellow}}Installing cargo-audit...{{nc}}"; \
        {{cargo}} install cargo-audit; \
    fi
    {{cargo}} audit

msrv:
    @echo "{{blue}}Checking minimum supported Rust version...{{nc}}"
    {{cargo}} +1.70 check --all-targets --all-features

# Development tools
install-tools:
    @echo "{{blue}}Installing development tools...{{nc}}"
    @echo "Installing cargo-tarpaulin..."
    {{cargo}} install cargo-tarpaulin
    @echo "Installing cargo-watch..."
    {{cargo}} install cargo-watch
    @echo "Installing cargo-audit..."
    {{cargo}} install cargo-audit
    @echo "{{green}}All tools installed successfully!{{nc}}"

# Check Just installation and show installation instructions
install-just:
    @echo "{{blue}}Checking Just command runner installation...{{nc}}"
    @if command -v just >/dev/null 2>&1; then \
        echo "{{green}}✅ Just is already installed!{{nc}}"; \
        just --version; \
    else \
        echo "{{red}}❌ Just is not installed{{nc}}"; \
        echo ""; \
        echo "{{bold}}To install Just, choose one of these methods:{{nc}}"; \
        echo ""; \
        echo "{{bold}}Option 1: Homebrew (macOS/Linux){{nc}}"; \
        echo "  brew install just"; \
        echo ""; \
        echo "{{bold}}Option 2: Cargo (any platform with Rust){{nc}}"; \
        echo "  cargo install just"; \
        echo ""; \
        echo "{{bold}}Option 3: Use migration script{{nc}}"; \
        echo "  ./scripts/migrate-to-just.sh"; \
        echo ""; \
        echo "{{yellow}}After installation, run: just install-tools{{nc}}"; \
        exit 1; \
    fi

# Complete development environment setup
setup: install-tools
    @echo "{{blue}}Setting up complete development environment...{{nc}}"
    @if command -v just >/dev/null 2>&1; then \
        echo "{{green}}✅ Development environment setup complete!{{nc}}"; \
        echo ""; \
        echo "{{bold}}Next steps:{{nc}}"; \
        echo "  just dev          # Run full development check"; \
        echo "  just test         # Run tests"; \
        echo "  just watch        # Start development with file watching"; \
        echo "  just help         # Show all available commands"; \
    else \
        echo "{{red}}❌ Just is required but not installed{{nc}}"; \
        echo "Run: {{blue}}just install-just{{nc}} for installation instructions"; \
        exit 1; \
    fi

# Development workflow
dev: test lint coverage
    @echo "{{green}}✅ All development checks passed!{{nc}}"

watch:
    @echo "{{blue}}Watching for changes...{{nc}}"
    @if ! command -v cargo-watch >/dev/null 2>&1; then \
        echo "{{yellow}}Installing cargo-watch...{{nc}}"; \
        {{cargo}} install cargo-watch; \
    fi
    {{cargo}} watch -x check -x test

run:
    @echo "{{blue}}Running application...{{nc}}"
    {{cargo}} run

# CI simulation
ci:
    @echo "{{blue}}Running CI simulation...{{nc}}"
    @echo "{{blue}}Step 1/4: Building...{{nc}}"
    just build
    @echo "{{blue}}Step 2/4: Testing...{{nc}}"
    just test
    @echo "{{blue}}Step 3/4: Linting...{{nc}}"
    just lint
    @echo "{{blue}}Step 4/4: Coverage...{{nc}}"
    just coverage
    @echo "{{green}}✅ CI simulation completed successfully!{{nc}}"

# Quick development cycle
quick: test clippy
    @echo "{{green}}✅ Quick checks passed!{{nc}}"

# Pre-commit hook simulation
pre-commit: fmt test clippy
    @echo "{{green}}✅ Pre-commit checks passed!{{nc}}"

# Release preparation
release-prep: clean build-release test lint coverage audit
    @echo "{{green}}✅ Release preparation completed!{{nc}}"

# Benchmarking (if needed in future)
bench:
    @echo "{{blue}}Running benchmarks...{{nc}}"
    {{cargo}} bench

# Documentation
docs:
    @echo "{{blue}}Generating documentation...{{nc}}"
    {{cargo}} doc --open

docs-check:
    @echo "{{blue}}Checking documentation...{{nc}}"
    {{cargo}} doc --no-deps

# Dependency management
update:
    @echo "{{blue}}Updating dependencies...{{nc}}"
    {{cargo}} update

outdated:
    @echo "{{blue}}Checking for outdated dependencies...{{nc}}"
    @if ! command -v cargo-outdated >/dev/null 2>&1; then \
        echo "{{yellow}}Installing cargo-outdated...{{nc}}"; \
        {{cargo}} install cargo-outdated; \
    fi
    {{cargo}} outdated

# Docker (if needed in future)
docker-build:
    @echo "{{blue}}Building Docker image...{{nc}}"
    docker build -t wimm .

docker-run:
    @echo "{{blue}}Running Docker container...{{nc}}"
    docker run --rm -it wimm

# Performance profiling (if needed in future)
profile:
    @echo "{{blue}}Running with profiling...{{nc}}"
    {{cargo}} build --release
    perf record --call-graph=dwarf target/release/wimm
    perf report

# Memory checking (if valgrind available)
memcheck:
    @echo "{{blue}}Running memory check...{{nc}}"
    {{cargo}} build
    valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all target/debug/wimm

# Feature checking
check-features:
    @echo "{{blue}}Checking feature combinations...{{nc}}"
    {{cargo}} check --no-default-features
    {{cargo}} check --all-features

# Size optimization check
bloat:
    @echo "{{blue}}Analyzing binary size...{{nc}}"
    @if ! command -v cargo-bloat >/dev/null 2>&1; then \
        echo "{{yellow}}Installing cargo-bloat...{{nc}}"; \
        {{cargo}} install cargo-bloat; \
    fi
    {{cargo}} bloat --release

# License checking
license-check:
    @echo "{{blue}}Checking licenses...{{nc}}"
    @if ! command -v cargo-license >/dev/null 2>&1; then \
        echo "{{yellow}}Installing cargo-license...{{nc}}"; \
        {{cargo}} install cargo-license; \
    fi
    {{cargo}} license
