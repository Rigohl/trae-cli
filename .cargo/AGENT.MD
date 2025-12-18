---
description: "Contexto .cargo - Configuración y Cache de Cargo"
agent: "Cargo_Config"
capabilities: ["dependency_resolution", "build_cache", "registry_management"]
triggers: ["dependency_update", "build_optimization", "cache_management"]
---

# .CARGO - CONTEXTO DE CARPETA

## DESCRIPCIÓN GENERAL
Esta carpeta contiene la configuración y caché de Cargo, el gestor de paquetes de Rust. Gestiona dependencias, compilación y optimización del proyecto TRAE-CLI.

## ARQUITECTURA CARGO

### Estructura de .cargo
```
.cargo/
├── config.toml        # Configuración global de Cargo
├── registry/          # Cache del registro de crates
│   ├── index/        # Índice de crates disponibles
│   └── cache/        # Crates descargadas
├── git/              # Cache de dependencias Git
│   └── db/          # Repositorios Git clonados
└── target/           # Artefactos de compilación (si no redirigido)
```

## CONFIGURACIÓN CARGO

### Config.toml Principal
```toml
# .cargo/config.toml - Configuración TRAE-CLI
[build]
# Optimizaciones de compilación
rustflags = ["-C", "target-cpu=native", "-C", "opt-level=3"]
# Paralelización máxima
jobs = 8

[net]
# Configuración de red para registry
retry = 3
timeout = 30

[registry]
# Registry personalizado si aplica
index = "https://github.com/rust-lang/crates.io-index"

[term]
# Output verbose para debugging
verbose = true
color = "always"

[source.crates-io]
# Mirrors para crates.io
replace-with = "ustc"

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

### Configuración de Features
```toml
# Configuración condicional por features
[build.rustflags]
default = ["-C", "opt-level=2"]
release = ["-C", "opt-level=3", "-C", "lto=fat", "-C", "codegen-units=1"]
dev = ["-C", "opt-level=0", "-C", "debuginfo=2"]

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-feature=+avx2"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "target-feature=+neon"]
```

## GESTIÓN DE DEPENDENCIAS

### Registry Cache
```bash
# Ver estado del registry
cargo tree --duplicates
cargo outdated

# Limpiar cache obsoleto
cargo cache --autoclean

# Verificar integridad
cargo cache --info
```

### Git Dependencies
```toml
# Cargo.toml - Dependencias Git
[dependencies]
some-crate = { git = "https://github.com/user/repo.git", rev = "commit-hash" }
another-crate = { git = "https://github.com/user/repo.git", tag = "v1.0.0" }
```

### Gestión de Cache Git
```bash
# Limpiar repositorios Git no usados
rm -rf .cargo/git/db/*

# Ver repositorios cacheados
ls .cargo/git/db/
```

## OPTIMIZACIONES DE BUILD

### Target Directory
```bash
# Usar target directory separado para evitar locks
export CARGO_TARGET_DIR=/tmp/cargo-target
cargo build

# O configurar globalmente
echo 'export CARGO_TARGET_DIR=/tmp/cargo-target' >> ~/.bashrc
```

### Build Caching
```toml
# Configuración de sccache para builds más rápidos
[build]
rustc-wrapper = "sccache"

# Instalar sccache
cargo install sccache
```

### Link Time Optimization (LTO)
```toml
# Cargo.toml - LTO para binarios optimizados
[profile.release]
lto = "fat"          # LTO completa
codegen-units = 1   # Un solo codegen unit
opt-level = 3       # Máxima optimización
strip = true        # Remover símbolos de debug
panic = "abort"     # Pánico aborta (más pequeño)
```

## REGISTRY MANAGEMENT

### Mirrors y Caching
```toml
# .cargo/config.toml - Mirrors
[source.crates-io]
replace-with = "tuna"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```

### Offline Builds
```bash
# Crear vendor directory para builds offline
cargo vendor > .cargo/vendor-config.toml

# Build offline
cargo build --offline --config .cargo/vendor-config.toml
```

## DEPENDENCY RESOLUTION

### Resolver Conflictos
```bash
# Ver árbol de dependencias
cargo tree

# Resolver versiones específicas
cargo update -p crate-name --precise 1.2.3

# Verificar compatibilidad
cargo check --all-targets
```

### Audit de Seguridad
```bash
# Instalar cargo-audit
cargo install cargo-audit

# Auditar dependencias
cargo audit

# Verificar licencias
cargo install cargo-license
cargo license
```

## TESTING Y CI/CD

### Configuración de Tests
```toml
# Cargo.toml - Configuración de tests
[profile.test]
opt-level = 1
debug = true
overflow-checks = true

[profile.bench]
opt-level = 3
debug = false
lto = "fat"
```

### CI/CD con Cargo
```yaml
# .github/workflows/ci.yml
- name: Cache Cargo registry
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

- name: Build
  run: cargo build --release

- name: Test
  run: cargo test --all-targets
```

## TROUBLESHOOTING

### Problemas Comunes

#### Registry Timeout
```bash
# Aumentar timeout
export CARGO_HTTP_TIMEOUT=300
cargo update
```

#### Build Locks
```bash
# Remover locks
rm -f .cargo/registry/index/.cargo-lock
rm -f target/debug/.cargo-lock
```

#### Disk Space
```bash
# Limpiar cache agresivamente
cargo cache --autoclean-expensive
du -sh .cargo/
```

## INTEGRACIÓN CON TRAE-CLI

### Comandos TRAE-CLI
```bash
# Análisis de dependencias
trae analyze --dependencies

# Build optimizado
trae build --release --lto

# Verificar cache
trae cache --status
```

### Configuración TRAE-CLI
```toml
# Cargo.toml - Metadata TRAE-CLI
[package.metadata.trae]
cache_strategy = "aggressive"
lto_enabled = true
registry_mirror = "tuna"
```

## CONTEXTO DE DESARROLLO

La carpeta .cargo es crítica para el desarrollo eficiente de TRAE-CLI. Gestiona todas las dependencias, optimizaciones de build y caching para mantener builds rápidos y confiables.

### Puntos de Contacto
- **Cargo.toml**: Define dependencias y configuración del proyecto
- **src/**: Código fuente que usa las dependencias cacheadas
- **target/**: Artefactos de compilación generados

### Mejores Prácticas
- Mantener Cargo.lock en version control
- Usar mirrors para builds más rápidos
- Configurar LTO para releases de producción
- Limpiar cache periódicamente para ahorrar espacio
- Usar cargo-audit para seguridad de dependencias
