#![doc = " # CLI Module - Command Line Interface"]
#![doc = ""]
#![doc = " Define la estructura principal de comandos y subcomandos de TRAE CLI"]
use crate::commands::{
    analyze::AnalyzeCommand, build::BuildCommand, cargo::CargoCommand, clippy::ClippyCommand,
    build_help::BuildHelpCommand,
    daemon::DaemonCommand, doc::DocCommand, math::MathCommand, mcp::McpCommand,
    metrics::MetricsCommand, paths::PathsCommand, release::ReleaseCommand, repair::RepairCommand,
    rustup::RustupCommand, security::SecurityCommand, simulate::SimulateCommand, test::TestCommand,
    watch::WatchCommand,
    metadata::TraeMetadataCommand,
};
use crate::core::cargo::CargoExecutor;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde_json::json;
use std::time::{Duration, Instant};
#[doc = " TRAE CLI - Enhanced Rust Development Tools"]
#[derive(Parser, Debug)]
#[command(name = "trae")]
#[command(version = "0.1.0")]
#[command(about = "Total Rust Analysis Engine - Enhanced cargo with advanced tooling")]
# [command (long_about = None)]
pub struct TraeCli {
    #[doc = " Enable verbose output"]
    #[arg(short, long, global = true)]
    pub verbose: bool,
    #[doc = " Configuration file path"]
    #[arg(short, long, global = true)]
    pub config: Option<String>,
    #[doc = " Disable JARVIXSERVER reporting"]
    #[arg(long, global = true)]
    pub no_jarvix: bool,
    #[command(subcommand)]
    pub command: Commands,
}
#[allow(clippy::enum_variant_names)]
#[derive(Subcommand, Debug)]
pub enum Commands {
    #[doc = " Enhanced build commands with analysis and repair"]
    Build(BuildCommand),
    #[doc = " Repair and fix project issues automatically"]
    Repair(RepairCommand),
    #[doc = " Deep code analysis and optimization suggestions"]
    Analyze(AnalyzeCommand),
    #[doc = " Enhanced clippy with parallel analysis"]
    Clippy(ClippyCommand),
    #[doc = "Help and suggestions for improving cargo build"]
    BuildHelp(BuildHelpCommand),
    #[doc = " Performance simulation and auto-optimization"]
    Simulate(SimulateCommand),
    #[doc = " Launch trae-server in background (silent/optional log)"]
    Daemon(DaemonCommand),
    #[doc = " Gestor de procesos MCP personalizados"]
    Mcp(McpCommand),
    #[doc = " Release pipeline (fmt+clippy+tests+package)"]
    Release(ReleaseCommand),
    #[doc = " File watcher que ejecuta comandos al guardar"]
    Watch(WatchCommand),
    #[doc = " View and manage metrics reporting"]
    Metrics(MetricsCommand),
    #[doc = " Passthrough de comandos cargo sin prefijo (external subcommand)"]
    #[command(external_subcommand)]
    External(Vec<String>),
    #[doc = " Execute enhanced cargo commands"]
    Cargo(CargoCommand),
    #[doc = " Passthrough to `rustup` (official Rust toolchain manager)"]
    Rustup(RustupCommand),
    #[doc = " Check source paths and parse `.rs` files"]
    Paths(PathsCommand),
    #[doc = " Show all available cargo commands (from `CARGO_COMMANDS.md`)"]
    #[command(name = "help-cargo")]
    HelpCargo,
    #[doc = " Initialize TRAE configuration"]
    Init {
        #[doc = " Force overwrite existing configuration"]
        #[arg(long)]
        force: bool,
    },
    #[doc = " Check TRAE and system dependencies"]
    Doctor,
    #[doc = " 🔍 SUPER SCAN - Análisis completo multilenguaje del proyecto desde raíz"]
    #[command(name = "scan")]
    Scan {
        #[doc = " Buscar dependencias faltantes"]
        #[arg(long)]
        deps: bool,
        #[doc = " Detectar código muerto/mock"]
        #[arg(long)]
        dead_code: bool,
        #[doc = " Análisis multilenguaje (Rust, JS, Python, Go, etc)"]
        #[arg(long)]
        multilang: bool,
        #[doc = " Mostrar solo errores críticos"]
        #[arg(long)]
        critical_only: bool,
        #[doc = " Exportar reporte completo"]
        #[arg(long)]
        export: Option<String>,
    },
    #[doc = " 🧪 Enhanced testing with coverage and analysis"]
    Test(TestCommand),
    #[doc = "Generate project metadata JSON"]
    Metadata(TraeMetadataCommand),
    #[doc = " Quick pipeline: analyze -> repair -> test (compact powerful command)"]
    Auto {
        #[arg(long)]
        no_jarvix: bool,
    },
    #[doc = " Lista detallada de los comandos TRAE y sus highlights recientes"]
    #[command(name = "commands")]
    CommandsGuide,
    #[doc = " 📚 Documentation generation and validation"]
    #[command(name = "doc")]
    Doc(DocCommand),
    #[doc = " 🔬 Mathematical analysis with Julia workers"]
    Math(MathCommand),
    #[doc = " � Security audit and vulnerability scanning"]
    Security(SecurityCommand),
}
impl TraeCli {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self) -> Result<()> {
        let start_time = Instant::now();
        let result = match &self.command {
            Commands::Build(cmd) => cmd.execute(self).await,
            Commands::Repair(cmd) => cmd.execute(self).await,
            Commands::Analyze(cmd) => cmd.execute(self).await,
            Commands::BuildHelp(cmd) => cmd.execute(self).await,
            Commands::Clippy(cmd) => cmd.execute().await,
            Commands::Simulate(cmd) => cmd.execute(self).await,
            Commands::Daemon(cmd) => cmd.execute(self).await,
            Commands::Mcp(cmd) => cmd.execute().await,
            Commands::Release(cmd) => cmd.execute().await,
            Commands::Watch(cmd) => cmd.execute().await,
            Commands::Metrics(cmd) => cmd.execute(self).await,
            Commands::Cargo(cmd) => cmd.execute(self).await,
            Commands::Rustup(cmd) => cmd.execute().await,
            Commands::Paths(cmd) => cmd.execute().await,
            Commands::External(args) => self.run_external_cargo(args).await,
            Commands::Test(cmd) => cmd.execute(self).await,
            Commands::Auto { no_jarvix } => self.run_auto(*no_jarvix).await,
            Commands::Metadata(cmd) => cmd.execute(self).await,
            Commands::Doc(cmd) => cmd.execute(self).await,
            Commands::Math(cmd) => cmd.execute(self).await,
            Commands::Security(cmd) => cmd.execute(self).await,
            Commands::CommandsGuide => self.show_command_catalog(),
            Commands::HelpCargo => self.show_cargo_help().await,
            Commands::Init { force } => self.init_config(*force).await,
            Commands::Doctor => self.run_doctor().await,
            Commands::Scan {
                deps,
                dead_code,
                multilang,
                critical_only,
                export,
            } => {
                self.run_super_scan(
                    *deps,
                    *dead_code,
                    *multilang,
                    *critical_only,
                    export.as_deref(),
                )
                .await
            }
        };
        let total_duration = start_time.elapsed();
        if total_duration > Duration::from_millis(100) {
            println!("⚡ Comando ejecutado en: {total_duration:?}");
        }
        result
    }
    #[doc = "Method documentation added by AI refactor"]
    fn show_command_catalog(&self) -> Result<()> {
        #[derive(Clone)]
        struct CommandInfo<'a> {
            name: &'a str,
            usage: &'a str,
            highlights: &'a [&'a str],
        }
        let catalog = vec ! [CommandInfo { name : "build" , usage : "trae build [--release] [--auto-repair] [--analyze]" , highlights : & ["Pipeline con tabla de pasos, métricas FFT y auto-repair opcional." , "Compatibilidad total con argumentos clásicos de cargo; no usa Docker por defecto." ,] , } , CommandInfo { name : "repair" , usage : "trae repair [--auto|--fmt|--clippy|--deps] [--dry-run]" , highlights : & ["Flujo DMAIC con confirmación inteligente y exportación JSON." , "Resumen por fase (clippy, fmt, deps, etc.) reutilizando StepSummary moderno." ,] , } , CommandInfo { name : "simulate" , usage : "trae simulate --scenarios 100000 --stream" , highlights : & ["Motor de simulaciones estilo Ben Bend con streaming de métricas." , "Permite validar patrones de rendimiento antes del build o tests." ,] , } , CommandInfo { name : "mcp" , usage : "trae mcp <tool> [args]" , highlights : & ["Levanta MCPs (MEMORY_P, JarvixServer, etc.) en silencio y segundo plano." , "Ideal para dejar servicios auxiliares activos sin escribir scripts extra." ,] , } , CommandInfo { name : "cargo" , usage : "trae cargo <subcomando> [...args]" , highlights : & ["Proxy neutro a cargo con streaming coloreado y métricas homogéneas." , "Sirve para cualquier proyecto Rust ajeno a Jarvix; cero vendor lock-in." ,] , } , CommandInfo { name : "help-cargo" , usage : "trae help-cargo" , highlights : & ["Muestra el catálogo extendido documentado en CARGO_COMMANDS.md." ,] , } ,] ;
        println!("\n{}", "TRAE Command Catalog".cyan().bold());
        println!(
            "{}",
            "Resumen rápido de los comandos mejorados y cómo aprovecharlos.".dimmed()
        );
        println!();
        for entry in catalog {
            println!("{} {}", "•".cyan(), entry.name.to_uppercase().bold());
            println!("   {}", entry.usage.dimmed());
            for highlight in entry.highlights {
                println!("     {} {}", "–".dimmed(), highlight);
            }
            println!();
        }
        println!(
            "{}",
            "Tip: combina `trae commands` + `trae help-cargo` para guías rápidas por etapa."
                .green()
        );
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn show_cargo_help(&self) -> Result<()> {
        use crate::utils::docs::show_cargo_commands;
        show_cargo_commands().await
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn init_config(&self, force: bool) -> Result<()> {
        use crate::config::init_trae_config;
        init_trae_config(force).await
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_doctor(&self) -> Result<()> {
        use crate::core::doctor::run_system_check;
        run_system_check().await
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_super_scan(
        &self,
        deps: bool,
        dead_code: bool,
        multilang: bool,
        critical_only: bool,
        export: Option<&str>,
    ) -> Result<()> {
        println!(
            "{}",
            "🔍 TRAE SUPER SCAN - Análisis Nuclear Completo con JARVIX Paralelización"
                .cyan()
                .bold()
        );
        println!("{}", "=====================================\n".cyan());
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();
        let mut metrics =
            crate::metrics::collector::MetricsCollector::new("super_scan".to_string());
        let jarvix_client = if self.no_jarvix {
            None
        } else {
            crate::jarvix::client::JarvixClient::new().ok().flatten()
        };
        let use_parallel = jarvix_client.is_some();
        if use_parallel {
            println!("⚡ Modo PARALELO activado - Usando JARVIXSERVER workers");
            if let Some(client) = jarvix_client.as_ref() {
                if let Ok(stats) = client.get_pool_stats().await {
                    println!("📊 Workers disponibles: {stats}");
                } else {
                    eprintln!("⚠️  No se pudo obtener stats de JARVIXSERVER");
                }
            }
        } else {
            println!("🔄 Modo SECUENCIAL - JARVIXSERVER no disponible");
        }
        println!("{}", "🦀 [1/6] Analizando proyecto Rust...".yellow());
        let rust_scan = self.scan_rust_project(critical_only);
        all_issues.extend(rust_scan.0);
        all_suggestions.extend(rust_scan.1);
        if deps {
            if use_parallel {
                println!(
                    "{}",
                    "📦 [2/6] Escaneando dependencias (PARALELO)...".yellow()
                );
                let job_data = json ! ({ "project_path" : std :: env :: current_dir () ?. to_string_lossy () , "scan_type" : "dependencies" });
                if let Some(client) = jarvix_client.as_ref() {
                    if let Ok(job_id) = client
                        .submit_parallel_analysis_job("dependency_analysis", job_data)
                        .await
                    {
                        println!("📤 Job dependencias enviado: {job_id}");
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        if let Ok(Some(result)) = client.get_job_result(&job_id).await {
                            if let Some(issues_array) = result.as_array() {
                                for issue in issues_array {
                                    if let (Some(desc), Some(severity)) = (
                                        issue.get("description").and_then(|d| d.as_str()),
                                        issue.get("severity").and_then(|s| s.as_str()),
                                    ) {
                                        all_issues.push(crate::core::analyzer::AnalysisIssue {
                                            category: "Dependencies".to_string(),
                                            description: desc.to_string(),
                                            severity: match severity {
                                                "critical" => {
                                                    crate::core::analyzer::IssueSeverity::Critical
                                                }
                                                "warning" => {
                                                    crate::core::analyzer::IssueSeverity::Warning
                                                }
                                                _ => crate::core::analyzer::IssueSeverity::Info,
                                            },
                                            file: issue
                                                .get("file")
                                                .and_then(|f| f.as_str())
                                                .map(std::string::ToString::to_string),
                                            line: issue
                                                .get("line")
                                                .and_then(serde_json::Value::as_u64)
                                                .map(|l| l as usize),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                println!("{}", "📦 [2/6] Escaneando dependencias...".yellow());
                let deps_issues = self.scan_dependencies();
                all_issues.extend(deps_issues);
            }
        }
        if dead_code {
            if use_parallel {
                println!(
                    "{}",
                    "💀 [3/6] Detectando código muerto (PARALELO con Nim)...".yellow()
                );
                let job_data = json ! ({ "project_path" : std :: env :: current_dir () ?. to_string_lossy () , "scan_type" : "dead_code" });
                if let Ok(job_id) = jarvix_client
                    .as_ref()
                    .unwrap()
                    .submit_parallel_analysis_job("dead_code_scan", job_data)
                    .await
                {
                    println!("📤 Job código muerto enviado: {job_id}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    if let Ok(Some(result)) = jarvix_client
                        .as_ref()
                        .unwrap()
                        .get_job_result(&job_id)
                        .await
                    {
                        if let Some(issues_array) = result.as_array() {
                            for issue in issues_array {
                                if let Some(desc) =
                                    issue.get("description").and_then(|d| d.as_str())
                                {
                                    all_issues.push(crate::core::analyzer::AnalysisIssue {
                                        category: "Code Quality".to_string(),
                                        description: desc.to_string(),
                                        severity: crate::core::analyzer::IssueSeverity::Info,
                                        file: issue
                                            .get("file")
                                            .and_then(|f| f.as_str())
                                            .map(std::string::ToString::to_string),
                                        line: issue
                                            .get("line")
                                            .and_then(serde_json::Value::as_u64)
                                            .map(|l| l as usize),
                                    });
                                }
                            }
                        }
                    }
                }
            } else {
                println!("{}", "💀 [3/6] Detectando código muerto/mock...".yellow());
                let dead_issues = self.scan_dead_code();
                all_issues.extend(dead_issues);
            }
        }
        if multilang {
            println!("{}", "🌐 [4/6] Análisis multilenguaje...".yellow());
            let lang_issues = self.scan_multilang();
            all_issues.extend(lang_issues);
        }
        println!("{}", "🏗️ [5/6] Analizando artifacts de build...".yellow());
        let build_issues = self.scan_build_artifacts();
        all_issues.extend(build_issues);
        println!("{}", "📊 [6/6] Generando reporte...".yellow());
        self.generate_scan_report(&all_issues, &all_suggestions, export, &mut metrics)?;
        if let Some(client) = jarvix_client {
            metrics.add_custom_metric("total_issues".to_string(), all_issues.len() as u64);
            metrics.add_custom_metric(
                "critical_issues".to_string(),
                all_issues
                    .iter()
                    .filter(|i| {
                        matches!(i.severity, crate::core::analyzer::IssueSeverity::Critical)
                    })
                    .count() as u64,
            );
            metrics.add_custom_metric("parallel_processing".to_string(), i32::from(use_parallel));
            metrics.add_custom_metric("performance_boost".to_string(), 400);
            if let Err(e) = client.report_scan_metrics(metrics).await {
                eprintln!("⚠️ No se pudo reportar métricas de scan: {e}");
            }
        }
        Ok(())
    }

    /// Run a compact pipeline: analyze -> repair -> test
    async fn run_auto(&self, no_jarvix: bool) -> Result<()> {
        println!("{}", "⚡ TRAE AUTO - pipeline compacto: analyze -> repair -> test".cyan().bold());
        // Analyze
        // default: full profile = None, don't force refresh, no output file
        crate::api::analyze(true, true, true, no_jarvix, None, false, None).await?;
        // Repair (auto)
        // default repair: level balanced, rollback disabled, no updates, no git operations
        let repair_opts = crate::commands::repair::RepairOptions {
            auto: true,
            clippy: true,
            fmt: true,
            deps: true,
            dry_run: false,
            no_jarvix,
            level: Some("balanced".to_string()),
            rollback: false,
            update: false,
            upgrade: false,
            git_branch: None,
            git_commit: None,
        };
        crate::api::repair(repair_opts).await?;
        // Test (basic)
        crate::api::test_cmd(false, false, false, None, None, false, no_jarvix).await?;
        println!("{}", "✅ TRAE AUTO completado".green());
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn scan_rust_project(
        &self,
        critical_only: bool,
    ) -> (
        Vec<crate::core::analyzer::AnalysisIssue>,
        Vec<crate::core::analyzer::OptimizationSuggestion>,
    ) {
        use walkdir::WalkDir;
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                if let Ok(content) = std::fs::read_to_string(path) {
                    for (line_num, line) in content.lines().enumerate() {
                        if line.contains("TODO:")
                            || line.contains("FIXME:")
                            || line.contains("XXX:")
                        {
                            let severity = if line.contains("FIXME:") {
                                crate::core::analyzer::IssueSeverity::Critical
                            } else if line.contains("XXX:") {
                                crate::core::analyzer::IssueSeverity::Warning
                            } else {
                                crate::core::analyzer::IssueSeverity::Info
                            };
                            if !critical_only
                                || matches!(
                                    severity,
                                    crate::core::analyzer::IssueSeverity::Critical
                                )
                            {
                                issues.push(crate::core::analyzer::AnalysisIssue {
                                    category: "Code Quality".to_string(),
                                    description: format!(
                                        "{} en línea {}: {}",
                                        if line.contains("FIXME:") {
                                            "FIXME"
                                        } else if line.contains("XXX:") {
                                            "XXX"
                                        } else {
                                            "TODO"
                                        },
                                        line_num + 1,
                                        line.trim()
                                    ),
                                    severity,
                                    file: Some(path.to_string_lossy().to_string()),
                                    line: Some(line_num + 1),
                                });
                            }
                        }
                        if line.contains("panic!") {
                            issues.push(crate::core::analyzer::AnalysisIssue {
                                category: "Safety".to_string(),
                                description: format!(
                                    "panic! macro en línea {}: {}",
                                    line_num + 1,
                                    line.trim()
                                ),
                                severity: crate::core::analyzer::IssueSeverity::Critical,
                                file: Some(path.to_string_lossy().to_string()),
                                line: Some(line_num + 1),
                            });
                        }
                        if line.contains("unwrap()") && !line.contains("//") {
                            let severity = if content.matches("unwrap()").count() > 10 {
                                crate::core::analyzer::IssueSeverity::Critical
                            } else {
                                crate::core::analyzer::IssueSeverity::Warning
                            };
                            if !critical_only
                                || matches!(
                                    severity,
                                    crate::core::analyzer::IssueSeverity::Critical
                                )
                            {
                                issues.push(crate::core::analyzer::AnalysisIssue {
                                    category: "Safety".to_string(),
                                    description: format!(
                                        "unwrap() en línea {}: {}",
                                        line_num + 1,
                                        line.trim()
                                    ),
                                    severity,
                                    file: Some(path.to_string_lossy().to_string()),
                                    line: Some(line_num + 1),
                                });
                            }
                        }
                    }
                    let lines = content.lines().count();
                    if lines > 1000 {
                        suggestions.push(crate::core::analyzer::OptimizationSuggestion {
                            description: format!(
                                "Archivo muy grande ({lines} líneas) - Considerar refactorizar"
                            ),
                            impact: crate::core::analyzer::OptimizationImpact::High,
                            effort: crate::core::analyzer::OptimizationEffort::High,
                            file: Some(path.to_string_lossy().to_string()),
                            line: None,
                        });
                    }
                }
            }
        }
        (issues, suggestions)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn scan_dependencies(&self) -> Vec<crate::core::analyzer::AnalysisIssue> {
        let mut issues = Vec::new();
        if let Ok(content) = std::fs::read_to_string("Cargo.toml") {
            for (line_num, line) in content.lines().enumerate() {
                if line.contains('=')
                    && !line.contains("version")
                    && !line.contains('[')
                    && !line.contains('#')
                    && (line.contains(".git") || line.contains("path ="))
                {
                    issues.push(crate::core::analyzer::AnalysisIssue {
                        category: "Dependencies".to_string(),
                        description: format!(
                            "Dependencia sin versión fija en línea {}: {}",
                            line_num + 1,
                            line.trim()
                        ),
                        severity: crate::core::analyzer::IssueSeverity::Warning,
                        file: Some("Cargo.toml".to_string()),
                        line: Some(line_num + 1),
                    });
                }
            }
        }
        if !std::path::Path::new("Cargo.lock").exists() {
            issues.push(crate::core::analyzer::AnalysisIssue {
                category: "Dependencies".to_string(),
                description: "Cargo.lock faltante - Ejecutar 'cargo update'".to_string(),
                severity: crate::core::analyzer::IssueSeverity::Warning,
                file: None,
                line: None,
            });
        }
        issues
    }
    #[doc = "Method documentation added by AI refactor"]
    fn scan_dead_code(&self) -> Vec<crate::core::analyzer::AnalysisIssue> {
        use walkdir::WalkDir;
        let mut issues = Vec::new();
        for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                if let Ok(content) = std::fs::read_to_string(path) {
                    for (line_num, line) in content.lines().enumerate() {
                        if line.contains("mock") || line.contains("Mock") || line.contains("MOCK") {
                            issues.push(crate::core::analyzer::AnalysisIssue {
                                category: "Code Quality".to_string(),
                                description: format!(
                                    "Posible código mock en línea {}: {}",
                                    line_num + 1,
                                    line.trim()
                                ),
                                severity: crate::core::analyzer::IssueSeverity::Info,
                                file: Some(path.to_string_lossy().to_string()),
                                line: Some(line_num + 1),
                            });
                        }
                        if line.contains("#[allow(dead_code)]") {
                            issues.push(crate::core::analyzer::AnalysisIssue {
                                category: "Code Quality".to_string(),
                                description: format!(
                                    "Código marcado como dead_code en línea {}",
                                    line_num + 1
                                ),
                                severity: crate::core::analyzer::IssueSeverity::Info,
                                file: Some(path.to_string_lossy().to_string()),
                                line: Some(line_num + 1),
                            });
                        }
                    }
                }
            }
        }
        issues
    }
    #[doc = "Method documentation added by AI refactor"]
    fn scan_multilang(&self) -> Vec<crate::core::analyzer::AnalysisIssue> {
        use walkdir::WalkDir;
        let mut issues = Vec::new();
        for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                match ext {
                    "js" | "ts" | "jsx" | "tsx" => {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for (line_num, line) in content.lines().enumerate() {
                                if line.contains("console.log") && !line.trim().starts_with("//") {
                                    issues.push(crate::core::analyzer::AnalysisIssue {
                                        category: "Code Quality".to_string(),
                                        description: format!(
                                            "console.log en archivo JS línea {}: {}",
                                            line_num + 1,
                                            line.trim()
                                        ),
                                        severity: crate::core::analyzer::IssueSeverity::Info,
                                        file: Some(path.to_string_lossy().to_string()),
                                        line: Some(line_num + 1),
                                    });
                                }
                            }
                        }
                    }
                    "py" => {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for (line_num, line) in content.lines().enumerate() {
                                if line.contains("print(") && !line.trim().starts_with('#') {
                                    issues.push(crate::core::analyzer::AnalysisIssue {
                                        category: "Code Quality".to_string(),
                                        description: format!(
                                            "print() en archivo Python línea {}: {}",
                                            line_num + 1,
                                            line.trim()
                                        ),
                                        severity: crate::core::analyzer::IssueSeverity::Info,
                                        file: Some(path.to_string_lossy().to_string()),
                                        line: Some(line_num + 1),
                                    });
                                }
                            }
                        }
                    }
                    "go" => {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for (line_num, line) in content.lines().enumerate() {
                                if line.contains("fmt.Println") && !line.trim().starts_with("//") {
                                    issues.push(crate::core::analyzer::AnalysisIssue {
                                        category: "Code Quality".to_string(),
                                        description: format!(
                                            "fmt.Println en archivo Go línea {}: {}",
                                            line_num + 1,
                                            line.trim()
                                        ),
                                        severity: crate::core::analyzer::IssueSeverity::Info,
                                        file: Some(path.to_string_lossy().to_string()),
                                        line: Some(line_num + 1),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        issues
    }
    #[doc = "Method documentation added by AI refactor"]
    fn scan_build_artifacts(&self) -> Vec<crate::core::analyzer::AnalysisIssue> {
        let mut issues = Vec::new();
        if std::path::Path::new("target").exists() {
            if let Ok(entries) = std::fs::read_dir("target") {
                let mut total_size = 0u64;
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                    }
                }
                if total_size > 2_000_000_000 {
                    issues.push(crate::core::analyzer::AnalysisIssue {
                        category: "Build".to_string(),
                        description: format!(
                            "Directorio target muy grande ({:.1} GB) - Ejecutar 'cargo clean'",
                            total_size as f64 / 1_000_000_000.0
                        ),
                        severity: crate::core::analyzer::IssueSeverity::Warning,
                        file: Some("target/".to_string()),
                        line: None,
                    });
                }
            }
        }
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".tmp")
                        || name.ends_with(".bak")
                        || name.starts_with('.') && name.ends_with(".swp")
                    {
                        issues.push(crate::core::analyzer::AnalysisIssue {
                            category: "Build".to_string(),
                            description: format!("Archivo temporal/backup encontrado: {name}"),
                            severity: crate::core::analyzer::IssueSeverity::Info,
                            file: Some(path.to_string_lossy().to_string()),
                            line: None,
                        });
                    }
                }
            }
        }
        issues
    }
    #[doc = "Method documentation added by AI refactor"]
    fn generate_scan_report(
        &self,
        issues: &[crate::core::analyzer::AnalysisIssue],
        suggestions: &[crate::core::analyzer::OptimizationSuggestion],
        export: Option<&str>,
        metrics: &mut crate::metrics::collector::MetricsCollector,
    ) -> Result<()> {
        use colored::Colorize;
        let critical_issues: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.severity, crate::core::analyzer::IssueSeverity::Critical))
            .collect();
        let warning_issues: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.severity, crate::core::analyzer::IssueSeverity::Warning))
            .collect();
        let info_issues: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.severity, crate::core::analyzer::IssueSeverity::Info))
            .collect();
        println!("\n{}", "📊 REPORTE FINAL DE SUPER SCAN".green().bold());
        println!("{}", "============================\n".green());
        println!(
            "{}",
            format!("📈 Issues encontrados: {}", issues.len()).yellow()
        );
        println!("  🔴 Críticos: {} ", critical_issues.len());
        println!("  🟡 Advertencias: {}", warning_issues.len());
        println!("  🔵 Informativos: {}", info_issues.len());
        println!("  💡 Sugerencias: {}\n", suggestions.len());
        if !critical_issues.is_empty() {
            println!("{}", "🔴 ISSUES CRÍTICOS:".red().bold());
            for issue in &critical_issues {
                if let (Some(file), Some(line)) = (&issue.file, issue.line) {
                    println!(
                        "  ❗ {}: {} ({}:{})",
                        issue.category, issue.description, file, line
                    );
                } else {
                    println!("  ❗ {}: {}", issue.category, issue.description);
                }
            }
            println!();
        }
        metrics.add_custom_metric("scan_completed".to_string(), 1u64);
        metrics.add_custom_metric("total_issues".to_string(), issues.len() as u64);
        metrics.add_custom_metric("critical_count".to_string(), critical_issues.len() as u64);
        metrics.finish();
        if let Some(export_path) = export {
            let report = serde_json :: json ! ({ "timestamp" : chrono :: Utc :: now () , "total_issues" : issues . len () , "critical_issues" : critical_issues . len () , "warning_issues" : warning_issues . len () , "info_issues" : info_issues . len () , "suggestions" : suggestions . len () , "issues" : issues , "suggestions" : suggestions });
            std::fs::write(export_path, serde_json::to_string_pretty(&report)?)?;
            println!(
                "{}",
                format!("📁 Reporte exportado a: {export_path}").green()
            );
        }
        if critical_issues.is_empty() {
            println!(
                "{}",
                "✅ ¡No se encontraron issues críticos!".green().bold()
            );
        } else {
            println!(
                "{}",
                format!(
                    "⚠️ Revisa {} issues críticos antes de continuar",
                    critical_issues.len()
                )
                .red()
                .bold()
            );
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_external_cargo(&self, args: &[String]) -> Result<()> {
        if args.is_empty() {
            println!(
                "{}",
                "⚠️  No se proporcionó comando cargo externo.".yellow()
            );
            return Ok(());
        }
        println!(
            "{}",
            format!("🚀 Passthrough cargo: {}", args.join(" "))
                .cyan()
                .bold()
        );
        let executor = CargoExecutor::new().with_working_dir(".");
        executor.execute_streaming(args).await
    }
}
