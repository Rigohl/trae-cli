#![doc = " # Rustup passthrough command"]
#![doc = ""]
#![doc = " Delegado sencillo a `rustup` para integración total con la CLI oficial de Rust."]
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use std::env;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::task;
use which::which;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct RustupCommand {
    #[doc = " rustup subcommand to execute"]
    #[arg(value_name = "COMMAND")]
    pub command: String,
    #[doc = " Additional arguments for rustup"]
    #[arg(last = true, trailing_var_arg = true, value_name = "ARGS")]
    pub args: Vec<String>,
    #[doc = " Run command interactively (inherit stdio)"]
    #[arg(long)]
    pub interactive: bool,
}
impl RustupCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self) -> Result<()> {
        println!(
            "{}",
            format!("🚀 Ejecutando rustup {}...", self.command)
                .cyan()
                .bold()
        );
        let program = match resolve_executable("rustup") {
            Some(p) => p,
            None => {
                eprintln ! ("❌ 'rustup' no se encuentra en PATH ni en RUSTUP_HOME/CARGO_HOME. Instálalo: https://www.rust-lang.org/tools/install");
                return Err(anyhow::anyhow!("rustup not found"));
            }
        };
        let mut arg_strings = Vec::new();
        arg_strings.push(self.command.clone());
        arg_strings.extend(self.args.clone());
        let arg_refs: Vec<&str> = arg_strings.iter().map(|s| s.as_str()).collect();
        if self.interactive {
            self.execute_interactive(&program, &arg_refs).await
        } else {
            self.execute_streaming(&program, &arg_refs).await
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn execute_interactive(&self, program: &str, args: &[&str]) -> Result<()> {
        let mut cmd = TokioCommand::new(program);
        cmd.args(args);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd.stdin(Stdio::inherit());
        let status = cmd.status().await?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "rustup failed with code: {:?}",
                status.code()
            ))
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn execute_streaming(&self, program: &str, args: &[&str]) -> Result<()> {
        let mut cmd = TokioCommand::new(program);
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut child = cmd.spawn()?;
        let mut handles = Vec::new();
        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout);
            handles.push(task::spawn(async move {
                let mut buffer = String::new();
                let mut out = String::new();
                while reader.read_line(&mut buffer).await? != 0 {
                    print!("{}", buffer);
                    out.push_str(&buffer);
                    buffer.clear();
                }
                Ok::<String, anyhow::Error>(out)
            }));
        }
        if let Some(stderr) = child.stderr.take() {
            let mut reader = BufReader::new(stderr);
            handles.push(task::spawn(async move {
                let mut buffer = String::new();
                let mut out = String::new();
                while reader.read_line(&mut buffer).await? != 0 {
                    eprint!("{}", buffer);
                    out.push_str(&buffer);
                    buffer.clear();
                }
                Ok::<String, anyhow::Error>(out)
            }));
        }
        for handle in handles {
            let _ = handle.await;
        }
        let status = child.wait().await?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "rustup failed with code: {:?}",
                status.code()
            ))
        }
    }
}
#[doc = "Function documentation added by AI refactor"]
fn resolve_executable(name: &str) -> Option<String> {
    if let Ok(path) = which(name) {
        return Some(path.to_string_lossy().to_string());
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(rustup_home) = env::var_os("RUSTUP_HOME") {
        candidates.push(PathBuf::from(rustup_home).join("bin").join(name));
    }
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
