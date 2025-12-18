#![doc = " # MCP Command"]
#![doc = ""]
#![doc = " Inicia y controla procesos MCP personalizados en background."]
use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio::process::Command;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct McpCommand {
    #[command(subcommand)]
    pub action: McpActions,
}
#[derive(Subcommand, Debug)]
pub enum McpActions {
    #[doc = " Inicia un MCP en background"]
    Start {
        #[arg(long, default_value = "memory_p")]
        name: String,
        #[arg(long, default_value = "memory_p")]
        binary: String,
        #[arg(long, default_value_t = 4003)]
        port: u16,
        #[arg(long)]
        log: Option<PathBuf>,
        #[arg(long)]
        quiet: bool,
        #[arg(long)]
        args: Vec<String>,
    },
    #[doc = " Detiene un MCP por nombre o puerto"]
    Stop {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        port: Option<u16>,
    },
    #[doc = " Lista MCPs registrados"]
    List,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[doc = "Struct documentation added by AI refactor"]
struct McpProcess {
    name: String,
    pid: u32,
    port: u16,
    binary: String,
    log: Option<String>,
    started_at: String,
}
impl McpCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self) -> Result<()> {
        match &self.action {
            McpActions::Start {
                name,
                binary,
                port,
                log,
                quiet,
                args,
            } => {
                self.start(name, binary, *port, log.clone(), *quiet, args)
                    .await
            }
            McpActions::Stop { name, port } => self.stop(name.clone(), *port).await,
            McpActions::List => self.list().await,
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn start(
        &self,
        name: &str,
        binary: &str,
        port: u16,
        log: Option<PathBuf>,
        quiet: bool,
        extra_args: &[String],
    ) -> Result<()> {
        println!(
            "{}",
            format!("🚀 Iniciando MCP '{name}' en puerto {port}...").cyan()
        );
        let mut cmd = Command::new(binary);
        cmd.arg("--port").arg(port.to_string());
        cmd.args(extra_args);
        if let Some(log_path) = &log {
            let file = fs::File::create(log_path)
                .with_context(|| format!("No se pudo abrir log {}", log_path.display()))?;
            let stderr = file.try_clone()?;
            cmd.stdout(std::process::Stdio::from(file));
            cmd.stderr(std::process::Stdio::from(stderr));
        } else if quiet {
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
        }
        let child = cmd
            .spawn()
            .with_context(|| format!("No se pudo iniciar {binary}"))?;
        let pid = child.id().unwrap_or(0);
        std::mem::forget(child);
        let mut registry = load_registry()?;
        registry.retain(|entry| entry.name != name);
        registry.push(McpProcess {
            name: name.to_string(),
            pid,
            port,
            binary: binary.to_string(),
            log: log.map(|p| p.display().to_string()),
            started_at: chrono::Utc::now().to_rfc3339(),
        });
        save_registry(&registry)?;
        println!(
            "{}",
            format!("✅ MCP '{name}' iniciado (PID {pid})").green()
        );
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn stop(&self, name: Option<String>, port: Option<u16>) -> Result<()> {
        let mut registry = load_registry()?;
        if registry.is_empty() {
            println!("{}", "⚠️  No hay MCPs registrados.".yellow());
            return Ok(());
        }
        let mut target = registry
            .iter()
            .find(|entry| {
                name.as_ref().is_none_or(|n| &entry.name == n)
                    && port.is_none_or(|p| entry.port == p)
            })
            .cloned();
        if target.is_none() && name.is_none() && port.is_none() {
            target = registry.last().cloned();
        }
        let Some(process) = target else {
            println!("{}", "⚠️  No se encontró MCP con esos criterios.".yellow());
            return Ok(());
        };
        println!(
            "{}",
            format!(
                "🛑 Deteniendo MCP '{}' (PID {})...",
                process.name, process.pid
            )
            .yellow()
        );
        kill_process(process.pid)?;
        registry.retain(|entry| entry.pid != process.pid);
        save_registry(&registry)?;
        println!("{}", "✅ MCP detenido".green());
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn list(&self) -> Result<()> {
        let registry = load_registry()?;
        if registry.is_empty() {
            println!("{}", "ℹ️  No hay MCPs activos.".blue());
            return Ok(());
        }
        println!("{}", "📋 MCPs registrados:".bold());
        for entry in registry {
            println!(
                "  • {} (PID {}, puerto {}, binario {}, iniciado {})",
                entry.name, entry.pid, entry.port, entry.binary, entry.started_at
            );
        }
        Ok(())
    }
}
#[doc = "Function documentation added by AI refactor"]
fn registry_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".trae")
        .join("mcp_processes.json")
}
#[doc = "Function documentation added by AI refactor"]
fn load_registry() -> Result<Vec<McpProcess>> {
    let path = registry_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let entries = serde_json::from_str(&data)?;
    Ok(entries)
}
#[doc = "Function documentation added by AI refactor"]
fn save_registry(registry: &[McpProcess]) -> Result<()> {
    let path = registry_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, serde_json::to_string_pretty(registry)?)?;
    Ok(())
}
#[doc = "Function documentation added by AI refactor"]
fn kill_process(pid: u32) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .status()
            .context("taskkill falló")?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .status()
            .context("kill falló")?;
    }
    Ok(())
}
