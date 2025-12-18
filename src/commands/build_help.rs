use anyhow::Result;
use clap::Args;
use std::process::Command;

#[derive(Args, Debug)]
pub struct BuildHelpCommand {
    #[arg(long, help = "Optimize for binary size (enables opt-level=s, strip)")]
    pub optimize_size: bool,
    #[arg(long, help = "Actually run the recommended cargo build")]
    pub run: bool,
    #[arg(long, help = "Build --release")]
    pub release: bool,
    #[arg(long, value_name = "TARGET", help = "Optional target triple to build for")]
    pub target: Option<String>,
    #[arg(long, help = "Verbose output")]
    pub verbose: bool,
}

impl BuildHelpCommand {
    pub async fn execute(&self, _cli: &crate::cli::TraeCli) -> Result<()> {
        // Minimal, sober suggestions
        println!("TRAE Build Helper - recomendaciones sobrias para compilar");
        if self.optimize_size {
            println!(" • Recomendación: optimizar tamaño: opt-level = 's', lto = true, strip símbolos");
        } else {
            println!(" • Recomendación: para rendimiento, usar --release con opt-level=3 y LTO si aplica");
        }
        if let Some(t) = &self.target {
            println!(" • Target objetivo: {}", t);
        }
        println!(" • Sugerencia: deshabilitar incremental en CI para artefactos reproducibles");

        if self.run {
            // build command composition
            let mut cmd = Command::new("cargo");
            cmd.arg("build");
            if self.release {
                cmd.arg("--release");
            }
            if let Some(t) = &self.target {
                cmd.args(&["--target", t]);
            }
            if self.optimize_size {
                // set env RUSTFLAGS for size optimizations
                cmd.env("RUSTFLAGS", "-C opt-level=s -C link-arg=-s");
            }
            if self.verbose {
                println!("Ejecutando: {:?}", cmd);
            }
            let status = cmd.status()?;
            if status.success() {
                println!("Build completado ✓");
                Ok(())
            } else {
                Err(anyhow::anyhow!("cargo build falló con estado {:?}", status.code()))
            }
        } else {
            println!("Para ejecutar la recomendación añade --run");
            Ok(())
        }
    }
}
