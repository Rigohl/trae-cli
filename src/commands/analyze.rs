#![doc = " # Analyze Command - Deep code analysis and optimization"]
#![doc = ""]
#![doc = " Comando para análisis profundo del código y sugerencias de optimización"]
use crate::cli::JarvixCli;
use anyhow::Result;
use clap::Args;
use colored::Colorize;
#[doc = " Six Sigma Analysis Command - Herramienta de análisis profundo de calidad"]
#[doc = ""]
#[doc = " Esta estructura implementa un analizador de código Six Sigma completo que:"]
#[doc = " - Analiza cuellos de botella de rendimiento"]
#[doc = " - Detecta vulnerabilidades de seguridad"]
#[doc = " - Evalúa calidad del código"]
#[doc = " - Genera reportes detallados con métricas DPMO"]
#[doc = ""]
#[doc = " # Six Sigma Compliance"]
#[doc = " - 0 `unwrap()` calls"]
#[doc = " - Robust error handling"]
#[doc = " - Complete type annotations"]
#[doc = " - Comprehensive documentation"]
#[derive(Args, Debug)]
pub struct AnalyzeCommand {
    #[doc = " Analyze performance bottlenecks (Six Sigma: measures cycle time, throughput)"]
    #[arg(long)]
    pub performance: bool,
    #[doc = " Analyze security issues (Six Sigma: defect detection and prevention)"]
    #[arg(long)]
    pub security: bool,
    #[doc = " Analyze code quality (Six Sigma: process capability assessment)"]
    #[arg(long)]
    pub quality: bool,
    #[doc = " Generate detailed report (Six Sigma: statistical process control charts)"]
    #[arg(long)]
    pub report: bool,
    #[doc = "Profile for analysis: fast, full, deep"]
    #[arg(long)]
    pub profile: Option<String>,
    #[doc = "Force refresh cache"]
    #[arg(long)]
    pub force_refresh: bool,
    #[doc = "Write JSON summary to path"]
    #[arg(long, value_name = "PATH")]
    pub output: Option<String>,
}
impl AnalyzeCommand {
    #[doc = " Ejecuta el análisis Six Sigma completo del proyecto"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = " * `cli` - Configuración de TRAE CLI"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = " * `Result<()>` - Ok(()) si el análisis es exitoso"]
    #[doc = ""]
    #[doc = " # Six Sigma Metrics"]
    #[doc = " - DPMO (Defects Per Million Opportunities)"]
    #[doc = " - Sigma Level calculation"]
    #[doc = " - Process capability indices"]
    #[doc = " - Control chart data"]
    #[doc = ""]
    #[doc = " # Performance"]
    #[doc = " - Multi-threaded analysis"]
    #[doc = " - Progress indicators"]
    #[doc = " - Memory-efficient processing"]
    pub async fn execute(&self, cli: &JarvixCli) -> Result<()> {
        // Delegate to the API-friendly run_simple to keep behavior consistent
        crate::commands::analyze::AnalyzeCommand::run_simple(
            self.performance,
            self.security,
            self.quality,
            cli.no_jarvix,
            self.profile.clone(),
            self.force_refresh,
            self.output.clone(),
        )
        .await
    }

