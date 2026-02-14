use crate::cli::TraeCli;
use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use std::process::Stdio;

/// `trae copilot` — wrapper mínimo para GitHub Copilot CLI / `gh copilot`.
/// - modo `--dry-run` para pruebas sin ejecutar el binario (útil en CI local).
#[derive(Args, Debug, Default)]
pub struct CopilotCommand {
    /// Prompt simple para enviar a Copilot (ej. "Resume README.md")
    #[arg(short, long)]
    pub prompt: Option<String>,

    /// Passthrough arguments to the underlying `copilot` CLI (or `gh copilot`).
    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,

    /// Do not actually execute the external CLI; print the resolved command.
    #[arg(long)]
    pub dry_run: bool,

    /// Model to use when invoking Copilot directly
    #[arg(long, default_value = "gemini-3-pro-preview")]
    pub model: String,
}

impl CopilotCommand {
    fn resolve_command_and_args(&self) -> (String, Vec<String>) {
        // If `copilot` exists in PATH, prefer it; otherwise use `gh copilot --`
        if which::which("copilot").is_ok() {
            if let Some(prompt) = &self.prompt {
                let mut a = vec!["-p".to_string(), prompt.clone(), "--model".to_string(), self.model.clone()];
                return ("copilot".to_string(), a);
            }
            if !self.args.is_empty() {
                return ("copilot".to_string(), self.args.clone());
            }
            return ("copilot".to_string(), vec!["--help".to_string()]);
        }
        // fallback to gh copilot -- <args>
        if let Some(prompt) = &self.prompt {
            let mut a = vec!["copilot".to_string(), "-p".to_string(), prompt.clone(), "--model".to_string(), self.model.clone()];
            return ("gh".to_string(), a);
        }
        if !self.args.is_empty() {
            let mut a = vec!["copilot".to_string()];
            a.extend(self.args.clone());
            return ("gh".to_string(), a);
        }
        ("gh".to_string(), vec!["copilot".to_string(), "--help".to_string()])
    }

    pub async fn execute(&self, _cli: &TraeCli) -> Result<()> {
        println!("{}","🤖 TRAE Copilot - wrapper".cyan().bold());
        let (exe, args) = self.resolve_command_and_args();
        if self.dry_run {
            println!("[dry-run] Would run: {} {}", exe, args.join(" "));
            return Ok(());
        }

        let mut cmd = std::process::Command::new(&exe);
        for a in &args {
            cmd.arg(a);
        }
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let status = cmd
            .spawn()
            .with_context(|| format!("failed to spawn '{}', ensure Copilot CLI or gh is installed", exe))?
            .wait()
            .with_context(|| format!("failed waiting for '{}'{exe}"))?;

        if !status.success() {
            Err(anyhow::anyhow!("Copilot command exited with status: {}", status))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_with_prompt_prefers_copilot_or_gh() {
        let cmd = CopilotCommand { prompt: Some("Hello".into()), args: vec![], dry_run: true, model: "gpt-5".into() };
        let (exe, args) = cmd.resolve_command_and_args();
        // exe should be either 'copilot' or 'gh' depending on PATH; we only assert args contains the prompt
        assert!(args.iter().any(|a| a == "Hello"));
    }

    #[test]
    fn test_dry_run_output_format() {
        let cmd = CopilotCommand { prompt: Some("TST".into()), args: vec![], dry_run: true, model: "gpt-5".into() };
        let (exe, args) = cmd.resolve_command_and_args();
        assert!(args.contains(&"TST".to_string()));
        assert!(args.contains(&"gpt-5".to_string()) || args.contains(&"--model".to_string()));
    }
}
