# JARVIX CLI - Referencia de Comandos

Guía completa de todos los comandos disponibles en JARVIX CLI.

## 🚀 Comandos Principales de Cargo

JARVIX CLI proporciona acceso directo a todos los comandos estándar de Cargo:

### Gestión de Proyectos
```bash
# Verificar código sin compilar
trae check
trae c  # alias corto

# Compilar proyecto
trae build
trae b  # alias corto

# Compilar en modo release
trae build --release

# Ejecutar tests
trae test
trae t  # alias corto

# Ejecutar binario
trae run
trae r  # alias corto

# Crear nuevo proyecto
trae new mi-proyecto

# Inicializar proyecto en directorio actual
trae init
```

### Gestión de Dependencias
```bash
# Agregar dependencias
trae add serde
trae add tokio --features full

# Remover dependencias
trae remove serde

# Actualizar dependencias
trae update

# Visualizar árbol de dependencias
trae tree
```

### Herramientas de Desarrollo
```bash
# Formatear código
trae fmt

# Análisis estático de código
trae clippy
trae lint  # alias

# Análisis estricto
trae clippy -- -D warnings

# Generar documentación
trae doc

# Abrir documentación en navegador
trae doc --open

# Limpiar artifacts compilados
trae clean
```

### Instalación de Binarios
```bash
# Instalar binario desde crates.io
trae install nombre-crate

# Instalar desde path local
trae install --path .

# Desinstalar binario
trae uninstall nombre-crate
```

### Benchmarks y Búsqueda
```bash
# Ejecutar benchmarks
trae bench

# Buscar crates en crates.io
trae search tokio
```

## 🔧 Super Comandos de JARVIX

Comandos especiales que combinan múltiples operaciones:

### Preflight Check
```bash
# Verifica todo antes de subir cambios
# Ejecuta: fmt + clippy + test + build
trae preflight

# Ver output detallado
trae preflight --verbose
```

### Auto Repair
```bash
# Intenta arreglar todo automáticamente
# Ejecuta: clippy --fix + fmt + optimizaciones
trae repair

# Modo dry-run (sin hacer cambios)
trae repair --dry-run

# Auto repair específico
trae repair --fmt      # Solo formato
trae repair --clippy   # Solo clippy fix
trae repair --deps     # Arreglar dependencias
```

## 🔍 Análisis y Detección

### Dead Code Analysis
```bash
# Detectar código muerto no utilizado
trae deadcode

# Con detalles verbosos
trae deadcode --verbose
```

### Module Analysis
```bash
# Analizar módulos no utilizados
trae modules
```

## 🌐 Integración con JARVIXSERVER

### Web Search
```bash
# Buscar información en internet
trae web-search "rust async programming"

# Configurar URL de JARVIXSERVER
trae --jarvix http://localhost:8080 web-search "query"
```

### Configuración
```bash
# Variables de entorno
export JARVIX_URL=http://localhost:8080

# Ejecutar sin reportar a JARVIXSERVER
trae --no-report build

# Especificar proyecto diferente
trae --project /ruta/proyecto build
```

## 📦 Comandos Personalizados

### Custom Cargo Command
```bash
# Ejecutar cualquier comando cargo personalizado
trae custom nombre-comando [args...]
```

## 🎯 Opciones Globales

```bash
Options:
  --jarvix <JARVIX>    URL del servidor JARVIXSERVER 
                       [env: JARVIX_URL=] 
                       [default: http://localhost:8080]
  
  --project <PROJECT>  Ruta del proyecto Rust a ejecutar 
                       [default: .]
  
  --no-report          No reportar resultado a JARVIXSERVER
  
  -v, --verbose        Mostrar output detallado
  
  -h, --help           Mostrar ayuda
  
  -V, --version        Mostrar versión
```

## 📝 Ejemplos de Uso

### Flujo de Desarrollo Típico
```bash
# 1. Verificar y formatear código
trae fmt
trae clippy

# 2. Ejecutar tests
trae test

# 3. Build de release
trae build --release
```

### Pipeline Completo
```bash
# Ejecutar preflight antes de commit
trae preflight

# Si hay problemas, intentar auto-reparación
trae repair

# Verificar nuevamente
trae preflight
```

### Desarrollo con JARVIXSERVER
```bash
# Iniciar desarrollo con integración JARVIX
export JARVIX_URL=http://localhost:8080

# Ejecutar comandos con reporting automático
trae build
trae test
trae clippy

# Buscar ayuda online
trae web-search "how to use tokio runtime"
```

## 🔧 Servidor HTTP

JARVIX CLI incluye un servidor HTTP para integración con JARVIXSERVER:

```bash
# Build del servidor
cargo build --release --bin server_http

# Ejecutar servidor
./target/release/server_http

# El servidor estará disponible en:
# http://localhost:3001
```

## 🚀 CI/CD

### Comandos para CI
```bash
# Verificar formato
cargo fmt -- --check

# Linting estricto
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all

# Build release
cargo build --release
```

### GitHub Actions Example
```yaml
- name: Run JARVIX checks
  run: |
    cargo run --bin trae -- preflight
```

## 📚 Ayuda Adicional

```bash
# Ver ayuda general
trae --help

# Ver ayuda de comando específico
trae build --help
trae repair --help
trae clippy --help
```

---

**Para más información sobre integración y comandos avanzados, consulta:**
- [README.md](./README.md) - Introducción y guía rápida
- [CARGO_COMMANDS.md](./CARGO_COMMANDS.md) - Comandos cargo optimizados
- [INTEGRATION.md](./INTEGRATION.md) - Guía de integración con JARVIXSERVER
