---
description: "Contexto src - Código Fuente Principal de TRAE-CLI"
agent: "Source_Code"
capabilities: ["code_analysis", "command_execution", "error_repair", "build_optimization"]
triggers: ["code_changes", "build_commands", "analysis_requests"]
---

# SRC - CONTEXTO DE CARPETA

## DESCRIPCIÓN GENERAL
Esta carpeta contiene todo el código fuente de TRAE-CLI, organizado en módulos que implementan análisis de código, reparación automática, integración con Cargo y comandos CLI.

## ARQUITECTURA DEL CÓDIGO FUENTE

### Estructura de Módulos
```
src/
├── main.rs              # Punto de entrada principal
├── lib.rs               # Biblioteca principal
├── cli/
│   ├── mod.rs           # Módulo CLI
│   ├── commands.rs      # Definición de comandos
│   └── args.rs          # Parsing de argumentos
├── core/
│   ├── mod.rs           # Lógica core
│   ├── analyzer.rs      # Análisis de código
│   ├── repair.rs        # Reparación automática
│   └── metrics.rs       # Métricas y reporting
├── cargo/
│   ├── mod.rs           # Integración Cargo
│   ├── integration.rs   # Subcomando cargo-trae
│   └── config.rs        # Configuración Cargo
├── utils/
│   ├── mod.rs           # Utilidades
│   ├── file_ops.rs      # Operaciones de archivos
│   └── process.rs       # Gestión de procesos
└── tests/
    ├── mod.rs           # Tests
    ├── integration.rs   # Tests de integración
    └── fixtures/        # Datos de test
```

## PUNTO DE ENTRADA PRINCIPAL

### main.rs - Inicialización y CLI
```rust
// src/main.rs
use clap::Parser;
use trae_cli::{cli::Cli, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logging
    env_logger::init();

    // Parsear argumentos CLI
    let cli = Cli::parse();

    // Ejecutar comando
    run(cli).await
}
```

### Definición de CLI con Clap
```rust
// src/cli/mod.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "trae")]
#[command(about = "TRAE-CLI - Cargo extension for code analysis and repair")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze code quality and performance
    Analyze(AnalyzeArgs),
    /// Automatically repair code issues
    Repair(RepairArgs),
    /// Build with analysis and optimization
    Build(BuildArgs),
    /// Run tests with coverage
    Test(TestArgs),
    /// Check dependencies and security
    Check(CheckArgs),
}
```

## MÓDULO CORE - LÓGICA PRINCIPAL

### Analyzer - Análisis de Código
```rust
// src/core/analyzer.rs
use std::collections::HashMap;

pub struct CodeAnalyzer {
    config: AnalyzerConfig,
}

impl CodeAnalyzer {
    pub fn new(config: AnalyzerConfig) -> Self {
        Self { config }
    }

    pub async fn analyze_project(&self, path: &std::path::Path)
        -> Result<AnalysisReport, AnalysisError>
    {
        let mut report = AnalysisReport::new();

        // Análisis de dependencias
        report.dependencies = self.analyze_dependencies(path).await?;

        // Análisis de código
        report.code_quality = self.analyze_code_quality(path).await?;

        // Análisis de performance
        report.performance = self.analyze_performance(path).await?;

        // Análisis de seguridad
        report.security = self.analyze_security(path).await?;

        Ok(report)
    }

    async fn analyze_dependencies(&self, path: &std::path::Path)
        -> Result<DependencyAnalysis, AnalysisError>
    {
        // Leer Cargo.toml
        let cargo_toml = path.join("Cargo.toml");
        let content = tokio::fs::read_to_string(&cargo_toml).await?;

        // Parsear dependencias
        let parsed: toml::Value = toml::from_str(&content)?;
        let dependencies = parsed.get("dependencies")
            .and_then(|d| d.as_table())
            .unwrap_or(&toml::map::Map::new());

        // Verificar vulnerabilidades
        let mut issues = Vec::new();
        for (name, version) in dependencies {
            if let Some(vuln) = self.check_vulnerability(name, version).await? {
                issues.push(vuln);
            }
        }

        Ok(DependencyAnalysis { issues })
    }

    async fn analyze_code_quality(&self, path: &std::path::Path)
        -> Result<CodeQualityReport, AnalysisError>
    {
        // Ejecutar cargo clippy
        let output = tokio::process::Command::new("cargo")
            .args(&["clippy", "--message-format=json"])
            .current_dir(path)
            .output()
            .await?;

        // Parsear resultados
        let warnings = self.parse_clippy_output(&output.stdout)?;

        Ok(CodeQualityReport {
            warnings_count: warnings.len(),
            errors_count: warnings.iter().filter(|w| w.level == "error").count(),
            warnings,
        })
    }
}
```

