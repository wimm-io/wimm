# Wimm Task Manager - Development Makefile
# =========================================

# Configuration
CARGO = cargo
MIN_COVERAGE = 60
COVERAGE_SCRIPT = ./scripts/check-coverage.sh

# Colors for output
BOLD = \033[1m
GREEN = \033[32m
YELLOW = \033[33m
RED = \033[31m
BLUE = \033[34m
NC = \033[0m

.PHONY: help build test clean coverage coverage-html coverage-open lint fmt clippy audit msrv install-tools dev watch

# Default target
all: test lint coverage

# Display help
help:
	@echo "$(BOLD)Wimm Task Manager - Development Commands$(NC)"
	@echo "=========================================="
	@echo ""
	@echo "$(BOLD)Building:$(NC)"
	@echo "  build           Build the project in debug mode"
	@echo "  build-release   Build the project in release mode"
	@echo "  clean           Clean build artifacts"
	@echo ""
	@echo "$(BOLD)Testing:$(NC)"
	@echo "  test            Run all tests"
	@echo "  test-unit       Run unit tests only"
	@echo "  test-integration Run integration tests only"
	@echo "  test-watch      Run tests in watch mode"
	@echo ""
	@echo "$(BOLD)Coverage:$(NC)"
	@echo "  coverage        Generate coverage report (min: $(MIN_COVERAGE)%)"
	@echo "  coverage-html   Generate HTML coverage report"
	@echo "  coverage-open   Generate and open HTML coverage report"
	@echo "  coverage-clean  Clean coverage artifacts"
	@echo ""
	@echo "$(BOLD)Code Quality:$(NC)"
	@echo "  lint            Run all linting (fmt + clippy)"
	@echo "  fmt             Format code with rustfmt"
	@echo "  fmt-check       Check code formatting"
	@echo "  clippy          Run clippy lints"
	@echo "  clippy-fix      Fix clippy warnings automatically"
	@echo ""
	@echo "$(BOLD)Security & Compatibility:$(NC)"
	@echo "  audit           Run security audit"
	@echo "  msrv            Check minimum supported Rust version"
	@echo ""
	@echo "$(BOLD)Development:$(NC)"
	@echo "  install-tools   Install required development tools"
	@echo "  dev             Full development check (test + lint + coverage)"
	@echo "  watch           Watch for changes and run tests"
	@echo "  run             Run the application"
	@echo ""
	@echo "$(BOLD)Environment Variables:$(NC)"
	@echo "  MIN_COVERAGE    Minimum coverage percentage (default: $(MIN_COVERAGE))"
	@echo ""

# Building
build:
	@echo "$(BLUE)Building project...$(NC)"
	$(CARGO) build

build-release:
	@echo "$(BLUE)Building project (release mode)...$(NC)"
	$(CARGO) build --release

clean:
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	$(CARGO) clean
	@echo "$(BLUE)Cleaning coverage reports...$(NC)"
	rm -f cobertura.xml tarpaulin-report.html tarpaulin-report.json lcov.info
	rm -rf target/tarpaulin

# Testing
test:
	@echo "$(BLUE)Running all tests...$(NC)"
	$(CARGO) test --verbose

test-unit:
	@echo "$(BLUE)Running unit tests...$(NC)"
	$(CARGO) test --lib --verbose

test-integration:
	@echo "$(BLUE)Running integration tests...$(NC)"
	$(CARGO) test --test '*' --verbose

test-watch:
	@echo "$(BLUE)Running tests in watch mode...$(NC)"
	$(CARGO) watch -x test

# Coverage
coverage:
	@echo "$(BLUE)Generating coverage report (minimum: $(MIN_COVERAGE)%)...$(NC)"
	@if [ -x "$(COVERAGE_SCRIPT)" ]; then \
		MIN_COVERAGE=$(MIN_COVERAGE) $(COVERAGE_SCRIPT) --quiet; \
	else \
		echo "$(YELLOW)Coverage script not found, falling back to basic tarpaulin...$(NC)"; \
		$(CARGO) tarpaulin --exclude-files "src/main.rs" --fail-under $(MIN_COVERAGE); \
	fi

coverage-html:
	@echo "$(BLUE)Generating HTML coverage report...$(NC)"
	@if [ -x "$(COVERAGE_SCRIPT)" ]; then \
		MIN_COVERAGE=$(MIN_COVERAGE) $(COVERAGE_SCRIPT) --quiet; \
	else \
		$(CARGO) tarpaulin --exclude-files "src/main.rs" --out Html; \
	fi

coverage-open:
	@echo "$(BLUE)Generating and opening HTML coverage report...$(NC)"
	@if [ -x "$(COVERAGE_SCRIPT)" ]; then \
		MIN_COVERAGE=$(MIN_COVERAGE) $(COVERAGE_SCRIPT) --open; \
	else \
		$(CARGO) tarpaulin --exclude-files "src/main.rs" --out Html; \
		@if [ -f "tarpaulin-report.html" ]; then \
			if command -v xdg-open >/dev/null 2>&1; then \
				xdg-open tarpaulin-report.html; \
			elif command -v open >/dev/null 2>&1; then \
				open tarpaulin-report.html; \
			else \
				echo "$(YELLOW)HTML report generated: tarpaulin-report.html$(NC)"; \
			fi; \
		fi; \
	fi

coverage-clean:
	@echo "$(BLUE)Cleaning coverage artifacts...$(NC)"
	rm -rf target/tarpaulin
	rm -f tarpaulin-report.html cobertura.xml tarpaulin-report.json

# Code quality
lint: fmt-check clippy

fmt:
	@echo "$(BLUE)Formatting code...$(NC)"
	$(CARGO) fmt

fmt-check:
	@echo "$(BLUE)Checking code formatting...$(NC)"
	$(CARGO) fmt --all -- --check

clippy:
	@echo "$(BLUE)Running clippy lints...$(NC)"
	$(CARGO) clippy --all-targets --all-features -- -D warnings

clippy-fix:
	@echo "$(BLUE)Fixing clippy warnings...$(NC)"
	$(CARGO) clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

# Security and compatibility
audit:
	@echo "$(BLUE)Running security audit...$(NC)"
	@if ! command -v cargo-audit >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-audit...$(NC)"; \
		$(CARGO) install cargo-audit; \
	fi
	$(CARGO) audit

msrv:
	@echo "$(BLUE)Checking minimum supported Rust version...$(NC)"
	$(CARGO) +1.70 check --all-targets --all-features

# Development tools
install-tools:
	@echo "$(BLUE)Installing development tools...$(NC)"
	@echo "Installing cargo-tarpaulin..."
	$(CARGO) install cargo-tarpaulin
	@echo "Installing cargo-watch..."
	$(CARGO) install cargo-watch
	@echo "Installing cargo-audit..."
	$(CARGO) install cargo-audit
	@echo "$(GREEN)All tools installed successfully!$(NC)"

# Development workflow
dev: test lint coverage
	@echo "$(GREEN)✅ All development checks passed!$(NC)"

watch:
	@echo "$(BLUE)Watching for changes...$(NC)"
	@if ! command -v cargo-watch >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-watch...$(NC)"; \
		$(CARGO) install cargo-watch; \
	fi
	$(CARGO) watch -x check -x test

run:
	@echo "$(BLUE)Running application...$(NC)"
	$(CARGO) run

# CI simulation
ci:
	@echo "$(BLUE)Running CI simulation...$(NC)"
	@echo "$(BLUE)Step 1/4: Building...$(NC)"
	@$(MAKE) build
	@echo "$(BLUE)Step 2/4: Testing...$(NC)"
	@$(MAKE) test
	@echo "$(BLUE)Step 3/4: Linting...$(NC)"
	@$(MAKE) lint
	@echo "$(BLUE)Step 4/4: Coverage...$(NC)"
	@$(MAKE) coverage
	@echo "$(GREEN)✅ CI simulation completed successfully!$(NC)"

# Quick development cycle
quick: test clippy
	@echo "$(GREEN)✅ Quick checks passed!$(NC)"

# Pre-commit hook simulation
pre-commit: fmt test clippy
	@echo "$(GREEN)✅ Pre-commit checks passed!$(NC)"

# Release preparation
release-prep: clean build-release test lint coverage audit
	@echo "$(GREEN)✅ Release preparation completed!$(NC)"

# Benchmarking (if needed in future)
bench:
	@echo "$(BLUE)Running benchmarks...$(NC)"
	$(CARGO) bench

# Documentation
docs:
	@echo "$(BLUE)Generating documentation...$(NC)"
	$(CARGO) doc --open

docs-check:
	@echo "$(BLUE)Checking documentation...$(NC)"
	$(CARGO) doc --no-deps

# Dependency management
update:
	@echo "$(BLUE)Updating dependencies...$(NC)"
	$(CARGO) update

outdated:
	@echo "$(BLUE)Checking for outdated dependencies...$(NC)"
	@if ! command -v cargo-outdated >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-outdated...$(NC)"; \
		$(CARGO) install cargo-outdated; \
	fi
	$(CARGO) outdated

# Docker (if needed in future)
docker-build:
	@echo "$(BLUE)Building Docker image...$(NC)"
	docker build -t wimm .

docker-run:
	@echo "$(BLUE)Running Docker container...$(NC)"
	docker run --rm -it wimm

# Performance profiling (if needed in future)
profile:
	@echo "$(BLUE)Running with profiling...$(NC)"
	$(CARGO) build --release
	perf record --call-graph=dwarf target/release/wimm
	perf report

# Memory checking (if valgrind available)
memcheck:
	@echo "$(BLUE)Running memory check...$(NC)"
	$(CARGO) build
	valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all target/debug/wimm

# Feature checking
check-features:
	@echo "$(BLUE)Checking feature combinations...$(NC)"
	$(CARGO) check --no-default-features
	$(CARGO) check --all-features

# Size optimization check
bloat:
	@echo "$(BLUE)Analyzing binary size...$(NC)"
	@if ! command -v cargo-bloat >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-bloat...$(NC)"; \
		$(CARGO) install cargo-bloat; \
	fi
	$(CARGO) bloat --release

# License checking
license-check:
	@echo "$(BLUE)Checking licenses...$(NC)"
	@if ! command -v cargo-license >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-license...$(NC)"; \
		$(CARGO) install cargo-license; \
	fi
	$(CARGO) license
