use crate::{cli::TraeCli, core::cargo::CargoExecutor};
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use serde_json::Value;

/// `trae diagnostics` — ejecuta `cargo check|build --message-format=json`,
/// parsea mensajes y muestra `file:line:col` con opción de abrir en editor.
#[derive(Args, Debug, Default)]
pub struct DiagnosticsCommand {
    /// Use `cargo build` instead of `cargo check`
    #[arg(long)]
    pub build: bool,

    /// Open each primary diagnostic location in the editor (uses `code -g` by default)
    #[arg(long)]
    pub open: bool,

    /// Editor command to use when `--open` (default: `code`)
    #[arg(long, value_name = "EDITOR")]
    pub editor: Option<String>,

    /// Print aggregated diagnostics as JSON to stdout
    #[arg(long)]
    pub json: bool,

    /// Include non-primary spans found in compiler messages
    #[arg(long)]
    pub include_all_spans: bool,
}

impl DiagnosticsCommand {
    async fn run_inner(&self) -> Result<()> {
        println!("{}", "🔎 TRAE Diagnostics - parsing cargo JSON output".cyan().bold());
        let cmd = if self.build { "build" } else { "check" };
        let args = &[cmd, "--message-format=json"] as &[&str];
        let executor = CargoExecutor::new();

        let mut errors = 0usize;
        let mut warnings = 0usize;
        let mut collected: Vec<Value> = Vec::new();

        let editor_cmd = self.editor.clone().unwrap_or_else(|| "code".to_string());

        let res = executor
            .execute_streaming_capture_with_handler(args, |_, line| {
                // Try parse JSON message per-line (Cargo emits JSON objects line-by-line)
                if let Ok(val) = serde_json::from_str::<Value>(line) {
                    if val.get("reason").and_then(Value::as_str) == Some("compiler-message") {
                        let m = &val["message"];
                        let level = m.get("level").and_then(Value::as_str).unwrap_or("info");
                        if level == "error" {
                            errors += 1;
                        } else if level == "warning" {
                            warnings += 1;
                        }
                        if let Some(spans) = m.get("spans").and_then(Value::as_array) {
                            for span in spans.iter() {
                                let is_primary = span.get("is_primary").and_then(Value::as_bool).unwrap_or(false);
                                if !is_primary && !self.include_all_spans {
                                    continue;
                                }
                                let file = span.get("file_name").and_then(Value::as_str).unwrap_or("<unknown>");
                                let line_start = span.get("line_start").and_then(Value::as_u64).unwrap_or(0);
                                let col_start = span.get("column_start").and_then(Value::as_u64).unwrap_or(0);
                                let msg_text = m.get("message").and_then(Value::as_str).unwrap_or("");
                                match level {
                                    "error" => eprintln!("{} {}:{}:{} — {}", "[ERROR]".red().bold(), file, line_start, col_start, msg_text),
                                    "warning" => println!("{} {}:{}:{} — {}", "[WARN]".yellow().bold(), file, line_start, col_start, msg_text),
                                    _ => println!("{} {}:{}:{} — {}", "[INFO]".blue().bold(), file, line_start, col_start, msg_text),
                                }
                                if self.open && (is_primary || self.include_all_spans) {
                                    // best-effort, do not block on editor
                                    let _ = std::process::Command::new(&editor_cmd)
                                        .args(["-g", &format!("{}:{}:{}", file, line_start, col_start)])
                                        .spawn();
                                }
                            }
                        }
                        if self.json {
                            collected.push(val.clone());
                        }
                    }
                } else {
                    // not JSON — show raw cargo diagnostics lines if they contain error/warning
                    if line.contains("error:") || line.contains("warning:") {
                        println!("{line}");
                    }
                }
            })
            .await;

        match res {
            Ok(_) => {
                println!("\n✅ Resumen: {} errores, {} warnings", errors, warnings);
                if self.json {
                    let out = serde_json::to_string_pretty(&collected)?;
                    println!("\nJSON OUTPUT:\n{}", out);
                }
                if errors > 0 {
                    Err(anyhow::anyhow!("Found {} error(s)", errors))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    /// API-friendly runner usable desde otros entrypoints
    pub async fn run_simple(opts: DiagnosticsCommand) -> Result<()> {
        opts.run_inner().await
    }

    pub async fn execute(&self, _cli: &TraeCli) -> Result<()> {
        self.run_inner().await
    }
}
