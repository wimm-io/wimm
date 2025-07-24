# Wimm - Where Is My Mind Task Manager 🧠

A fast, efficient terminal-based task manager built in Rust with comprehensive test coverage and modern development practices.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Coverage](https://img.shields.io/badge/coverage-62.59%25-brightgreen?style=for-the-badge)
![Tests](https://img.shields.io/badge/tests-130%20passing-brightgreen?style=for-the-badge)
![CI](https://img.shields.io/github/actions/workflow/status/wimm-io/wimm/rust.yml?style=for-the-badge)

## 🚀 Features

- **Fast Terminal UI**: Built with [ratatui](https://ratatui.rs/) for responsive terminal interface
- **Flexible Storage**: Support for both in-memory and persistent (Sled) storage backends
- **Smart Date Parsing**: Natural language date input (`today`, `2d`, `friday`, etc.)
- **Vim-like Navigation**: Intuitive keyboard shortcuts for power users
- **Task Management**: Create, edit, complete, and delete tasks with ease
- **Visual Indicators**: Color-coded tasks based on due dates and status
- **Help System**: Built-in help panel with keyboard shortcuts
- **Customizable Configuration**: Color schemes, keymaps, and default settings
- **CLI Interface**: Command-line configuration management and overrides

## 📋 Task Features

- ✅ **Task CRUD**: Create, read, update, delete tasks
- 🗓️ **Due Dates**: Set due dates with natural language parsing
- ⏰ **Defer Dates**: Schedule tasks to appear later
- ✅ **Completion Tracking**: Mark tasks as complete/incomplete
- 🏷️ **Task Selection**: Multi-select for batch operations
- 📝 **Rich Descriptions**: Full descriptions with in-place editing
- 🎨 **Themes**: Built-in color schemes (default, dark, light) and custom themes
- ⌨️ **Keymaps**: Multiple keymap options (default, vi-style) and custom bindings
- ⏰ **Time Defaults**: Configurable default hours for defer and due dates

## 🛠️ Installation

### Prerequisites

- Rust 1.74+ (MSRV)
- A terminal emulator

### From Source

```bash
git clone https://github.com/username/wimm.git
cd wimm
cargo build --release
./target/release/wimm
```

### Development Setup

```bash
# First, install Just command runner
# Option 1: Using Homebrew (macOS/Linux)
brew install just

# Option 2: Using Cargo (any platform with Rust)
cargo install just

# Option 3: Using the migration script
./scripts/migrate-to-just.sh

# Then install development tools
just install-tools

# Run all checks (tests + lint + coverage)
just dev

# Start development with file watching
just watch
```

## ⚙️ Configuration

WIMM supports extensive customization through CLI commands and configuration files.

### Quick Configuration

```bash
# List available themes and keymaps
wimm config list-colors
wimm config list-keymaps

# Set your preferences (key-value format)
wimm config set color-scheme dark
wimm config set keymap vi
wimm config set defer-hour 8
wimm config set due-hour 18

# Or use flag format for multiple changes
wimm config set --color-scheme dark --keymap vi --defer-hour 8 --due-hour 18

# Show current configuration
wimm config show

# Reset to defaults
wimm config reset
```

### Bulk Configuration Changes

Set multiple configuration values at once:

```bash
# Use flag format for multiple changes
wimm config set --color-scheme dark --defer-hour 8 --due-hour 18

# Mix flag and key-value formats
wimm config set timezone UTC --color-scheme light
```

### Built-in Themes

- **default**: High-contrast black and white
- **dark**: Modern dark theme with blue accents
- **light**: Clean light theme

### Built-in Keymaps

- **default**: Standard TUI keybindings
- **vi**: Vim-inspired navigation and commands

See [`docs/configuration.md`](docs/configuration.md) for complete configuration documentation.

## 🎮 Usage

### Basic Navigation

| Key     | Action                |
| ------- | --------------------- |
| `j`/`k` | Move up/down          |
| `g`/`G` | Go to first/last task |
| `h`     | Toggle help panel     |
| `q`     | Quit                  |

### Task Management

| Key | Action                 |
| --- | ---------------------- |
| `o` | Create new task below  |
| `O` | Create new task above  |
| `i` | Edit current task      |
| `!` | Toggle task completion |
| `x` | Toggle task selection  |
| `D` | Delete selected tasks  |

### Insert Mode

| Key         | Action                                         |
| ----------- | ---------------------------------------------- |
| `Tab`       | Next field (Title → Description → Due → Defer) |
| `Shift+Tab` | Previous field                                 |
| `Enter`     | Save and return to normal mode                 |
| `Esc`       | Cancel and return to normal mode               |

### Date Input Examples

```
today              # Today at 5pm
tomorrow           # Tomorrow at 5pm
friday             # Next Friday at 5pm
next monday        # Next Monday at 5pm
2d                 # 2 days from now
1w                 # 1 week from now
3h                 # 3 hours from now
30m                # 30 minutes from now
2024-12-25         # Christmas 2024
12-25              # December 25th this year
(empty)            # Clear the date
```

## 🧪 Testing & Coverage

This project maintains **high test coverage (62.59%)** with comprehensive unit and integration tests.

### Quick Coverage Check

```bash
# Generate and open HTML coverage report
just coverage-open

# Run coverage with threshold enforcement
just coverage

# Run all development checks
just dev
```

### Coverage Details

| Module        | Coverage   | Status                     |
| ------------- | ---------- | -------------------------- |
| Storage       | 92.30%     | ✅ Excellent               |
| Types         | 100%       | ✅ Complete                |
| Time Tracking | 100%       | ✅ Complete                |
| UI Events     | 84.71%     | ✅ Good                    |
| Help Panel    | 100%       | ✅ Complete                |
| Layout        | 100%       | ✅ Complete                |
| **Overall**   | **62.59%** | ✅ **Above 60% threshold** |

### Test Types

- **130 Total Tests**: Comprehensive test suite
- **116 Unit Tests**: Individual component testing
- **14 Integration Tests**: End-to-end workflows
- **Error Path Testing**: Failure scenario coverage
- **Edge Case Testing**: Boundary condition handling

## 🏗️ Development

### Available Commands

```bash
# Testing
just test              # Run all tests
just test-unit         # Unit tests only
just test-integration  # Integration tests only
just test-watch        # Watch mode

# Coverage
just coverage          # Basic coverage check (60% threshold)
just coverage-html     # Generate HTML report
just coverage-open     # Generate and open HTML report
just coverage-clean    # Clean coverage artifacts

# Code Quality
just lint              # Format + clippy
just fmt               # Format code
just clippy            # Lint code
just audit             # Security audit

# Development Workflow
just dev               # Full check (test + lint + coverage)
just ci                # CI simulation
just quick             # Fast check (test + clippy)
just pre-commit        # Pre-commit simulation
```

### Project Structure

```
wimm/
├── src/
│   ├── storage/           # Data persistence layer
│   ├── time_tracking/     # Time tracking functionality
│   ├── ui/               # Terminal user interface
│   │   ├── app.rs        # Main application logic
│   │   ├── events.rs     # Event handling
│   │   ├── help_panel.rs # Help system
│   │   └── layout.rs     # UI layout management
│   ├── input/            # Input handling
│   ├── types.rs          # Core data types
│   └── main.rs           # Application entry point
├── tests/                # Integration tests
├── scripts/              # Development scripts
├── docs/                 # Documentation
└── .github/              # CI/CD configuration
```

### Storage Backends

#### Memory Storage

- Fast in-memory storage
- Perfect for testing and temporary use
- No persistence between sessions

#### Sled Storage

- Persistent embedded database
- Fast key-value storage
- Automatic data recovery

### Pre-commit Hooks

Set up automatic code quality checks:

```bash
pip install pre-commit
pre-commit install

# Manual run
pre-commit run --all-files
```

Hooks include:

- Rust formatting check
- Clippy lints
- Test execution
- Coverage threshold enforcement
- Security audit

## 🔧 Configuration

### Coverage Configuration

The project enforces a **60% minimum coverage threshold**. Configuration in `tarpaulin.toml`:

```toml
[coverage]
fail-under = 60
exclude-files = ["src/main.rs", "tests/*"]

[output]
out = ["Html", "Xml", "Json"]
```

### CI/CD

GitHub Actions workflow includes:

- ✅ Automated testing on all PRs
- ✅ Coverage enforcement (fails below 60%)
- ✅ Code quality checks (rustfmt, clippy)
- ✅ Security audit
- ✅ MSRV compatibility check
- ✅ Automatic coverage comments on PRs

### Optional Coverage Services

The CI includes optional integrations with external coverage services:

- **Codecov**: Attempts upload if `CODECOV_TOKEN` is configured
- **Coveralls**: Uses GitHub token (automatic)

To enable Codecov:

1. Sign up at [codecov.io](https://codecov.io)
2. Add your repository
3. Add `CODECOV_TOKEN` to GitHub repository secrets in your repo settings

These services are optional - CI will pass even if tokens are missing or uploads fail.

## 🚦 Quality Gates

### Pull Request Requirements

- [ ] All tests pass
- [ ] Coverage ≥ 60%
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Security audit clean
- [ ] Documentation updated

### Coverage Monitoring

- **Threshold**: 60% minimum coverage
- **Enforcement**: CI fails if coverage drops
- **Reporting**: Automatic PR comments with coverage details
- **Artifacts**: HTML coverage reports in CI artifacts

## 🤝 Contributing

1. **Fork and Clone**

   ```bash
   git clone your-fork-url
   cd wimm
   ```

2. **Set Up Development Environment**

   ```bash
   # Install Just first (choose one method):
   brew install just              # macOS/Linux with Homebrew
   cargo install just             # Any platform with Rust

   # Then set up development tools
   just install-tools             # Install dev tools
   pre-commit install             # Install pre-commit hooks
   ```

3. **Make Changes**

   ```bash
   # Create feature branch
   git checkout -b feature/your-feature

   # Make changes and add tests
   # Ensure coverage doesn't drop
   just dev
   ```

4. **Submit Pull Request**
   - Ensure all quality gates pass
   - Include tests for new functionality
   - Update documentation if needed

### Contributing Guidelines

- **Test Coverage**: New code must include tests
- **Code Style**: Follow existing patterns and rustfmt
- **Documentation**: Update docs for user-facing changes
- **Commit Messages**: Use conventional commit format
- **Performance**: Consider performance impact of changes

## 📊 Performance

- **Startup Time**: < 100ms
- **Memory Usage**: < 10MB typical
- **Storage**: Efficient embedded database
- **UI Responsiveness**: 60fps terminal rendering

## 🔒 Security

- **Dependency Audit**: Automated security scanning
- **Input Validation**: Comprehensive input sanitization
- **Error Handling**: Graceful failure modes
- **Data Safety**: Atomic database operations

## 📚 Documentation

- [`COVERAGE.md`](docs/COVERAGE.md) - Detailed coverage documentation
- [`ARCHITECTURE.md`](ARCHITECTURE.md) - System architecture overview
- [`docs/configuration.md`](docs/configuration.md) - Complete configuration guide
- [API Documentation](https://docs.rs/wimm) - Generated API docs

## 🐛 Troubleshooting

### Common Issues

**Coverage Below Threshold**

```bash
just coverage-open  # View detailed HTML report
# Add tests for red highlighted lines
```

**Build Failures**

```bash
cargo clean
cargo build
```

**Test Failures**

```bash
cargo test -- --nocapture  # See test output
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ratatui](https://ratatui.rs/) - Excellent TUI framework
- [sled](https://sled.rs/) - Fast embedded database
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) - Coverage tool
- Rust community for excellent tooling and libraries

## 🔗 Links

- [GitHub Repository](https://github.com/username/wimm)
- [Issue Tracker](https://github.com/username/wimm/issues)
- [Releases](https://github.com/username/wimm/releases)
- [Documentation](https://docs.rs/wimm)

---

**Made with ❤️ in Rust**

_Keep your tasks organized, your mind clear, and your code covered!_ 🧠✅
