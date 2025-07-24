#!/bin/bash

# Coverage check script for local development
# This script runs coverage analysis and enforces minimum thresholds

set -e

# Configuration
MIN_COVERAGE=${MIN_COVERAGE:-60}
COVERAGE_DIR="target/coverage"
REPORT_FILE="$COVERAGE_DIR/coverage-summary.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_usage() {
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help              Show this help message"
    echo "  -t, --threshold NUM     Set minimum coverage threshold (default: $MIN_COVERAGE)"
    echo "  -o, --open             Open HTML report in browser after generation"
    echo "  -q, --quiet            Suppress verbose output"
    echo "  -f, --fail-fast        Exit immediately on first failure"
    echo "  --no-html              Skip HTML report generation"
    echo "  --clean                Clean previous coverage data before running"
    echo ""
    echo "Environment variables:"
    echo "  MIN_COVERAGE           Minimum coverage percentage (default: 60)"
    echo ""
    echo "Examples:"
    echo "  $0                     # Run with default settings"
    echo "  $0 -t 70 -o           # Require 70% coverage and open report"
    echo "  $0 --clean --quiet    # Clean run with minimal output"
}

# Parse command line arguments
OPEN_REPORT=false
QUIET=false
FAIL_FAST=false
GENERATE_HTML=true
CLEAN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            print_usage
            exit 0
            ;;
        -t|--threshold)
            MIN_COVERAGE="$2"
            shift 2
            ;;
        -o|--open)
            OPEN_REPORT=true
            shift
            ;;
        -q|--quiet)
            QUIET=true
            shift
            ;;
        -f|--fail-fast)
            FAIL_FAST=true
            shift
            ;;
        --no-html)
            GENERATE_HTML=false
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# Validate threshold
if ! [[ "$MIN_COVERAGE" =~ ^[0-9]+(\.[0-9]+)?$ ]] || (( $(echo "$MIN_COVERAGE < 0" | bc -l) )) || (( $(echo "$MIN_COVERAGE > 100" | bc -l) )); then
    log_error "Invalid coverage threshold: $MIN_COVERAGE (must be between 0 and 100)"
    exit 1
fi

# Check if required tools are installed
check_dependencies() {
    log_info "Checking dependencies..."

    if ! command -v cargo &> /dev/null; then
        log_error "cargo is not installed or not in PATH"
        exit 1
    fi

    if ! command -v cargo-tarpaulin &> /dev/null; then
        log_warning "cargo-tarpaulin is not installed. Installing..."
        cargo install cargo-tarpaulin
    fi

    if ! command -v bc &> /dev/null; then
        log_warning "bc (calculator) is not available. Install it for threshold checks."
    fi
}

# Clean previous coverage data
clean_coverage() {
    if [[ "$CLEAN" == true ]]; then
        log_info "Cleaning previous coverage data..."
        rm -rf target/tarpaulin
        rm -rf $COVERAGE_DIR
        rm -f tarpaulin-report.html
        rm -f cobertura.xml
        rm -f tarpaulin-report.json
        cargo clean
    fi
}

# Run tests first to ensure they pass
run_tests() {
    log_info "Running tests..."
    if [[ "$QUIET" == true ]]; then
        cargo test --quiet
    else
        cargo test
    fi
    log_success "All tests passed"
}

# Generate coverage report
generate_coverage() {
    log_info "Generating coverage report (minimum threshold: ${MIN_COVERAGE}%)..."

    # Create coverage directory
    mkdir -p $COVERAGE_DIR

    # Build tarpaulin command
    TARPAULIN_CMD="cargo tarpaulin"

    if [[ "$QUIET" != true ]]; then
        TARPAULIN_CMD="$TARPAULIN_CMD --verbose"
    fi

    if [[ "$GENERATE_HTML" == true ]]; then
        TARPAULIN_CMD="$TARPAULIN_CMD --out Html"
    fi

    # Always generate XML and JSON for parsing
    TARPAULIN_CMD="$TARPAULIN_CMD --out Xml --out Json"

    # Add other options
    TARPAULIN_CMD="$TARPAULIN_CMD --all-features --workspace --timeout 120"
    TARPAULIN_CMD="$TARPAULIN_CMD --exclude-files 'src/main.rs'"

    # Run coverage
    if [[ "$FAIL_FAST" == true ]]; then
        TARPAULIN_CMD="$TARPAULIN_CMD --fail-under $MIN_COVERAGE"
    fi

    eval $TARPAULIN_CMD | tee $REPORT_FILE
}