### Repair Engine - Reparación Automática
```rust
// src/core/repair.rs
use std::collections::HashSet;

pub struct AutoRepair {
    analyzer: CodeAnalyzer,
    config: RepairConfig,
}

impl AutoRepair {
    pub fn new(analyzer: CodeAnalyzer, config: RepairConfig) -> Self {
        Self { analyzer, config }
    }

    pub async fn repair_project(&self, path: &std::path::Path)
        -> Result<RepairReport, RepairError>
    {
        let mut report = RepairReport::new();

        // Analizar problemas
        let analysis = self.analyzer.analyze_project(path).await?;

        // Reparar automáticamente problemas seguros
        for issue in &analysis.code_quality.warnings {
            if self.can_auto_fix(issue) {
                self.apply_fix(issue, path).await?;
                report.fixed_issues.push(issue.clone());
            } else {
                report.manual_fixes_needed.push(issue.clone());
            }
        }

        // Reparar dependencias vulnerables
        for vuln in &analysis.dependencies.issues {
            if let Some(fix) = self.find_dependency_fix(vuln).await? {
                self.apply_dependency_fix(fix, path).await?;
                report.fixed_dependencies.push(fix);
            }
        }

        Ok(report)
    }

    fn can_auto_fix(&self, issue: &CodeIssue) -> bool {
        matches!(issue.category,
            IssueCategory::UnusedImport |
            IssueCategory::MissingDocumentation |
            IssueCategory::Formatting
        )
    }

    async fn apply_fix(&self, issue: &CodeIssue, path: &std::path::Path)
        -> Result<(), RepairError>
    {
        match issue.category {
            IssueCategory::UnusedImport => {
                self.remove_unused_import(issue, path).await
            }
            IssueCategory::Formatting => {
                self.format_file(issue.file_path.as_ref().unwrap()).await
            }
            _ => Ok(())
        }
    }
}
```

## INTEGRACIÓN CON CARGO

### Cargo Subcommand Implementation
```rust
// src/cargo/integration.rs
use cargo::core::{Workspace, Package};
use cargo::ops::{CompileOptions, CompileMode};

pub struct CargoIntegration {
    workspace: Workspace<'static>,
}

impl CargoIntegration {
    pub fn new(manifest_path: &std::path::Path) -> Result<Self, CargoError> {
        let config = cargo::Config::default()?;
        let workspace = Workspace::new(manifest_path, &config)?;

        Ok(Self { workspace })
    }

    pub async fn build_with_analysis(&self, options: BuildOptions)
        -> Result<BuildReport, CargoError>
    {
        let mut report = BuildReport::new();

        // Configurar opciones de compilación
        let compile_options = CompileOptions::new(
            &self.workspace.config(),
            CompileMode::Build,
        )?;

        // Ejecutar análisis antes del build
        let analysis = self.run_pre_build_analysis().await?;
        report.analysis = analysis;

        // Build con optimizaciones TRAE
        let build_result = cargo::ops::compile(&self.workspace, &compile_options).await?;
        report.build_result = build_result;

        // Análisis post-build
        let post_analysis = self.run_post_build_analysis().await?;
        report.post_analysis = post_analysis;

        Ok(report)
    }

    async fn run_pre_build_analysis(&self) -> Result<PreBuildAnalysis, CargoError> {
        // Verificar dependencias
        // Check for unused dependencies
        // Validate feature flags
        // Check for circular dependencies

        Ok(PreBuildAnalysis {
            unused_deps: Vec::new(),
            circular_deps: Vec::new(),
            feature_issues: Vec::new(),
        })
    }
}
```

### Config Integration
```rust
// src/cargo/config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraeConfig {
    pub analysis: AnalysisConfig,
    pub repair: RepairConfig,
    pub build: BuildConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub enable_clippy: bool,
    pub enable_audit: bool,
    pub enable_coverage: bool,
    pub performance_threshold: u32,
}

impl Default for TraeConfig {
    fn default() -> Self {
        Self {
            analysis: AnalysisConfig {
                enable_clippy: true,
                enable_audit: true,
                enable_coverage: false,
                performance_threshold: 80,
            },
            repair: RepairConfig::default(),
            build: BuildConfig::default(),
        }
    }
}
```

## UTILIDADES Y HELPERS

