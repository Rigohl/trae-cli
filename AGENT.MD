---
description: "Contexto TRAE-CLI - Herramienta de Análisis y Reparación Rust"
agent: "TRAE-CLI"
capabilities: ["code_analysis", "auto_repair", "build_optimization"]
triggers: ["cargo_build", "code_change", "error_detection"]
---

# TRAE-CLI - CONTEXTO DE CARPETA

## DESCRIPCIÓN GENERAL
Esta carpeta contiene TRAE-CLI, la herramienta de línea de comandos en Rust que se integra como subcomando de Cargo (`cargo trae`) para análisis de código, reparación automática y optimización de builds.

## ARQUITECTURA PRINCIPAL

### Componentes Core
- **Cargo Subcommand**: Extensión nativa de `cargo trae`
- **Code Analyzer**: Análisis estático de código Rust
- **Auto Repair Engine**: Corrección automática de errores
- **Build Optimizer**: Optimizaciones de compilación

### Funcionalidades Principales
- `cargo trae analyze` - Análisis de calidad de código
- `cargo trae repair` - Reparación automática de errores
- `cargo trae build` - Builds optimizados con métricas
- `cargo trae test` - Ejecución de tests con análisis

## INTEGRACIÓN CON SISTEMA

### Conexión JARVIXSERVER
- Reportes automáticos a `http://localhost:5050/trae/*`
- Métricas de build y análisis en tiempo real
- Sincronización de estado con servidor central

### Dependencias del Sistema
- **Cargo**: Sistema de builds y dependencias
- **Rustc**: Compilador Rust con versiones específicas
- **JARVIXSERVER**: Backend para métricas y coordinación

## REGLAS DE DESARROLLO

### Estándares Aplicables
- **No Mocks**: Implementaciones concretas, no simulaciones
- **Zero Warnings**: Compilación limpia sin advertencias
- **Ownership**: Manejo estricto de ownership y borrowing
- **Error Handling**: Result<T,E> y Option explícitos

### Patrones de Código
- `Result<T, E>` para operaciones que pueden fallar
- `?` operator para propagación de errores
- Lifetimes explícitos cuando el compilador los requiere
- `Arc<Mutex<T>>` solo cuando es estrictamente necesario

## DEPENDENCIAS PRINCIPALES

### Cargo.toml
```toml
[dependencies]
clap = "4.0"          # CLI argument parsing
tokio = "1.0"         # Async runtime
serde = { version = "1.0", features = ["derive"] }
reqwest = "0.11"      # HTTP client for JARVIXSERVER
anyhow = "1.0"        # Error handling
thiserror = "1.0"     # Custom error types
```

### Features Condicionales
```toml
[features]
default = []
analysis = ["dep:ra_ap", "dep:rustc_tools_util"]
repair = ["analysis"]
```

## TESTING Y VALIDACIÓN

### Estrategia de Tests
- Tests unitarios con datos reales
- Tests de integración con builds completas
- cargo test --all-targets para cobertura completa
- cargo test --doc para tests de documentación

### Validación de Builds
- cargo check --all-targets para verificación rápida
- cargo build --release para builds optimizados
- cargo clippy para linting adicional

## FUNCIONALIDADES AVANZADAS

### Análisis de Código
- Detección de código muerto
- Optimización de imports
- Verificación de convenciones
- Análisis de dependencias circulares

### Reparación Automática
- Corrección de errores de sintaxis
- Fix de imports faltantes
- Optimización de lifetimes
- Eliminación de warnings

### Optimización de Builds
- Paralelización inteligente
- Caching de dependencias
- Link-time optimization (LTO)
- Profile-guided optimization (PGO)

## MONITOREO Y MÉTRICAS

### Métricas Recolectadas
- Tiempo de compilación por crate
- Número de warnings/errors corregidos
- Cobertura de análisis de código
- Eficiencia de reparaciones automáticas

### Reportes a JARVIXSERVER
- Métricas de build en `/trae/api/metrics`
- Logs de análisis en `/trae/api/logs`
- Estado de reparaciones en `/trae/api/status`

## INTEGRACIÓN CON WORKERS

### C Workers
- Llamadas FFI seguras a funciones C
- Bindings generados automáticamente
- Manejo seguro de memoria

### Nim Workers
- Integración con runtime Nim
- Async HTTP con Jester
- Caching con SQLite

## DESPLIEGUE Y DISTRIBUCIÓN

### Instalación como Cargo Subcommand
```bash
cargo install --git https://github.com/usuario/trae-cli.git
# Esto instala `cargo-trae` en el PATH
```

### Configuración
- Archivo de configuración en `~/.config/trae-cli/config.toml`
- Variables de entorno para JARVIXSERVER_URL
- Flags de línea de comandos para opciones específicas

## CONTEXTO DE DESARROLLO

TRAE-CLI es la herramienta principal para desarrollo Rust en el ecosistema CLI. Se integra profundamente con Cargo y proporciona automatización inteligente para mantener la calidad del código.

### Puntos de Contacto
- **JARVIXSERVER**: Backend central para métricas
- **Workers**: Extensibilidad con C/Nim
- **Cargo**: Integración nativa como subcommand

### Consideraciones de Performance
- Análisis incremental para grandes codebases
- Paralelización de tareas de análisis
- Caching inteligente de resultados
- Optimización de memoria para proyectos grandes
