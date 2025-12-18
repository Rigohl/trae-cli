#![doc = " # Enhanced Cargo Commands"]
#![doc = ""]
#![doc = " Wrapper inteligente de `cargo` con métricas, progreso y passthrough."]
use crate::{
    cli::TraeCli,
    core::cargo::{CargoExecutor, CargoStream},
    utils::progress,
};
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use which::which;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct CargoCommand {
    #[doc = " Cargo subcommand to execute"]
    #[arg(value_name = "COMMAND")]
    pub command: String,
    #[doc = " Additional arguments for cargo"]
    #[arg(
        last = true,
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_name = "ARGS"
    )]
    pub args: Vec<String>,
    #[doc = " Run command interactively"]
    #[arg(long)]
    pub interactive: bool,
}
#[doc = "Function documentation added by AI refactor"]
fn resolve_executable(name: &str) -> Option<String> {
    if let Ok(path) = which(name) {
        return Some(path.to_string_lossy().to_string());
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(cargo_home) = env::var_os("CARGO_HOME") {
        candidates.push(PathBuf::from(cargo_home).join("bin").join(name));
    }
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".cargo").join("bin").join(name));
    }
    for p in candidates {
        if p.exists() {
            return Some(p.to_string_lossy().to_string());
        }
        let mut pexe = p.clone();
        pexe.set_extension("exe");
        if pexe.exists() {
            return Some(pexe.to_string_lossy().to_string());
        }
    }
    None
}
impl CargoCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self, cli: &TraeCli) -> Result<()> {
        println!(
            "{}",
            format!("🚀 Ejecutando cargo {} mejorado...", self.command)
                .cyan()
                .bold()
        );
        if resolve_executable("cargo").is_none() {
            eprintln ! ("❌ 'cargo' no se encuentra en PATH ni en CARGO_HOME. Instálalo: https://www.rust-lang.org/tools/install");
            return Err(anyhow::anyhow!("cargo not found"));
        }
        let mut metrics =
            crate::metrics::collector::MetricsCollector::new(format!("cargo_{}", self.command));
        let start_time = Instant::now();
        let executor = CargoExecutor::new().with_working_dir(".");
        let mut arg_strings = Vec::new();
        arg_strings.push(self.command.clone());
        arg_strings.extend(self.args.clone());
        if !arg_strings
            .iter()
            .any(|arg| arg.starts_with("--color") || arg == "--color")
        {
            arg_strings.push("--color=always".to_string());
        }
        let arg_refs: Vec<&str> = arg_strings.iter().map(|s| s.as_str()).collect();
        if self.interactive {
            self.run_interactive(cli, &executor, &mut metrics, &arg_refs, start_time)
                .await
        } else {
            self.run_streaming(cli, &executor, &mut metrics, &arg_refs, start_time)
                .await
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_interactive(
        &self,
        cli: &TraeCli,
        executor: &CargoExecutor,
        metrics: &mut crate::metrics::collector::MetricsCollector,
        args: &[&str],
        start_time: Instant,
    ) -> Result<()> {
        match executor.execute_interactive(args).await {
            Ok(()) => {
                let duration = start_time.elapsed();
                metrics.add_custom_metric(
                    "execution_time_ms".to_string(),
                    duration.as_millis() as u64,
                );
                metrics.add_custom_metric("success".to_string(), 1);
                metrics.add_custom_metric("interactive_mode".to_string(), 1);
                self.report_metrics(cli, metrics).await;
                println!(
                    "{} Comando cargo {} (interactivo) completado en {:.2}s",
                    "✅".green(),
                    self.command,
                    duration.as_secs_f64()
                );
                Ok(())
            }
            Err(e) => {
                let duration = start_time.elapsed();
                metrics.add_custom_metric(
                    "execution_time_ms".to_string(),
                    duration.as_millis() as u64,
                );
                metrics.add_custom_metric("success".to_string(), 0);
                println!(
                    "{} Error ejecutando cargo {} (interactivo): {}",
                    "❌".red(),
                    self.command,
                    e
                );
                Err(e)
            }
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_streaming(
        &self,
        cli: &TraeCli,
        executor: &CargoExecutor,
        metrics: &mut crate::metrics::collector::MetricsCollector,
        args: &[&str],
        start_time: Instant,
    ) -> Result<()> {
        let total_units = progress::estimate_cargo_units().max(1);
        let progress_bar = ProgressBar::new(total_units as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} {pos}/{len} {wide_bar:.cyan/blue} {msg}")
                .expect("progress template"),
        );
        progress_bar.enable_steady_tick(Duration::from_millis(120));
        progress_bar.set_message(format!("Ejecutando cargo {}...", self.command));
        let verbose = cli.verbose;
        let mut completed = 0usize;
        let result = executor
            .execute_streaming_capture_with_handler(args, |stream, line| {
                let show_line = verbose
                    || matches!(stream, CargoStream::Stderr)
                    || line.contains("error:")
                    || line.contains("warning:");
                if show_line {
                    match stream {
                        CargoStream::Stdout => println!("{line}"),
                        CargoStream::Stderr => eprintln!("{line}"),
                    }
                }
                if line.contains("Compiling ") {
                    completed = (completed + 1).min(total_units);
                    progress_bar.set_position(completed as u64);
                    progress_bar.set_message(line.to_string());
                } else if line.starts_with("Finished ") {
                    progress_bar.set_position(total_units as u64);
                    progress_bar.set_message(line.to_string());
                }
            })
            .await;
        match result {
            Ok(_) => {
                progress_bar.finish_with_message("Cargo completado");
                let duration = start_time.elapsed();
                metrics.add_custom_metric(
                    "execution_time_ms".to_string(),
                    duration.as_millis() as u64,
                );
                metrics.add_custom_metric("success".to_string(), 1);
                metrics.add_custom_metric("streaming_mode".to_string(), 1);
                self.report_metrics(cli, metrics).await;
                println!(
                    "{} Comando cargo {} completado en {:.2}s",
                    "✅".green(),
                    self.command,
                    duration.as_secs_f64()
                );
                Ok(())
            }
            Err(e) => {
                progress_bar.abandon_with_message("Cargo falló");
                let duration = start_time.elapsed();
                metrics.add_custom_metric(
                    "execution_time_ms".to_string(),
                    duration.as_millis() as u64,
                );
                metrics.add_custom_metric("success".to_string(), 0);
                println!(
                    "{} Error ejecutando cargo {}: {}",
                    "❌".red(),
                    self.command,
                    e
                );
                Err(e)
            }
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn report_metrics(
        &self,
        cli: &TraeCli,
        metrics: &crate::metrics::collector::MetricsCollector,
    ) {
        if cli.no_jarvix {
            return;
        }
        if let Ok(Some(client)) = crate::jarvix::client::JarvixClient::new() {
            if let Err(e) = client.report_cargo_metrics(metrics.clone()).await {
                eprintln!("⚠️  No se pudo reportar métricas cargo a JARVIXSERVER: {e}");
            }
        }
    }

    /// API-friendly wrapper to run cargo subcommands programmatically without TraeCli.
    pub async fn run_simple(
        command: &str,
        args: &[String],
        interactive: bool,
        verbose: bool,
        no_jarvix: bool,
    ) -> Result<()> {
        // Attempt to offload cargo build to JarvixServer if available and command is build/test
        if !no_jarvix {
            if let Ok(Some(client)) = crate::jarvix::client::JarvixClient::new() {
                if command == "build" || command == "test" {
                    let job_data = serde_json::json!({
                        "cwd": std::env::current_dir()?.to_string_lossy().to_string(),
                        "command": command,
                        "args": args,
                    });
                    if let Ok(job_id) = client
                        .submit_parallel_analysis_job("cargo_build", job_data)
                        .await
                    {
                        println!("⚡ Offloading cargo {} to JarvixServer (job {})", command, job_id);
                        // Poll for result with timeout
                        let start = std::time::Instant::now();
                        let timeout = std::time::Duration::from_secs(120);
                        loop {
                            if start.elapsed() > timeout {
                                eprintln!("⚠️ Offload timed out, falling back to local cargo");
                                break;
                            }
                            if let Ok(Some(res)) = client.get_job_result(&job_id).await {
                                        // If remote job returns logs, stream them
                                        if let Some(logs) = res.get("logs") {
                                            println!("📤 Remote job logs:\n{}", logs);
                                        }
                                        // If remote job provides an artifact URL, try to download it
                                        if let Some(artifact) = res.get("artifact_url").and_then(|v| v.as_str()) {
                                            println!("📥 Downloading artifact from {}", artifact);
                                            match reqwest::get(artifact).await {
                                                Ok(resp) => {
                                                    if resp.status().is_success() {
                                                        let bytes = resp.bytes().await.unwrap_or_default();
                                                        let path = std::path::Path::new("target").join("remote_artifact.tar.gz");
                                                        let _ = std::fs::create_dir_all("target");
                                                        std::fs::write(&path, &bytes).ok();
                                                        println!("📦 Artifact saved to {}", path.to_string_lossy());
                                                    } else {
                                                        eprintln!("⚠️ Failed to download artifact: {}", resp.status());
                                                    }
                                                }
                                                Err(e) => eprintln!("⚠️ Error downloading artifact: {}", e),
                                            }
                                        }
                                        println!("📤 Remote job result: {}", res);
                                        return Ok(());
                            }
                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        }
                    }
                }
            }
        }
        println!(
            "{}",
            format!("🚀 Ejecutando cargo {} mejorado (API)...", command)
                .cyan()
                .bold()
        );
        if resolve_executable("cargo").is_none() {
            eprintln!("❌ 'cargo' no se encuentra en PATH ni en CARGO_HOME. Instálalo: https://www.rust-lang.org/tools/install");
            return Err(anyhow::anyhow!("cargo not found"));
        }
        let mut metrics = crate::metrics::collector::MetricsCollector::new(format!("cargo_{}", command));
        let start_time = Instant::now();
        let executor = CargoExecutor::new().with_working_dir(".");
        let mut arg_strings: Vec<String> = Vec::new();
        arg_strings.push(command.to_string());
        arg_strings.extend_from_slice(args);
        if !arg_strings
            .iter()
            .any(|arg| arg.starts_with("--color") || arg == "--color")
        {
            arg_strings.push("--color=always".to_string());
        }
        let arg_refs: Vec<&str> = arg_strings.iter().map(|s| s.as_str()).collect();
        if interactive {
            match executor.execute_interactive(&arg_refs).await {
                Ok(()) => {
                    let duration = start_time.elapsed();
                    metrics.add_custom_metric("execution_time_ms".to_string(), duration.as_millis() as u64);
                    metrics.add_custom_metric("success".to_string(), 1);
                    metrics.add_custom_metric("interactive_mode".to_string(), 1);
                    if !no_jarvix {
                        if let Ok(Some(client)) = crate::jarvix::client::JarvixClient::new() {
                            if let Err(e) = client.report_cargo_metrics(metrics.clone()).await {
                                eprintln!("⚠️ No se pudo reportar métricas cargo a JARVIXSERVER: {e}");
                            }
                        }
                    }
                    println!("{} Comando cargo {} (interactivo) completado en {:.2}s", "✅".green(), command, duration.as_secs_f64());
                    Ok(())
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    metrics.add_custom_metric("execution_time_ms".to_string(), duration.as_millis() as u64);
                    metrics.add_custom_metric("success".to_string(), 0);
                    println!("{} Error ejecutando cargo {} (interactivo): {}", "❌".red(), command, e);
                    Err(e)
                }
            }
        } else {
            let total_units = crate::utils::progress::estimate_cargo_units().max(1);
            let progress_bar = ProgressBar::new(total_units as u64);
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} {pos}/{len} {wide_bar:.cyan/blue} {msg}")
                    .expect("progress template"),
            );
            progress_bar.enable_steady_tick(Duration::from_millis(120));
            progress_bar.set_message(format!("Ejecutando cargo {}...", command));
            let mut completed = 0usize;
            let result = executor
                .execute_streaming_capture_with_handler(&arg_refs, |stream, line| {
                    let show_line = verbose
                        || matches!(stream, crate::core::cargo::CargoStream::Stderr)
                        || line.contains("error:")
                        || line.contains("warning:");
                    if show_line {
                        match stream {
                            crate::core::cargo::CargoStream::Stdout => println!("{line}"),
                            crate::core::cargo::CargoStream::Stderr => eprintln!("{line}"),
                        }
                    }
                    if line.contains("Compiling ") {
                        completed = (completed + 1).min(total_units);
                        progress_bar.set_position(completed as u64);
                        progress_bar.set_message(line.to_string());
                    } else if line.starts_with("Finished ") {
                        progress_bar.set_position(total_units as u64);
                        progress_bar.set_message(line.to_string());
                    }
                })
                .await;
            match result {
                Ok(_) => {
                    progress_bar.finish_with_message("Cargo completado");
                    let duration = start_time.elapsed();
                    metrics.add_custom_metric("execution_time_ms".to_string(), duration.as_millis() as u64);
                    metrics.add_custom_metric("success".to_string(), 1);
                    metrics.add_custom_metric("streaming_mode".to_string(), 1);
                    if !no_jarvix {
                        if let Ok(Some(client)) = crate::jarvix::client::JarvixClient::new() {
                            if let Err(e) = client.report_cargo_metrics(metrics.clone()).await {
                                eprintln!("⚠️ No se pudo reportar métricas cargo a JARVIXSERVER: {e}");
                            }
                        }
                    }
                    println!("{} Comando cargo {} completado en {:.2}s", "✅".green(), command, duration.as_secs_f64());
                    Ok(())
                }
                Err(e) => {
                    progress_bar.abandon_with_message("Cargo falló");
                    let duration = start_time.elapsed();
                    metrics.add_custom_metric("execution_time_ms".to_string(), duration.as_millis() as u64);
                    metrics.add_custom_metric("success".to_string(), 0);
                    println!("{} Error ejecutando cargo {}: {}", "❌".red(), command, e);
                    Err(e)
                }
            }
        }
    }
}
