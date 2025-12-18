#![doc = " # Watch Command"]
#![doc = ""]
#![doc = " Observa cambios en el filesystem y re-ejecuta comandos con un resumen moderno."]
use crate::core::cargo::CargoExecutor;
use anyhow::{anyhow, Context, Result};
use clap::Args;
use colored::Colorize;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::time;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct WatchCommand {
    #[doc = " Comando principal (ej. 'check', 'test', o 'cargo check')"]
    #[arg(value_name = "CMD")]
    pub command: String,
    #[doc = " Argumentos adicionales para el comando"]
    #[arg(
        value_name = "ARGS",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub args: Vec<String>,
    #[doc = " Rutas a observar (por defecto src/ y Cargo.toml)"]
    #[arg(long, value_delimiter = ',')]
    pub paths: Vec<PathBuf>,
    #[doc = " Tiempo de debounce en ms"]
    #[arg(long, default_value_t = 300)]
    pub debounce_ms: u64,
    #[doc = " Saltar ejecución inicial (por defecto corre una vez al comenzar)"]
    #[arg(long)]
    pub skip_initial: bool,
}
impl WatchCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self) -> Result<()> {
        let mut watch_paths = if self.paths.is_empty() {
            vec![PathBuf::from("src"), PathBuf::from("Cargo.toml")]
        } else {
            self.paths.clone()
        };
        watch_paths.sort();
        watch_paths.dedup();
        println!(
            "{}",
            format!(
                "👀 Watch activo en: {}",
                watch_paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .cyan()
        );
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let mut watcher = new_watcher(tx).context("No se pudo crear watcher")?;
        for path in &watch_paths {
            watcher
                .watch(path, RecursiveMode::Recursive)
                .with_context(|| format!("No se pudo observar {}", path.display()))?;
        }
        let mut run_counter = 0usize;
        if !self.skip_initial {
            run_counter += 1;
            let report = self.run_once(run_counter).await?;
            self.print_summary(&report);
        }
        loop {
            rx.recv().await;
            let debounce = Duration::from_millis(self.debounce_ms);
            time::sleep(debounce).await;
            while rx.try_recv().is_ok() {}
            run_counter += 1;
            let report = self.run_once(run_counter).await?;
            self.print_summary(&report);
            println!("{}", "⌛ Esperando cambios...".dimmed());
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_once(&self, run_no: usize) -> Result<RunReport> {
        let start = Instant::now();
        let command_display = if self.args.is_empty() {
            self.command.clone()
        } else {
            format!("{} {}", self.command, self.args.join(" "))
        };
        println!(
            "{}",
            format!("⚙️  Run #{run_no:02} → {command_display}").bold()
        );
        let exec_result = if self.command == "cargo" || self.command.starts_with("cargo ") {
            let mut parts: Vec<String> = self
                .command
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            if !parts.is_empty() && parts[0] == "cargo" {
                parts.remove(0);
            }
            parts.extend(self.args.clone());
            CargoExecutor::new()
                .execute_streaming(&parts)
                .await
                .context("Fallo comando cargo")
        } else if self.command.starts_with('-') {
            Err(anyhow!(
                "Comando inválido para watch: {} (usa 'cargo <subcmd>' o '<subcmd>')",
                self.command
            ))
        } else {
            let mut parts = vec![self.command.clone()];
            parts.extend(self.args.clone());
            CargoExecutor::new()
                .execute_streaming(&parts)
                .await
                .context("Fallo comando cargo")
        };
        let duration = start.elapsed();
        match exec_result {
            Ok(()) => Ok(RunReport {
                run_no,
                command_display,
                duration,
                success: true,
                error: None,
            }),
            Err(e) => Ok(RunReport {
                run_no,
                command_display,
                duration,
                success: false,
                error: Some(e.to_string()),
            }),
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn print_summary(&self, report: &RunReport) {
        println!("{}", "┌────────────────────────────────┐".dimmed());
        println!(
            "{} {:<28} {}",
            "│".dimmed(),
            format!(
                "Run #{:02} • {:<15}",
                report.run_no,
                truncate(&report.command_display, 15)
            ),
            if report.success {
                "✅".green()
            } else {
                "❌".red()
            }
        );
        println!(
            "{} {:<28} │",
            "│".dimmed(),
            format!("Tiempo {:>6.2}s", report.duration.as_secs_f64()).bold()
        );
        if let Some(err) = &report.error {
            println!(
                "{} {}",
                "│".dimmed(),
                format!("Error: {}", truncate(err, 45)).red()
            );
        }
        println!("{}", "└────────────────────────────────┘".dimmed());
    }
}
#[doc = "Struct documentation added by AI refactor"]
struct RunReport {
    run_no: usize,
    command_display: String,
    duration: Duration,
    success: bool,
    error: Option<String>,
}
#[doc = "Function documentation added by AI refactor"]
fn truncate(input: &str, len: usize) -> String {
    if input.chars().count() <= len {
        input.to_string()
    } else {
        input
            .chars()
            .take(len.saturating_sub(1))
            .collect::<String>()
            + "…"
    }
}
#[doc = "Function documentation added by AI refactor"]
fn new_watcher(tx: tokio::sync::mpsc::UnboundedSender<()>) -> notify::Result<RecommendedWatcher> {
    notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if res.is_ok() {
            let _ = tx.send(());
        }
    })
}
