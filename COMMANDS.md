# Comandos Reproducibles — trae-cli

Este archivo recoge los comandos usados para preparar, validar y empaquetar `cargo-trae`. No incluye commits ni push.

## Entorno
- Rust + Cargo instalados
- Go instalado (para validar `JARVIXSERVER`)

## Formateo y lint
```bash
# Formatear
cargo fmt

# Intentar arreglos automáticos y luego validar estrictamente
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo clippy --all-targets --all-features -- -D warnings

# Comprobación rápida
cargo check
```

## Build
```bash
# Build (desarrollo)
cargo build

# Build release (binario optimizado)
cargo build --release
```

## Ejecutar servidor HTTP (trae-server)
```bash
# Ejecutar binario servidor (release)
./target/release/trae-server

# O en Windows
.\target\release\trae-server.exe
```

## Tests
```bash
# Ejecutar tests
cargo test --all
```

## Instalación local para pruebas
```bash
# Instala localmente la crate desde el directorio actual
cargo install --path . --force

# Verificar versión
cargo-trae --version
# o
trae --version    # si apuntas al bin alias local
```

## Validación de empaquetado (pre-publish)
```bash
# Crea y verifica el paquete localmente (no publica)
cargo package --allow-dirty
```

## Publicar (ejecutar localmente — requiere token de crates.io)
```bash
# Login (ejecutar localmente; no compartir token)
cargo login <CRATES_IO_TOKEN>

# Publicar (ejecutar localmente desde la raíz del proyecto)
cargo publish --allow-dirty
```

Nota: publicar requiere que `version` en `Cargo.toml` no esté ya registrada en crates.io.

## Validación de JARVIXSERVER (Go)
```bash
cd ../JARVIXSERVER
# Compila todos los paquetes
go build ./...
```

## Comandos CI (local / reproducible)
```bash
# Simular pasos del CI (fmt, clippy, tests, build)
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --release
cargo build --release
```

## Notas rápidas
- Ya se eliminó la entrada duplicada de `[[bin]] name = "trae"` en `Cargo.toml`.
- Archivos añadidos: `LICENSE`, `CHANGELOG.md`, `.github/workflows/ci.yml`.
- Para publicar, ejecuta los pasos de la sección "Publicar" desde tu máquina local.