# Parse and check coverage results
check_coverage_threshold() {
    log_info "Checking coverage threshold..."

    # Extract coverage percentage from the output
    if [[ -f "$REPORT_FILE" ]]; then
        COVERAGE=$(grep -oP '\d+\.\d+(?=% coverage)' "$REPORT_FILE" | head -1)
    else
        # Fallback: run tarpaulin just to get summary
        COVERAGE=$(cargo tarpaulin --print-summary --exclude-files "src/main.rs" 2>/dev/null | grep -oP '\d+\.\d+(?=% coverage)' | head -1 || echo "0")
    fi

    if [[ -z "$COVERAGE" ]]; then
        log_error "Could not parse coverage percentage from output"
        exit 1
    fi

    echo ""
    echo "==================== COVERAGE REPORT ===================="
    echo -e "Current coverage:     ${BLUE}${COVERAGE}%${NC}"
    echo -e "Minimum required:     ${BLUE}${MIN_COVERAGE}%${NC}"
    echo ""

    # Check if bc is available for precise comparison
    if command -v bc &> /dev/null; then
        if (( $(echo "$COVERAGE < $MIN_COVERAGE" | bc -l) )); then
            echo -e "Status:               ${RED}❌ BELOW THRESHOLD${NC}"
            echo "========================================================"
            echo ""
            log_error "Coverage ${COVERAGE}% is below minimum threshold of ${MIN_COVERAGE}%"

            # Calculate how much coverage is needed
            NEEDED=$(echo "scale=2; $MIN_COVERAGE - $COVERAGE" | bc -l)
            log_info "You need to increase coverage by ${NEEDED}% to meet the threshold"

            # Suggest actions
            echo ""
            echo "💡 Suggestions to improve coverage:"
            echo "   • Add tests for uncovered functions and modules"
            echo "   • Add edge case tests for existing functionality"
            echo "   • Add integration tests for cross-module interactions"
            echo "   • Check the HTML report for specific uncovered lines"
            echo ""

            exit 1
        else
            echo -e "Status:               ${GREEN}✅ PASSING${NC}"
            echo "========================================================"
            log_success "Coverage ${COVERAGE}% meets minimum threshold of ${MIN_COVERAGE}%"
        fi
    else
        # Fallback to basic comparison without bc
        COVERAGE_INT=${COVERAGE%.*}
        MIN_COVERAGE_INT=${MIN_COVERAGE%.*}

        if [[ "$COVERAGE_INT" -lt "$MIN_COVERAGE_INT" ]]; then
            echo -e "Status:               ${RED}❌ BELOW THRESHOLD${NC}"
            echo "========================================================"
            log_error "Coverage ${COVERAGE}% appears to be below minimum threshold of ${MIN_COVERAGE}%"
            exit 1
        else
            echo -e "Status:               ${GREEN}✅ PASSING${NC}"
            echo "========================================================"
            log_success "Coverage ${COVERAGE}% meets minimum threshold of ${MIN_COVERAGE}%"
        fi
    fi

    echo ""
}

# Show coverage breakdown by file
show_coverage_breakdown() {
    if [[ "$QUIET" != true ]] && [[ -f "tarpaulin-report.json" ]]; then
        log_info "Coverage breakdown by file:"
        echo ""

        # Try to parse JSON report for detailed breakdown
        if command -v jq &> /dev/null; then
            echo "| File | Coverage |"
            echo "|------|----------|"
            jq -r '.files[] | "\(.path) | \(.coverage * 100 | round)%"' tarpaulin-report.json 2>/dev/null || true
        else
            # Fallback: show basic file list from XML
            if [[ -f "cobertura.xml" ]] && command -v grep &> /dev/null; then
                grep -oP 'filename="\K[^"]*' cobertura.xml | head -10
            fi
        fi
        echo ""
    fi
}

# Open HTML report in browser
open_html_report() {
    if [[ "$OPEN_REPORT" == true ]] && [[ "$GENERATE_HTML" == true ]]; then
        HTML_FILE="tarpaulin-report.html"
        if [[ -f "$HTML_FILE" ]]; then
            log_info "Opening coverage report in browser..."

            if command -v xdg-open &> /dev/null; then
                xdg-open "$HTML_FILE"
            elif command -v open &> /dev/null; then
                open "$HTML_FILE"
            elif command -v start &> /dev/null; then
                start "$HTML_FILE"
            else
                log_warning "Could not detect how to open browser. HTML report is at: $HTML_FILE"
            fi
        else
            log_warning "HTML report not found at $HTML_FILE"
        fi
    fi
}

# Print summary
print_summary() {
    echo ""
    echo "📊 Coverage check completed!"
    echo ""
    echo "Generated files:"
    [[ -f "tarpaulin-report.html" ]] && echo "  • HTML report: tarpaulin-report.html"
    [[ -f "cobertura.xml" ]] && echo "  • XML report: cobertura.xml"
    [[ -f "tarpaulin-report.json" ]] && echo "  • JSON report: tarpaulin-report.json"
    echo ""

    if [[ "$GENERATE_HTML" == true ]]; then
        echo "💡 Open the HTML report in your browser for detailed line-by-line coverage"
    fi
    echo ""
}

# Main execution
main() {
    echo "🧪 Wimm Coverage Checker"
    echo "========================"
    echo ""

    check_dependencies
    clean_coverage
    run_tests
    generate_coverage
    check_coverage_threshold
    show_coverage_breakdown
    open_html_report
    print_summary
}

# Run main function
main "$@"
