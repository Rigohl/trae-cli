use crate::cli::TraeCli;
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use regex::Regex;
use serde_json::json;
use std::fs;

#[derive(Args, Debug, Default)]
pub struct CodeHealthCommand {
    /// Print results as JSON
    #[arg(long)]
    pub json: bool,
    /// Open each finding in editor (uses `code`)
    #[arg(long)]
    pub open: bool,
}

impl CodeHealthCommand {
    pub async fn execute(&self, _cli: &TraeCli) -> Result<()> {
        println!("{}", "🔍 TRAE Code‑Health - scanning source files".cyan().bold());
        let patterns = vec![
            ("unwrap", Regex::new(r"\bunwrap\s*\(").unwrap()),
            ("expect", Regex::new(r"\bexpect\s*\(").unwrap()),
            ("panic", Regex::new(r"\bpanic!\s*\(").unwrap()),
            ("unsafe", Regex::new(r"\bunsafe\b").unwrap()),
            ("TODO", Regex::new(r"\bTODO\b").unwrap()),
            ("unwrap_or_else", Regex::new(r"\bunwrap_or_else\s*\(").unwrap()),
        ];
        let mut findings: Vec<serde_json::Value> = Vec::new();
        for entry in walkdir::WalkDir::new(".")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|s| s == "rs").unwrap_or(false))
        {
            let p = entry.path();
            if p.components().any(|c| c.as_os_str() == "target") { continue; }
            if let Ok(content) = fs::read_to_string(p) {
                for (i, line) in content.lines().enumerate() {
                    for (name, re) in &patterns {
                        if re.is_match(line) {
                            let obj = json!({
                                "file": p.to_string_lossy(),
                                "line": i+1,
                                "text": line.trim(),
                                "pattern": name
                            });
                            findings.push(obj.clone());
                            println!("[ISSUE] {}:{}:{} - {}", p.to_string_lossy(), i+1, re.find(line).map(|m| m.start()+1).unwrap_or(1), line.trim());
                            if self.open {
                                let _ = std::process::Command::new("code").args(["-g", &format!("{}:{}", p.to_string_lossy(), i+1)]).spawn();
                            }
                        }
                    }
                }
            }
        }
        if self.json {
            println!("{}", serde_json::to_string_pretty(&findings)?);
        }
        if findings.is_empty() {
            println!("{} No issues found", "✅".green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Found {} code-health issues", findings.len()))
        }
    }

    /// API wrapper
    pub async fn run_simple(opts: CodeHealthCommand) -> Result<()> {
        opts.execute(&crate::cli::TraeCli { verbose: false, config: None, no_jarvix: true, command: crate::cli::Commands::CommandsGuide }).await
    }
}
