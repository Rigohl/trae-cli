# Jarvix CLI — Advanced Rust Development Toolkit

[![CI](https://github.com/Rigohl/jarvix-cli/workflows/CI/badge.svg)](https://github.com/Rigohl/jarvix-cli/actions)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/Rigohl/jarvix-cli/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Rigohl/jarvix-cli/blob/master/LICENSE)

Jarvix CLI (Jarvix Analysis Runtime) is a command-line tool for analyzing, repairing and optimizing Rust projects with JARVIXSERVER integration.

## ✨ Features

- 🚀 **Fast Analysis**: File-system aware analysis with intelligent caching (.jarvix/cache)
- 🔒 **Security First**: Detects unsafe blocks, unwrap calls, and panic macros
- 📊 **Quality Metrics**: Six Sigma analysis with DPMO calculations
- 🔧 **Auto Repair**: Automatic code improvements and optimizations
- 🌐 **JARVIXSERVER Integration**: Seamless integration with MCP tools
- ⚡ **Zero Warnings**: Strict CI policy with clippy -D warnings
- 📈 **Performance Optimized**: Parallel processing with rayon

## 🚀 Quick Start

```bash
# Install globally
cargo install --path .

# Run analysis (programmatic API)
cargo run --bin jar -- repair

# Check code quality
cargo run --bin jar -- clippy --strict

# View available commands
cargo run --bin jar -- --help
```

## 📊 Current Analysis Results

Latest project analysis shows:
- **49 total files analyzed**
- **31,267 lines of code**
- **92 issues detected** (security, performance, quality)
- **30 optimization suggestions**
- **Parallel processing**: 4,900 units in optimized chunks

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
jarvix-cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── core/
│   │   └── analyzer.rs      # Six Sigma analysis engine
│   ├── commands/
│   │   ├── analyze.rs       # Analysis command (API)
│   │   └── repair.rs        # Auto repair functionality
│   └── bin/
│       └── server_http.rs   # HTTP server for JARVIXSERVER
├── tests/
│   ├── analyze_cache.rs     # Analysis testing
│   └── integration_jarvix.rs # JARVIXSERVER integration
├── Cargo.toml               # Dependencies (pinned versions)
├── CHANGELOG.md            # Version history
└── README.md              # This file
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

Jarvix CLI follows strict Rust development practices:

- ✅ **No Mocks**: Real code analysis only (constitution requirement)
- ✅ **Zero Warnings**: CI enforces clippy -D warnings
- ✅ **Explicit Error Handling**: No unwrap() in production code
- ✅ **Performance First**: Parallel processing with rayon
- ✅ **Security Focused**: Unsafe block detection and analysis
- ✅ **Real Compilation**: No false positives in analysis

## 🤝 JARVIXSERVER Integration

Jarvix CLI integrates seamlessly with JARVIXSERVER for enhanced capabilities:

### Architecture
```
┌─────────────────┐    ┌─────────────────┐
│   JARVIXSERVER  │    │   Jarvix CLI    │
│    (Port 8080)  │◄──►│   (Port 3001)   │
│                 │    │                 │
│ • API Gateway   │    │ • Code Analysis │
│ • Proxy Router  │    │ • Auto Repair   │
│ • Metrics Hub   │    │ • Quality Score │
└─────────────────┘    └─────────────────┘
```

### Proxy Endpoints
- `GET /jar/health` - Health check
- `POST /jar/api/analyze` - Code analysis
- `POST /jar/api/repair` - Auto repair
- `GET /jar/api/metrics` - System metrics

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

# Test Jarvix CLI integration
curl http://localhost:8080/jar/health
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