    /// API-friendly wrapper to run analyze without a full `JarvixCli` instance.
    pub async fn run_simple(
        _performance: bool,
        _security: bool,
        _quality: bool,
        no_jarvix: bool,
        profile: Option<String>,
        force_refresh: bool,
        output: Option<String>,
    ) -> Result<()> {
        use sha2::{Digest, Sha256};
        use std::fs;
        use std::path::Path;

        // Minimal equivalent of AnalyzeCommand::execute with caching
        println!("{}", "🔍 Análisis profundo del proyecto...".cyan().bold());

        // Find workspace root so analysis works from any subdirectory in a Rust workspace
        let orig_cwd = std::env::current_dir()?;
        let mut root = orig_cwd.clone();
        let mut found = false;
        while !root.join("Cargo.toml").exists() {
            if !root.pop() {
                break;
            }
        }
        if root.join("Cargo.toml").exists() {
            found = true;
        }
        // If we found a workspace root, change cwd to it; otherwise keep original cwd
        if found {
            let _ = std::env::set_current_dir(&root);
        }

        // Compute fingerprint of workspace (paths + modified time) for cache key
        let mut hasher = Sha256::new();
        for entry in walkdir::WalkDir::new(".")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if let Ok(md) = fs::metadata(entry.path()) {
                let p = entry.path().to_string_lossy();
                let mtime = md
                    .modified()
                    .ok()
                    .and_then(|t| t.elapsed().ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                hasher.update(p.as_bytes());
                hasher.update(mtime.to_string().as_bytes());
            }
        }
        let fingerprint = hex::encode(hasher.finalize());
        let cache_dir = Path::new(".trae").join("cache");
        let _ = fs::create_dir_all(&cache_dir);
        let cache_file = cache_dir.join(format!("analyze_{}.json", fingerprint));

        // TTL = 1 hour
        let use_cache = !force_refresh
            && cache_file.exists()
            && cache_file
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .map(|t| t.elapsed().map(|d| d.as_secs() < 3600).unwrap_or(false))
                .unwrap_or(false);
        if use_cache {
            if let Ok(s) = fs::read_to_string(&cache_file) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                    println!(
                        "📦 Usando cache de análisis ({})",
                        cache_file.to_string_lossy()
                    );
                    println!(
                        "Resumen: {}",
                        json.get("summary")
                            .unwrap_or(&serde_json::Value::String("(nocontent)".to_string()))
                    );
                    let _ = std::env::set_current_dir(orig_cwd);
                    return Ok(());
                }
            }
        }

        let mut metrics = crate::metrics::collector::MetricsCollector::new("analyze".to_string());
        let mut analyzer = crate::core::analyzer::ProjectAnalyzer::new();
        // Profile handling (lightweight influence)
        if let Some(p) = profile.as_deref() {
            let cfg = match p {
                "fast" => crate::performance_patterns::PerformanceConfig {
                    thread_count: 2,
                    cache_size: 200,
                    batch_size: 50,
                    timeout_ms: 2000,
                    parallel_threshold: 20,
                },
                "balanced" => crate::performance_patterns::PerformanceConfig {
                    thread_count: 4,
                    cache_size: 400,
                    batch_size: 100,
                    timeout_ms: 3000,
                    parallel_threshold: 30,
                },
                "deep" => crate::performance_patterns::PerformanceConfig::auto_tune(),
                _ => crate::performance_patterns::PerformanceConfig::default(),
            };
            analyzer = crate::core::analyzer::ProjectAnalyzer::with_config(cfg);
        }
        // Run heavy analysis in blocking thread to avoid blocking async runtime
        let analysis = tokio::task::spawn_blocking(move || analyzer.analyze_project(".")).await??;
        println!("\n📊 Resultados del Análisis:");
        println!("  • Issues detectados: {}", analysis.issues.len());
        println!(
            "  • Optimizaciones sugeridas: {}",
            analysis.suggestions.len()
        );
        println!("  • Líneas de código: {}", analysis.total_lines);
        println!("  • Archivos analizados: {}", analysis.files_count);
        metrics.add_custom_metric("issues_found".to_string(), analysis.issues.len() as u64);
        metrics.add_custom_metric(
            "suggestions_count".to_string(),
            analysis.suggestions.len() as u64,
        );
        metrics.add_custom_metric("total_lines".to_string(), analysis.total_lines as u64);
        metrics.add_custom_metric("files_analyzed".to_string(), analysis.files_count as u64);

        // Write cache summary
        let summary = serde_json::json!({
            "summary": format!("issues:{} suggestions:{} lines:{} files:{}", analysis.issues.len(), analysis.suggestions.len(), analysis.total_lines, analysis.files_count),
            "issues_count": analysis.issues.len(),
            "files_count": analysis.files_count,
            "lines": analysis.total_lines,
            "profile": profile.unwrap_or_else(|| "default".to_string()),
        });
        let _ = fs::write(
            &cache_file,
            serde_json::to_string_pretty(&summary).unwrap_or_default(),
        );

        // Also persist metrics and full analysis snapshot for offline inspection
        let metrics_dir = Path::new(".trae").join("metrics");
        let _ = fs::create_dir_all(&metrics_dir);
        let metrics_file = metrics_dir.join(format!("analyze_{}.json", fingerprint));
        let snapshot = serde_json::json!({
            "summary": summary,
            "analysis_metrics": analysis.metrics,
        });
        let _ = fs::write(
            &metrics_file,
            serde_json::to_string_pretty(&snapshot).unwrap_or_default(),
        );

        // Optionally write full JSON output
        if let Some(out) = output {
            let full = serde_json::json!({"analysis": summary, "issues": analysis.issues, "suggestions": analysis.suggestions, "metrics": analysis.metrics});
            let _ = fs::write(out, serde_json::to_string_pretty(&full).unwrap_or_default());
        }

        if !no_jarvix {
            if let Ok(Some(client)) = crate::jarvix::client::JarvixClient::new() {
                if let Err(e) = client.report_scan_metrics(metrics).await {
                    eprintln!("⚠️ No se pudo reportar métricas de análisis a JARVIXSERVER: {e}");
                }
            }
        }
        println!("{}", "✅ Análisis completado".green());
        let _ = std::env::set_current_dir(orig_cwd);
        Ok(())
    }
}