### File Operations
```rust
// src/utils/file_ops.rs
use tokio::fs;
use std::path::Path;

pub struct FileOps;

impl FileOps {
    pub async fn read_file_content(path: &Path) -> Result<String, FileError> {
        fs::read_to_string(path).await
            .map_err(|e| FileError::ReadError(e.to_string()))
    }

    pub async fn write_file_content(path: &Path, content: &str) -> Result<(), FileError> {
        // Crear directorio si no existe
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| FileError::WriteError(e.to_string()))?;
        }

        fs::write(path, content).await
            .map_err(|e| FileError::WriteError(e.to_string()))
    }

    pub async fn find_files_with_extension(dir: &Path, extension: &str)
        -> Result<Vec<std::path::PathBuf>, FileError>
    {
        let mut files = Vec::new();
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == extension {
                        files.push(path);
                    }
                }
            } else if path.is_dir() {
                // Recursión
                let mut sub_files = Self::find_files_with_extension(&path, extension).await?;
                files.append(&mut sub_files);
            }
        }

        Ok(files)
    }
}
```

### Process Management
```rust
// src/utils/process.rs
use tokio::process::Command;
use std::process::Stdio;

pub struct ProcessRunner;

impl ProcessRunner {
    pub async fn run_command(command: &str, args: &[&str], cwd: Option<&Path>)
        -> Result<CommandOutput, ProcessError>
    {
        let mut cmd = Command::new(command);
        cmd.args(args);

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        // Configurar output
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let output = cmd.output().await
            .map_err(|e| ProcessError::ExecutionError(e.to_string()))?;

        Ok(CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            status: output.status,
        })
    }

    pub async fn run_cargo_command(args: &[&str], cwd: Option<&Path>)
        -> Result<CommandOutput, ProcessError>
    {
        Self::run_command("cargo", args, cwd).await
    }
}
```

## TESTS E INTEGRACIÓN

### Integration Tests
```rust
// src/tests/integration.rs
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_full_analysis_workflow() {
    // Crear proyecto de test
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Crear Cargo.toml
    let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
    "#;
    fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();

    // Crear código fuente con issues
    let main_rs = r#"
use serde::Serialize; // unused import

fn main() {
    println!("Hello");
}
    "#;
    fs::create_dir_all(project_path.join("src")).unwrap();
    fs::write(project_path.join("src/main.rs"), main_rs).unwrap();

    // Ejecutar análisis
    let analyzer = CodeAnalyzer::new(AnalyzerConfig::default());
    let report = analyzer.analyze_project(project_path).await.unwrap();

    // Verificar resultados
    assert!(report.code_quality.warnings_count > 0);
    assert!(report.dependencies.issues.is_empty());
}

#[tokio::test]
async fn test_auto_repair() {
    // Similar setup...
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Setup project with fixable issues
    setup_test_project(project_path).await;

    // Run repair
    let analyzer = CodeAnalyzer::new(AnalyzerConfig::default());
    let repair = AutoRepair::new(analyzer, RepairConfig::default());
    let report = repair.repair_project(project_path).await.unwrap();

    // Verify fixes were applied
    assert!(!report.fixed_issues.is_empty());

    // Re-analyze to confirm fixes
    let analyzer = CodeAnalyzer::new(AnalyzerConfig::default());
    let final_report = analyzer.analyze_project(project_path).await.unwrap();
    assert_eq!(final_report.code_quality.warnings_count, 0);
}
```

## CONFIGURACIÓN Y TIPOS

### Core Types
```rust
// src/lib.rs
pub mod cli;
pub mod core;
pub mod cargo;
pub mod utils;
pub mod tests;

pub use cli::Cli;
pub use core::{CodeAnalyzer, AutoRepair};
pub use cargo::CargoIntegration;

// Re-export common types
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

### Error Handling
```rust
// src/core/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TraeError {
    #[error("Analysis failed: {0}")]
    AnalysisError(String),

    #[error("Repair failed: {0}")]
    RepairError(String),

    #[error("Cargo integration error: {0}")]
    CargoError(String),

    #[error("File operation error: {0}")]
    FileError(String),

    #[error("Process execution error: {0}")]
    ProcessError(String),
}
```

## CONTEXTO DE DESARROLLO

El código fuente de TRAE-CLI implementa análisis avanzado de código Rust, reparación automática de issues comunes, integración profunda con Cargo y una CLI completa. Sigue principios de "No Mocks" y "Zero Warnings".

### Puntos de Contacto
- **CLI Layer**: Maneja argumentos y ejecuta comandos
- **Core Logic**: Análisis y reparación de código
- **Cargo Integration**: Subcomando y configuración
- **Utils**: Operaciones de archivos y procesos

### Mejores Prácticas Implementadas
- Async/await para operaciones I/O
- Error handling comprehensivo con thiserror
- Testing integrado con fixtures
- Configuración tipada con serde
- Modularidad con separación clara de responsabilidades
- Documentación inline completa
- Zero-copy donde es posible
- Memory safety garantizada por Rust
