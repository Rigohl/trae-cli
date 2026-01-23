# TRAE CLI — Advanced Rust Development Toolkit

[![CI](https://github.com/Rigohl/trae-cli/workflows/CI/badge.svg)](https://github.com/Rigohl/trae-cli/actions)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/Rigohl/trae-cli/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Rigohl/trae-cli/blob/master/LICENSE)

TRAE CLI (Total Rust Analysis Engine) is a command-line tool for analyzing, repairing and optimizing Rust projects with JARVIXSERVER integration.

## ✨ Features

- 🚀 **Fast Analysis**: File-system aware analysis with intelligent caching
- 🔒 **Security First**: Detects unsafe blocks, unwrap calls, and security issues
- 📊 **Quality Metrics**: Comprehensive code quality analysis
- 🔧 **Auto Repair**: Automatic code improvements and optimizations
- 🌐 **JARVIXSERVER Integration**: Seamless integration with MCP tools
- ⚡ **Zero Warnings**: Strict CI policy with clippy checks
- 📈 **Performance Optimized**: Parallel processing with rayon
- 🛠️ **Rich Command Set**: Full cargo wrapper with enhanced functionality

## 🚀 Quick Start

```bash
# Install globally
cargo install --path .

# Run analysis and repair
cargo run --bin trae -- repair

# Check code quality
cargo run --bin trae -- clippy

# Run preflight check (fmt + clippy + test + build)
cargo run --bin trae -- preflight

# View available commands
cargo run --bin trae -- --help

# Start HTTP server for JARVIXSERVER integration
cargo run --bin server_http
```

## 📊 Current Analysis Results

TRAE CLI provides comprehensive analysis capabilities:
- **Multi-file analysis**: Analyzes all Rust files in your project
- **Security detection**: Identifies unsafe blocks and potential vulnerabilities
- **Code quality metrics**: Provides detailed quality assessments
- **Performance insights**: Suggests optimization opportunities
- **Parallel processing**: Utilizes rayon for fast analysis

## 🛠️ Development

### Build Commands
```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Code quality (zero warnings policy)
cargo clippy -- -D warnings
cargo fmt
```

### Project Structure
```
trae-cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli.rs               # CLI structure and commands
│   ├── config.rs            # Configuration management
│   ├── api.rs               # API definitions
│   ├── core/
│   │   └── analyzer.rs      # Code analysis engine
│   ├── commands/
│   │   ├── analyze.rs       # Analysis command
│   │   ├── repair.rs        # Auto repair functionality
│   │   ├── build.rs         # Build command
│   │   ├── clippy.rs        # Clippy integration
│   │   ├── test.rs          # Testing command
│   │   ├── security.rs      # Security analysis
│   │   ├── metrics.rs       # Metrics collection
│   │   └── ...              # Other commands
│   ├── bin/
│   │   └── server_http.rs   # HTTP server for JARVIXSERVER
│   ├── jarvix/              # JARVIXSERVER integration
│   ├── metrics/             # Metrics collection system
│   └── utils/               # Utility functions
├── tests/
│   ├── analyze_cache.rs     # Analysis caching tests
│   └── integration_jarvix.rs # JARVIXSERVER integration tests
├── Cargo.toml               # Dependencies (pinned versions)
├── CHANGELOG.md             # Version history
├── COMMANDS.md              # Available commands reference
└── README.md                # This file
```

## 🔧 Configuration

### Environment Variables
```bash
# JARVIXSERVER integration
export JARVIX_URL=http://localhost:8080

# Debug mode
export RUST_LOG=debug
```

### Cargo Configuration
```toml
# .cargo/config.toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## 📈 Performance

- **Startup Time**: < 2 seconds
- **Analysis Speed**: ~500ms for 10k lines
- **Memory Usage**: < 50MB
- **Binary Size**: ~5MB (release build)
- **Cache Hit Rate**: Intelligent fingerprint-based caching

## 🔍 Code Quality Standards

TRAE CLI follows strict Rust development practices:

- ✅ **No Mocks**: Real code analysis only (constitution requirement)
- ✅ **Zero Warnings**: CI enforces clippy -D warnings
- ✅ **Explicit Error Handling**: No unwrap() in production code
- ✅ **Performance First**: Parallel processing with rayon
- ✅ **Security Focused**: Unsafe block detection and analysis
- ✅ **Real Compilation**: No false positives in analysis

## 🤝 JARVIXSERVER Integration

TRAE CLI integrates seamlessly with JARVIXSERVER for enhanced capabilities:

### Architecture
```
┌─────────────────┐    ┌─────────────────┐
│   JARVIXSERVER  │    │    TRAE CLI     │
│    (Port 8080)  │◄──►│   (Port 3001)   │
│                 │    │                 │
│ • API Gateway   │    │ • Code Analysis │
│ • Proxy Router  │    │ • Auto Repair   │
│ • Metrics Hub   │    │ • Quality Score │
└─────────────────┘    └─────────────────┘
```

### HTTP Server API

TRAE CLI includes an HTTP server for JARVIXSERVER integration. The server provides these endpoints:

```
GET  /health         - Health check
POST /api/analyze    - Code analysis
POST /api/repair     - Auto repair
GET  /api/metrics    - System metrics
```

When integrated with JARVIXSERVER, these endpoints are proxied under `/trae/*`:
```
GET  /trae/health         → http://localhost:3001/health
POST /trae/api/analyze    → http://localhost:3001/api/analyze
POST /trae/api/repair     → http://localhost:3001/api/repair
GET  /trae/api/metrics    → http://localhost:3001/api/metrics
```

### MCP Tools Integration
- **Nuclear Crawler**: Advanced code analysis
- **Memory Performance**: Optimization tools
- **Web Search**: Documentation lookup

## 📚 Documentation

- [INTEGRATION.md](./INTEGRATION.md) - JARVIXSERVER integration guide
- [CARGO_COMMANDS.md](./CARGO_COMMANDS.md) - Build optimization guide
- [COMMANDS.md](./COMMANDS.md) - Available commands reference
- [CHANGELOG.md](./CHANGELOG.md) - Version history
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines

## 🐛 Troubleshooting

### Build Issues
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Analysis Errors
```bash
# Check file permissions
ls -la src/

# Verify Rust toolchain
rustc --version
cargo --version
```

### JARVIXSERVER Connection
```bash
# Check JARVIXSERVER status
curl http://localhost:8080/health

# Test TRAE CLI server
curl http://localhost:3001/health

# Test via JARVIXSERVER proxy
curl http://localhost:8080/trae/health
```

## 📄 License

Licensed under MIT OR Apache-2.0.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`feat/` or `fix/`)
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check quality: `cargo clippy -- -D warnings`
7. Submit a pull request

### Pre-commit Hooks

This repo includes pre-commit hooks that enforce code quality:

```bash
# Install hooks
./scripts/install-git-hooks.ps1

# Manual verification
cargo clippy -- -D warnings
```

---

**Built with ❤️ for the Rust community - Zero Warnings, No Mocks, Production Ready**
