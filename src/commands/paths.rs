use anyhow::Result;
use clap::Args;
use colored::Colorize;
use serde_json::json;
use std::{fs, path::PathBuf};
use tokio::task;
use walkdir::WalkDir;
const RUST_EXTENSION: &str = "rs";
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct PathsCommand {
    #[doc = " Paths or directories to check"]
    # [arg (value_name = "PATHS" , num_args = 1 ..)]
    pub paths: Vec<String>,
    #[doc = " Output as JSON"]
    #[arg(long)]
    pub json: bool,
    #[doc = " Run cargo check on found crates (not yet implemented)"]
    #[arg(long)]
    pub cargo_check: bool,
}
impl PathsCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self) -> Result<()> {
        let mut handles = Vec::with_capacity(self.paths.len());
        for path in &self.paths {
            let path = path.clone();
            handles.push(task::spawn_blocking(move || analyze_path(&path)));
        }
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(value)) => results.push(value),
                Ok(Err(err)) => results.push(json ! ({ "error" : err . to_string () })),
                Err(err) => results.push(json ! ({ "error" : format ! ("task panicked: {err}") })),
            }
        }
        if self.json {
            println!("{}", serde_json::to_string_pretty(&results)?);
        } else {
            render_human_readable(&results);
        }
        if self.cargo_check {
            println!(
                "{}",
                "⚠️  --cargo-check requested but running cargo check is not implemented.".yellow()
            );
        }
        Ok(())
    }
}
#[doc = "Function documentation added by AI refactor"]
fn analyze_path(path_str: &str) -> Result<serde_json::Value> {
    let path = PathBuf::from(path_str);
    if !path.exists() {
        return Ok(json ! ({ "path" : path_str , "exists" : false }));
    }
    let mut file_entries = Vec::new();
    if path.is_file() {
        file_entries.push(path);
    } else {
        for entry in WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.path().is_file())
        {
            if entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == RUST_EXTENSION)
                .unwrap_or(false)
            {
                file_entries.push(entry.into_path());
            }
        }
    }
    let mut files_report = Vec::new();
    for file in file_entries {
        let file_display = file.to_string_lossy().to_string();
        let content = match fs::read_to_string(&file) {
            Ok(content) => content,
            Err(err) => {
                files_report
                    .push(json ! ({ "file" : file_display , "read_error" : err . to_string () , }));
                continue;
            }
        };
        let todo_count = count_occurrences(&content, "TODO");
        let unwrap_count = count_occurrences(&content, "unwrap()");
        let panic_count = count_occurrences(&content, "panic!");
        match syn :: parse_file (& content) { Ok (_) => files_report . push (json ! ({ "file" : file_display , "parse_ok" : true , "todo_count" : todo_count , "unwrap_count" : unwrap_count , "panic_count" : panic_count })) , Err (err) => files_report . push (json ! ({ "file" : file_display , "parse_ok" : false , "parse_error" : err . to_string () , "todo_count" : todo_count , "unwrap_count" : unwrap_count , "panic_count" : panic_count })) , }
    }
    Ok(json ! ({ "path" : path_str , "exists" : true , "files" : files_report }))
}
#[doc = "Function documentation added by AI refactor"]
fn count_occurrences(haystack: &str, needle: &str) -> u64 {
    haystack.matches(needle).count() as u64
}
#[doc = "Function documentation added by AI refactor"]
fn render_human_readable(results: &[serde_json::Value]) {
    for entry in results {
        let Some(path) = entry.get("path").and_then(|v| v.as_str()) else {
            println!("{}", json_to_string(entry).yellow());
            continue;
        };
        match entry.get("exists").and_then(|v| v.as_bool()) {
            Some(true) => println!("{} {}", "✔".green(), path),
            Some(false) => {
                println!("{} {} does not exist", "✖".red(), path);
                continue;
            }
            None => {
                println!("{}", json_to_string(entry).yellow());
                continue;
            }
        }
        if let Some(files) = entry.get("files").and_then(|v| v.as_array()) {
            for file_entry in files {
                let file = file_entry
                    .get("file")
                    .and_then(|v| v.as_str())
                    .unwrap_or("<unknown>");
                if let Some(read_error) = file_entry.get("read_error") {
                    println!(
                        "  {} {} -> {}",
                        "!".red(),
                        file,
                        read_error.as_str().unwrap_or("read error")
                    );
                    continue;
                }
                if let Some(false) = file_entry.get("parse_ok").and_then(|v| v.as_bool()) {
                    let err = file_entry
                        .get("parse_error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("parse error");
                    println!("  {} {} -> {}", "!".red(), file, err);
                } else {
                    let todos = value_to_u64(file_entry.get("todo_count"));
                    let unwraps = value_to_u64(file_entry.get("unwrap_count"));
                    let panics = value_to_u64(file_entry.get("panic_count"));
                    println!(
                        "  {} {} (TODOs: {todos}, unwraps: {unwraps}, panics: {panics})",
                        "-".cyan(),
                        file
                    );
                }
            }
        }
    }
}
#[doc = "Function documentation added by AI refactor"]
fn json_to_string(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<invalid json>".to_string())
}
#[doc = "Function documentation added by AI refactor"]
fn value_to_u64(value: Option<&serde_json::Value>) -> u64 {
    value.and_then(|v| v.as_u64()).unwrap_or(0)
}
