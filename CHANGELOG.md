# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
- Renamed project to jarcli (Jarvix CLI)
- Updated binary names from jar to jarcli
- Improved README and contributor docs.
- Added persistent analysis snapshot to `.jarcli/metrics`.
- Added `balanced` profile for `jarcli analyze`.

## [0.2.0] - 2025-12-15
- Enhanced security analysis: Added detection for unsafe blocks, unwrap/expect calls, panic macros
- Added performance analysis: Detection of unnecessary clones on collections
- Improved code quality checks: Enhanced FIXME/TODO detection, deprecated API warnings
- Added complexity analysis: File size and function count optimization suggestions
- Added file-level analysis: #[allow] attribute reviews, dead_code detection
- Maintained zero warnings policy and existing functionality

## [0.1.0] - 2025-12-15
- Initial release candidate: CLI analysis and repair features, JARVIXSERVER integration.

### Registro de comandos y cambios recientes

- Comandos ejecutados: `cargo fmt`; `cargo clippy --fix --allow-dirty --allow-no-vcs`; `cargo clippy -- -D warnings`; `cargo check`; `cargo install --path . --force`; `cargo package --allow-dirty`; `go build ./...`.
- Archivos creados: `LICENSE`, `CHANGELOG.md`, `.github/workflows/ci.yml`.
- Archivos modificados: `src/commands/doc.rs`, `src/commands/cargo.rs`, `src/commands/paths.rs`, `src/cli.rs`, `Cargo.toml` (package name actualizado a `cargo-jarvix`, entrada `[[bin]] name = "jar"` eliminada).
- Backups eliminados: `internal/cargo/detector.go.backup`, `internal/auth/middleware.go.backup`.
- Empaquetado local: `cargo package --allow-dirty` — paquete verificado (49 archivos empaquetados).
