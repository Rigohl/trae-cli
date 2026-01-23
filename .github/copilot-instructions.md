# Instrucciones para Copilot (trae-cli)

## Resumen rápido del repositorio
- **TRAE CLI** es una herramienta de línea de comandos escrita en Rust para analizar, reparar y optimizar proyectos Rust, con integración opcional a **JARVIXSERVER** (API HTTP).
- Proyecto **single-crate** (Rust 2021) con CLI principal (`trae`), un servidor HTTP (`server_http`) y utilidades de análisis/caché.
- Tamaño medio: ~50 archivos fuente y ~30k líneas (ver README). Lenguaje principal: **Rust**.

## Datos clave del repo
- **Toolchain**: CI usa Rust **stable** (`dtolnay/rust-toolchain@v1`).
- **Comandos CI reales**: `.github/workflows/ci.yml` ejecuta `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --release`, `cargo build --release`.
- **Alias local**: `.cargo/config.toml` define `cargo clippy-clean` → `cargo clippy --all-targets -- -D warnings`.
- **build.rs**: en Windows copia `target/release/trae.exe` a `bin/`.
- **Hook**: `.githooks/pre-commit` intenta ejecutar `verify-clippy.ps1` (no existe en el repo). Si tienes PowerShell instalado puede fallar; en Linux sin pwsh cae a `cargo clippy --all-targets -- -D warnings`.

## Bootstrap/Build/Test/Lint (validado)
> **Siempre** usa la toolchain stable. En esta máquina funcionó con `rustc 1.92.0` y `cargo 1.92.0`.

### Bootstrap
```bash
rustc --version
cargo --version
```
No hay pasos extra; `cargo` descargará dependencias desde crates.io en la primera ejecución.

### Lint/Format (CI exige cero warnings)
⚠️ **Importante**: `cargo fmt -- --check` **falla en un clone limpio** (hay diferencias de formato).  
Solución: ejecuta `cargo fmt` primero (modifica muchos archivos) y luego el check pasa.

```bash
cargo fmt
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Tests (CI)
```bash
cargo test --all --release
```
- Primera ejecución puede tardar ~2-3 min por descargas/compilación.

### Build (CI)
```bash
cargo build --release
```

### Ejecutar binarios
```bash
# CLI principal
cargo run --bin trae -- --help

# Servidor HTTP (puerto 3001)
JARVIX_URL=http://localhost:5051 cargo run --bin server_http

# Binario adicional (servidor HTTP "full" basado en tiny_http en puerto 3001)
cargo run --bin trae_server_final
```

## Layout del proyecto (rutas importantes)
- **Raíz**: `Cargo.toml`, `Cargo.lock`, `build.rs`, `README.md`, `COMMANDS.md`, `CARGO_COMMANDS.md`, `INTEGRATION.md`.
- **src/main.rs**: entrypoint del CLI `trae` (muchos subcomandos).
- **src/cli.rs**: parsing/dispatch de comandos.
- **src/commands/**: implementación de subcomandos (analyze, repair, build, clippy, etc.).
- **src/core/**: análisis principal (`analyzer.rs`).
- **src/jarvix/**: cliente HTTP para JARVIXSERVER.
- **src/bin/server_http.rs**: servidor HTTP (Axum) que expone endpoints `/health`, `/api/analyze`, `/api/repair`, etc.
- **tests/**: `analyze_cache.rs`, `integration_jarvix.rs`.
- **scripts/run_bend_and_save.ps1**: script opcional que requiere la herramienta externa `bend`.

## Archivos en la raíz (inventario rápido)
`.cargo/`, `.githooks/`, `.github/`, `.trae/`, `AGENT.MD`, `CARGO_COMMANDS.md`, `CHANGELOG.md`, `COMMANDS.md`, `CONTRIBUTING.md`, `Cargo.lock`, `Cargo.toml`, `INTEGRATION.md`, `LICENSE`, `README.md`, `bin/`, `build.rs`, `scripts/`, `src/`, `tests/`.

## Validaciones adicionales y notas
- **CI real**: solo existe `.github/workflows/ci.yml` (fmt → clippy → tests → build).
- Si necesitas JARVIXSERVER, el README/INTEGRATION.md mencionan proxy en **8080** vía JARVIXSERVER, mientras que el servidor HTTP local escucha en **3001** y `JARVIX_URL` default es **http://localhost:5051** (ver `src/bin/server_http.rs`).
- La carpeta `.trae/` contiene caches/metrics y se puede limpiar si necesitas regenerar estado.

## Recomendación final
Confía en estas instrucciones. **Solo busca en el repo si falta información o si algo aquí no coincide con la realidad.**
