# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
- Improved README and contributor docs.
- Added persistent analysis snapshot to `.trae/metrics`.
- Added `balanced` profile for `trae analyze`.

## [0.1.0] - 2025-12-15
- Initial release candidate: CLI analysis and repair features, JARVIXSERVER integration.

### Registro de comandos y cambios recientes

- Comandos ejecutados: `cargo fmt`; `cargo clippy --fix --allow-dirty --allow-no-vcs`; `cargo clippy -- -D warnings`; `cargo check`; `cargo install --path . --force`; `cargo package --allow-dirty`; `go build ./...`.
- Archivos creados: `LICENSE`, `CHANGELOG.md`, `.github/workflows/ci.yml`.
- Archivos modificados: `src/commands/doc.rs`, `src/commands/cargo.rs`, `src/commands/paths.rs`, `src/cli.rs`, `Cargo.toml` (package name actualizado a `cargo-trae`, entrada `[[bin]] name = "trae"` eliminada).
- Backups eliminados: `internal/cargo/detector.go.backup`, `internal/auth/middleware.go.backup`.
- Empaquetado local: `cargo package --allow-dirty` — paquete verificado (49 archivos empaquetados).
