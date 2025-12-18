#![doc = " # Build Command - Enhanced cargo build with analysis"]
#![doc = ""]
#![doc = " Comando de build mejorado con anÃ¡lisis automÃ¡tico y reporte de mÃ©tricas"]
use crate::{
    cli::TraeCli,
    commands::repair::RepairCommand,
    core::{analyzer::ProjectAnalyzer, cargo::CargoExecutor},
    jarvix::client::JarvixClient,
    metrics::collector::MetricsCollector,
    utils::ui::{print_step_table, StepSummary},
};
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::time::Instant;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct BuildCommand {
    #[doc = " Build in release mode"]
    #[arg(long)]
    pub release: bool,
    #[doc = " Build specific target"]
    #[arg(long)]
    pub target: Option<String>,
    #[doc = " Enable specific features"]
    #[arg(long)]
    pub features: Vec<String>,
    #[doc = " Build all packages in workspace"]
    #[arg(long)]
    pub workspace: bool,
    #[doc = " Run automatic analysis after build"]
    #[arg(long, default_value = "true")]
    pub analyze: bool,
    #[doc = " Auto-repair detected issues"]
    #[arg(long)]
    pub auto_repair: bool,
    #[doc = " Benchmark build time"]
    #[arg(long)]
    pub benchmark: bool,
    #[doc = " Use Docker for build with Chapel support"]
    #[arg(long)]
    pub docker: bool,
    #[doc = " Additional cargo arguments"]
    #[arg(last = true)]
    pub cargo_args: Vec<String>,
}
impl BuildCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self, cli: &TraeCli) -> Result<()> {
        info!("??? Iniciando build mejorado con TRAE CLI");
        let total_start = Instant::now();
        let mut metrics = MetricsCollector::new("build".to_string());
        let mut perf_metrics = crate::performance_patterns::MetricsCollector::new();
        let mut steps = Vec::new();
        let mut artifacts = Vec::new();
        let mut fatal_error: Option<anyhow::Error> = None;
        perf_metrics.start_operation("show_config".to_string());
        self.show_build_config(cli);
        perf_metrics.end_operation(true);
        if self.analyze {
            perf_metrics.start_operation("pre_analysis".to_string());
            let step_start = Instant::now();
            match self.pre_build_analysis() {
                Ok(_) => {
                    perf_metrics.end_operation(true);
                    steps.push(StepSummary::success("Pre-anÃ¡lisis", step_start.elapsed()));
                }
                Err(e) => {
                    perf_metrics.end_operation(false);
                    steps.push(StepSummary::failed(
                        "Pre-anÃ¡lisis",
                        step_start.elapsed(),
                        e.to_string(),
                    ));
                    fatal_error = Some(e);
                }
            }
        } else {
            steps.push(StepSummary::skipped("Pre-anÃ¡lisis"));
        }
        if fatal_error.is_none() {
            perf_metrics.start_operation("cargo_build".to_string());
            let step_start = Instant::now();
            match self.execute_build(cli).await {
                Ok(result_artifacts) => {
                    perf_metrics.end_operation(true);
                    steps.push(StepSummary::success("Cargo build", step_start.elapsed()));
                    artifacts = result_artifacts;
                }
                Err(e) => {
                    perf_metrics.end_operation(false);
                    steps.push(StepSummary::failed(
                        "Cargo build",
                        step_start.elapsed(),
                        e.to_string(),
                    ));
                    fatal_error = Some(e);
                }
            }
        } else {
            steps.push(StepSummary::skipped("Cargo build"));
        }
        if self.analyze {
            if fatal_error.is_none() {
                perf_metrics.start_operation("post_analysis".to_string());
                let step_start = Instant::now();
                match self.post_build_analysis(&artifacts) {
                    Ok(_) => {
                        perf_metrics.end_operation(true);
                        steps.push(StepSummary::success("Post-anÃ¡lisis", step_start.elapsed()));
                    }
                    Err(e) => {
                        perf_metrics.end_operation(false);
                        steps.push(StepSummary::failed(
                            "Post-anÃ¡lisis",
                            step_start.elapsed(),
                            e.to_string(),
                        ));
                        fatal_error = Some(e);
                    }
                }
            } else {
                steps.push(StepSummary::skipped("Post-anÃ¡lisis"));
            }
        } else {
            steps.push(StepSummary::skipped("Post-anÃ¡lisis"));
        }
        if self.auto_repair {
            if fatal_error.is_none() {
                perf_metrics.start_operation("auto_repair".to_string());
                let step_start = Instant::now();
                match self.run_auto_repair(cli).await {
                    Ok(()) => {
                        perf_metrics.end_operation(true);
                        steps.push(StepSummary::success("Auto-repair", step_start.elapsed()));
                    }
                    Err(e) => {
                        perf_metrics.end_operation(false);
                        steps.push(StepSummary::failed(
                            "Auto-repair",
                            step_start.elapsed(),
                            e.to_string(),
                        ));
                        fatal_error = Some(e);
                    }
                }
            } else {
                steps.push(StepSummary::skipped("Auto-repair"));
            }
        } else {
            steps.push(StepSummary::skipped("Auto-repair"));
        }
        let total_duration = total_start.elapsed();
        metrics.record_build_time(total_duration);
        metrics.record_build_result(fatal_error.is_none());
        metrics.finish();
        if cli.no_jarvix {
            steps.push(StepSummary::skipped("Jarvix report"));
        } else {
            let step_start = Instant::now();
            match self.report_metrics(metrics.clone()).await {
                Ok(()) => steps.push(StepSummary::success("Jarvix report", step_start.elapsed())),
                Err(e) => {
                    steps.push(StepSummary::failed(
                        "Jarvix report",
                        step_start.elapsed(),
                        e.to_string(),
                    ));
                }
            }
        }
        print_step_table("Build Summary", &steps, total_duration);
        if fatal_error.is_none() {
            if !perf_metrics.operations.is_empty() {
                let stability = perf_metrics.fft_pattern_analysis();
                if stability < 0.7 {
                    println ! ("??  Patrones de build inestables detectados (Estabilidad FFT: {stability:.2})");
                } else {
                    println!("? Patrones de build estables (FFT: {stability:.2})");
                }
                println!("\n{}", perf_metrics.report());
                let slowest = perf_metrics.slowest_operations(3);
                if !slowest.is_empty() {
                    println!("\n?? Operaciones mÃ¡s lentas:");
                    for (i, op) in slowest.iter().enumerate() {
                        println!(
                            "   {}. {:?} ({})",
                            i + 1,
                            op.duration,
                            if op.success { "V" } else { "?" }
                        );
                    }
                }
            }
            println!(
                "{} Build completado en {:.2}s",
                "?".green(),
                total_duration.as_secs_f64()
            );
            Ok(())
        } else if let Some(err) = fatal_error {
            if let Err(suggest_err) = self.suggest_repairs(&err) {
                eprintln!("?? No se pudieron sugerir reparaciones: {suggest_err}");
            }
            Err(err)
        } else {
            unreachable!()
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn show_build_config(&self, _cli: &TraeCli) {
        println!("{}", "ðŸ“‹ ConfiguraciÃ³n del Build:".cyan().bold());
        println!(
            "  â€¢ Modo: {}",
            if self.release {
                "Release".yellow()
            } else {
                "Debug".blue()
            }
        );
        if let Some(target) = &self.target {
            println!("  â€¢ Target: {}", target.green());
        }
        if !self.features.is_empty() {
            println!("  â€¢ Features: {}", self.features.join(", ").green());
        }
        if self.workspace {
            println!("  â€¢ Workspace: {}", "SÃ\u{AD}".green());
        }
        println!(
            "  â€¢ AnÃ¡lisis: {}",
            if self.analyze {
                "Habilitado".green()
            } else {
                "Deshabilitado".red()
            }
        );
        println!(
            "  â€¢ Auto-repair: {}",
            if self.auto_repair {
                "Habilitado".green()
            } else {
                "Deshabilitado".red()
            }
        );
        println!();
    }
    #[doc = "Method documentation added by AI refactor"]
    fn pre_build_analysis(&self) -> Result<()> {
        println!("{}", "ðŸ” Ejecutando pre-anÃ¡lisis...".cyan());
        let quantum_start = Instant::now();
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Failed to set spinner template"),
        );
        spinner.set_message("Analizando estructura del proyecto...");
        let mut analyzer = ProjectAnalyzer::new();
        let analysis = analyzer.analyze_project(".")?;
        let analysis_time = quantum_start.elapsed();
        let fft_factor = (1000.0 / (analysis_time.as_millis() as f64 + 1.0)).min(10.0);
        spinner.finish_with_message(format!(
            "Pre-anÃ¡lisis completado âœ“ (FFT: {fft_factor:.2})"
        ));
        if analysis.has_critical_issues() {
            warn!("âš ï¸ Se encontraron issues crÃ\u{AD}ticos que podrÃ\u{AD}an afectar el build");
            analysis.show_critical_issues();
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn execute_build(&self, _cli: &TraeCli) -> Result<Vec<String>> {
        let build_msg = if self.docker {
            "ðŸš€ Ejecutando cargo build con Docker y Chapel..."
        } else {
            "ðŸš€ Ejecutando cargo build..."
        };
        println!("{}", build_msg.cyan());
        let progress = ProgressBar::new_spinner();
        progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Failed to set progress template"),
        );
        progress.set_message("Compilando proyecto...");
        let result = if self.docker {
            self.execute_build_with_docker().await
        } else {
            let executor = CargoExecutor::new();
            let mut build_args = vec!["build".to_string()];
            if self.release {
                build_args.push("--release".to_string());
            }
            if let Some(target) = &self.target {
                build_args.extend_from_slice(&["--target".to_string(), target.clone()]);
            }
            if !self.features.is_empty() {
                build_args.extend_from_slice(&["--features".to_string(), self.features.join(",")]);
            }
            if self.workspace {
                build_args.push("--workspace".to_string());
            }
            build_args.extend_from_slice(&self.cargo_args);
            executor.execute_streaming_capture(&build_args).await
        };
        progress.finish_with_message("Build completado âœ“".to_string());
        match result {
            Ok(output) => {
                let artifacts = self.extract_artifacts(&output);
                Ok(artifacts)
            }
            Err(e) => Err(e),
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn execute_build_with_docker(&self) -> Result<String> {
        use tokio::process::Command;
        let mut docker_args = vec![
            "run".to_string(),
            "--rm".to_string(),
            "-v".to_string(),
            format!("{}:/app", std::env::current_dir()?.display()),
            "-w".to_string(),
            "/app".to_string(),
            "trae-cli:latest".to_string(),
            "cargo".to_string(),
            "build".to_string(),
        ];
        if self.release {
            docker_args.push("--release".to_string());
        }
        if let Some(target) = &self.target {
            docker_args.extend_from_slice(&["--target".to_string(), target.clone()]);
        }
        if !self.features.is_empty() {
            docker_args.extend_from_slice(&["--features".to_string(), self.features.join(",")]);
        }
        if self.workspace {
            docker_args.push("--workspace".to_string());
        }
        docker_args.extend_from_slice(&self.cargo_args);
        let output = Command::new("docker").args(&docker_args).output().await?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow::anyhow!(
                "Docker build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn post_build_analysis(&self, artifacts: &[String]) -> Result<()> {
        println!("{}", "ðŸ” Ejecutando post-anÃ¡lisis...".cyan());
        let analyzer = ProjectAnalyzer::new();
        let analysis = analyzer.analyze_artifacts(artifacts)?;
        analysis.show_summary();
        if analysis.has_optimizations() {
            println!(
                "{}",
                "ðŸ’¡ Sugerencias de optimizaciÃ³n encontradas:"
                    .yellow()
                    .bold()
            );
            analysis.show_optimizations();
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_auto_repair(&self, cli: &TraeCli) -> Result<()> {
        println!("{}", "ðŸ”§ Ejecutando auto-repair...".cyan());
        let repair = RepairCommand {
            auto: true,
            force: true,
            ..Default::default()
        };
        repair.execute(cli).await
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn report_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        match JarvixClient::new() {
            Ok(Some(client)) => {
                client.report_build_metrics(metrics).await?;
                println!("{}", "ðŸ“Š MÃ©tricas reportadas a JARVIXSERVER".green());
            }
            Ok(None) => {
                println!("{}", "âš ï¸ JARVIXSERVER no configurado".yellow());
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Error conectando a JARVIXSERVER: {e}"));
            }
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn suggest_repairs(&self, error: &anyhow::Error) -> Result<()> {
        println!("{}", "ðŸ”§ Sugerencias de reparaciÃ³n:".yellow().bold());
        let error_str = error.to_string();
        if error_str.contains("could not compile") {
            println!(
                "  â€¢ Ejecuta: {} para anÃ¡lisis detallado",
                "trae analyze".green()
            );
        }
        if error_str.contains("dependency") {
            println!(
                "  â€¢ Ejecuta: {} para reparar dependencias",
                "trae repair --deps".green()
            );
        }
        if error_str.contains("format") || error_str.contains("clippy") {
            println!(
                "  â€¢ Ejecuta: {} para formato automÃ¡tico",
                "trae repair --fmt".green()
            );
        }
        println!(
            "  â€¢ Ejecuta: {} para reparaciÃ³n automÃ¡tica completa",
            "trae repair --auto".green()
        );
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn extract_artifacts(&self, output: &str) -> Vec<String> {
        let mut artifacts = Vec::new();
        for line in output.lines() {
            if line.trim().starts_with("Finished") {
                if let Some(target_start) = line.find("target") {
                    let target_path = &line[target_start..];
                    if let Some(target_end) = target_path.find(' ') {
                        artifacts.push(target_path[..target_end].to_string());
                    }
                }
            } else if line.contains("target/")
                && (line.contains(".exe") || line.contains("debug/") || line.contains("release/"))
            {
                if let Some(start) = line.find("target/") {
                    let path_part = &line[start..];
                    if let Some(end) = path_part.find(' ') {
                        artifacts.push(path_part[..end].to_string());
                    } else {
                        artifacts.push(path_part.to_string());
                    }
                }
            }
        }
        artifacts
    }
}
