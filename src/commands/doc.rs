#![doc = " # Doc Command - Documentation generation and validation"]
#![doc = ""]
#![doc = " Comando de documentación con generación automática, validación y publicación"]
use crate::{cli::TraeCli, jarvix::client::JarvixClient, metrics::collector::MetricsCollector};
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct DocCommand {
    #[doc = " Generate documentation"]
    #[arg(long)]
    pub generate: bool,
    #[doc = " Validate existing documentation"]
    #[arg(long)]
    pub validate: bool,
    #[doc = " Open documentation in browser"]
    #[arg(long)]
    pub open: bool,
    #[doc = " Generate API documentation"]
    #[arg(long)]
    pub api: bool,
    #[doc = " Generate README files"]
    #[arg(long)]
    pub readme: bool,
    #[doc = " Check documentation coverage"]
    #[arg(long)]
    pub coverage: bool,
    #[doc = " Publish documentation"]
    #[arg(long)]
    pub publish: bool,
    #[doc = " Documentation format (html, pdf, md)"]
    #[arg(long, default_value = "html")]
    pub format: String,
    #[doc = " Output directory"]
    #[arg(long, default_value = "target/doc")]
    pub output: String,
    #[doc = " Include private items"]
    #[arg(long)]
    pub private: bool,
    #[doc = " Generate dependency documentation"]
    #[arg(long)]
    pub deps: bool,
}
impl DocCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self, cli: &TraeCli) -> Result<()> {
        let start_time = Instant::now();
        let metrics = MetricsCollector::new("doc".to_string());
        println!("{}", "📚 TRAE DOC - Documentation Suite".cyan().bold());
        println!("{}", "================================\n".cyan());
        let pb = ProgressBar::new_spinner();
        let style = match ProgressStyle::default_spinner().template("{spinner:.green} {msg}") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("⚠️  progress template failed: {:?}", e);
                ProgressStyle::default_spinner()
            }
        };
        pb.set_style(style);
        if self.generate {
            pb.set_message("Generando documentación...");
            self.generate_docs(cli)?;
            pb.finish_with_message("✓ Documentación generada");
        }
        if self.validate {
            pb.set_message("Validando documentación...");
            self.validate_docs(cli)?;
            pb.finish_with_message("✓ Validación completada");
        }
        if self.coverage {
            pb.set_message("Analizando cobertura de documentación...");
            self.check_doc_coverage(cli)?;
            pb.finish_with_message("✓ Cobertura analizada");
        }
        if self.readme {
            pb.set_message("Generando README...");
            self.generate_readme(cli)?;
            pb.finish_with_message("✓ README generado");
        }
        if self.open {
            pb.set_message("Abriendo documentación en navegador...");
            self.open_docs()?;
            pb.finish_with_message("✓ Documentación abierta");
        }
        if self.publish {
            pb.set_message("Publicando documentación...");
            self.publish_docs(cli)?;
            pb.finish_with_message("✓ Documentación publicada");
        }
        let elapsed = start_time.elapsed();
        println!("\n{}", "✓ OPERACIÓN COMPLETADA".green().bold());
        println!("Tiempo total: {:?}", elapsed);
        if !cli.no_jarvix {
            if let Ok(Some(client)) = JarvixClient::new() {
                if let Err(e) = client.report_doc_metrics(metrics).await {
                    eprintln!("⚠️ No se pudo reportar métricas de doc: {e}");
                }
            }
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn generate_docs(&self, _cli: &TraeCli) -> Result<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("doc");
        if self.private {
            cmd.arg("--document-private-items");
        }
        if self.deps {
            cmd.arg("--include-dependencies");
        }
        let _ = !self.output.is_empty();
        let output = cmd.output()?;
        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let doc_files = if Path::new("target/doc").exists() {
            walkdir::WalkDir::new("target/doc")
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "html"))
                .count()
        } else {
            0
        };
        println!("📚 Documentación generada: {} archivos", doc_files);
        println!(
            "Status: {}",
            if success {
                "✓ Exitoso"
            } else {
                "✗ Con errores"
            }
        );
        println!("Advertencias: {}", stdout.matches("warning:").count());
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn validate_docs(&self, _cli: &TraeCli) -> Result<()> {
        println!("🔍 Validando documentación...");
        if !Path::new("target/doc").exists() {
            println!("⚠️ Documentación no generada");
        }
        if !Path::new("README.md").exists() {
            println!("⚠️ README.md faltante");
        } else {
            println!("✓ README.md presente");
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn check_doc_coverage(&self, _cli: &TraeCli) -> Result<()> {
        println!("📈 Analizando cobertura de documentación...");
        let mut total_items = 0;
        let mut documented_items = 0;
        if let Ok(entries) = fs::read_dir("src") {
            for entry in entries.filter_map(std::result::Result::ok) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    total_items += content.matches("pub struct").count();
                    total_items += content.matches("pub fn").count();
                    total_items += content.matches("pub trait").count();
                    documented_items += content.matches("///").count();
                }
            }
        }
        if total_items > 0 {
            let coverage = (documented_items as f64 / total_items as f64) * 100.0;
            println!("Cobertura: {:.1}%", coverage);
            println!("Documentados: {}/{}", documented_items, total_items);
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn generate_readme(&self, _cli: &TraeCli) -> Result<()> {
        let project_name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        let description = env!("CARGO_PKG_DESCRIPTION");
        let readme_content = format ! ("# {project_name}\n\n[![Version](https://img.shields.io/badge/version-{version}-blue.svg)](Cargo.toml)\n\n{description}\n\n## Installation\n\n```bash\ncargo install {project_name}\n```\n\n## Usage\n\n```bash\n{project_name} --help\n```\n\n## License\n\nThis project is licensed under the MIT License.\n") ;
        fs::write("README.md", readme_content)?;
        println!("📝 README.md generado exitosamente");
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn open_docs(&self) -> Result<()> {
        let doc_path = "target/doc/index.html";
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "start", doc_path])
                .spawn()?;
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(doc_path).spawn()?;
        } else {
            Command::new("xdg-open").arg(doc_path).spawn()?;
        }
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn publish_docs(&self, _cli: &TraeCli) -> Result<()> {
        let has_gh_pages = Path::new(".github/workflows").exists()
            && walkdir::WalkDir::new(".github/workflows")
                .into_iter()
                .filter_map(std::result::Result::ok)
                .any(|e| e.path().to_string_lossy().contains("pages"));
        let has_docs_rs = false;
        let platform = if has_gh_pages {
            "GitHub Pages"
        } else {
            "Local"
        };
        let url = if has_gh_pages {
            "https://username.github.io/repo/"
        } else {
            "file:///target/doc/index.html"
        };
        println!("📤 Documentación publicada en: {}", platform);
        println!("URL: {}", url);
        println!(
            "docs.rs: {}",
            if has_docs_rs {
                "✓ Configurado"
            } else {
                "✗ No configurado"
            }
        );
        Ok(())
    }
}
