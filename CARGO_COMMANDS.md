# 🚀 TRAE CLI - Comandos Cargo Optimizados

## Comandos Principales

### Compilación y Build
```bash
# Build del servidor HTTP
cargo build --release --bin server_http

# Build del CLI principal
cargo build --release --bin trae

# Build con todas las features
cargo build --release --all-features

# Build con verbose output
cargo build --release --verbose
```

### Testing y Quality
```bash
# Ejecutar todos los tests
cargo test

# Tests con output detallado
cargo test -- --nocapture

# Tests de integración
cargo test --test integration

# Verificar código sin compilar
cargo check

# Linting con clippy
cargo clippy

# Formateo automático
cargo fmt

# Verificar formato
cargo fmt --check
```

### Análisis y Métricas
```bash
# Análisis completo del proyecto
curl -X POST http://localhost:3001/api/analyze

# Reparación automática
curl -X POST http://localhost:3001/api/repair

# Métricas del sistema
curl http://localhost:3001/api/metrics
```

### Dependencias
```bash
# Actualizar dependencias
cargo update

# Ver árbol de dependencias
cargo tree

# Limpiar cache
cargo clean

# Verificar dependencias
cargo audit
```

### Documentación
```bash
# Generar documentación
cargo doc

# Abrir documentación en navegador
cargo doc --open

# Documentación con dependencias privadas
cargo doc --document-private-items
```

## Workflows Optimizados

### Desarrollo Diario
```bash
# Verificar código
cargo check

# Formatear
cargo fmt

# Linting
cargo clippy

# Tests
cargo test

# Build final
cargo build --release
```

### Release
```bash
# Verificación completa
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release

# Crear release
cargo build --release
```

### Debug y Troubleshooting
```bash
# Build con debug symbols
cargo build

# Ejecutar con backtrace (servidor HTTP)
RUST_BACKTRACE=1 cargo run --bin server_http

# Ejecutar CLI con backtrace
RUST_BACKTRACE=1 cargo run --bin trae

# Profiling del servidor HTTP
cargo build --release --bin server_http
perf record ./target/release/server_http
perf report

# Profiling del CLI
cargo build --release --bin trae
perf record ./target/release/trae --help
perf report
```

## Configuración Optimizada

### Cargo Config (~/.cargo/config.toml)
```toml
[build]
rustflags = ["-C", "target-cpu=native"]

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
debug = 1
```

### Variables de Entorno
```bash
# Optimización de compilación
export RUSTFLAGS="-C target-cpu=native"

# Backtrace completo
export RUST_BACKTRACE=full

# Configuración de JARVIX
export JARVIX_URL=http://localhost:8080
```

## Métricas de Rendimiento

### Tamaño del Binario
```bash
# Ver tamaño de binarios
ls -lh target/release/server_http
ls -lh target/release/trae

# Analizar tamaño por crate (servidor)
cargo bloat --release --bin server_http

# Analizar tamaño por crate (CLI)
cargo bloat --release --bin trae
```

### Tiempo de Compilación
```bash
# Medir tiempo de compilación completa
time cargo build --release

# Medir tiempo de compilación del servidor
time cargo build --release --bin server_http
```

### Cobertura de Código
```bash
# Instalar herramienta de cobertura
cargo install cargo-tarpaulin

# Generar reporte de cobertura completo
cargo tarpaulin --all

# Generar reporte de cobertura para binarios específicos
cargo tarpaulin --bin trae
cargo tarpaulin --bin server_http
```

## Integración con CI/CD

### GitHub Actions
```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo build --release
```

### Docker
```dockerfile
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin server_http

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/server_http /usr/local/bin/
EXPOSE 3001
CMD ["server_http"]
```

## Troubleshooting

### Errores Comunes
```bash
# Resolver conflictos de dependencias
cargo update

# Limpiar build cache
cargo clean
rm -rf target/

# Rebuild desde cero
cargo clean && cargo build --release
```

### Optimización
```bash
# Build con optimizaciones agresivas
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat" cargo build --release

# Reducir tamaño de los binarios
strip target/release/server_http
strip target/release/trae
```