use anyhow::Result;
use cargo_metadata::MetadataCommand as CargoMetadataCommand;
use clap::Args;
use std::fs;

#[derive(Args, Debug)]
pub struct TraeMetadataCommand {
    #[arg(long, value_name = "PATH", help = "Write metadata JSON to file")]
    pub output: Option<String>,
    #[arg(long, help = "Include lines-of-code counts (may be slow)")]
    pub include_loc: bool,
    #[arg(long, help = "Include dependency list")]
    pub include_deps: bool,
    #[arg(long, help = "Verbose output")]
    pub verbose: bool,
}

impl TraeMetadataCommand {
    pub async fn execute(&self, _cli: &crate::cli::TraeCli) -> Result<()> {
        // Fetch cargo metadata
        let meta = CargoMetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!(e))?;
        let mut out = serde_json::json!({
            "workspace_root": meta.workspace_root,
            "packages": [],
            "rustc_version": null,
        });
        // packages
        let pkgs: Vec<_> = meta
            .packages
            .iter()
            .map(|package| {
                serde_json::json!({
                    "name": package.name,
                    "version": package.version.to_string(),
                    "id": package.id.to_string(),
                    "manifest_path": package.manifest_path.to_string()
                })
            })
            .collect();
        out["packages"] = serde_json::Value::Array(pkgs);

        // Try to get rustc version
        if let Ok(r) = std::process::Command::new("rustc")
            .arg("--version")
            .output()
        {
            if r.status.success() {
                out["rustc_version"] = serde_json::Value::String(
                    String::from_utf8_lossy(&r.stdout).trim().to_string(),
                );
            }
        }

        if self.include_deps {
            let deps: Vec<_> = meta
                .packages
                .iter()
                .flat_map(|package| package.dependencies.iter().map(move |dependency| serde_json::json!({"pkg": package.name, "dep": dependency.name, "req": dependency.req.to_string()})))
                .collect();
            out["dependencies"] = serde_json::Value::Array(deps);
        }

        if self.include_loc {
            // Count lines in src/**/*.rs
            let mut total = 0usize;
            for entry in walkdir::WalkDir::new(".")
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| {
                    entry.path().is_file()
                        && entry
                            .path()
                            .extension()
                            .map(|ext| ext == "rs")
                            .unwrap_or(false)
                })
            {
                if let Ok(s) = fs::read_to_string(entry.path()) {
                    total += s.lines().count();
                }
            }
            out["loc"] = serde_json::Value::Number(serde_json::Number::from(total));
        }

        let serialized = serde_json::to_string_pretty(&out)?;
        if let Some(path) = &self.output {
            fs::write(path, &serialized)?;
            println!("Metadata guardada en {}", path);
        } else {
            println!("{}", serialized);
        }
        Ok(())
    }
}
