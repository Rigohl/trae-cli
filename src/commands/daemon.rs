#![doc = " # Daemon Command - Launch trae-server silently in background"]
#![doc = ""]
#![doc = " Inicia el binario `trae-server` en segundo plano, con opción de silenciar"]
#![doc = " stdout/stderr o redirigir a un archivo de log."]
use crate::cli::TraeCli;
use anyhow::{anyhow, Context, Result};
use clap::Args;
use colored::Colorize;
use std::fs::File;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct DaemonCommand {
    #[doc = " Ruta del binario a ejecutar (por defecto usa trae-server en PATH)"]
    #[arg(long, default_value = "trae-server")]
    pub binary: String,
    #[doc = " Puerto para el server (exportado como PORT)"]
    #[arg(long, default_value_t = 3001)]
    pub port: u16,
    #[doc = " Archivo de log donde redirigir stdout/stderr"]
    #[arg(long)]
    pub log: Option<String>,
    #[doc = " Silenciar completamente la salida (ignora stdout/stderr)"]
    #[arg(long)]
    pub quiet: bool,
}
impl DaemonCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self, _cli: &TraeCli) -> Result<()> {
        println!(
            "{}",
            format!(
                "🚀 Lanzando {} en segundo plano (puerto {})...",
                self.binary, self.port
            )
            .cyan()
            .bold()
        );

        let mut cmd = Command::new(&self.binary);
        cmd.env("PORT", self.port.to_string());
        match (&self.log, self.quiet) {
            (Some(path_str), _) => {
                let path = Path::new(path_str);

                // Security check: Prevent path traversal
                if path.is_absolute()
                    || path.components().any(|c| matches!(c, std::path::Component::ParentDir))
                {
                    return Err(anyhow!("Ruta de log inválida: {}. No se permiten rutas absolutas o con '..'", path_str));
                }

                let log_file = File::create(path)
                    .with_context(|| format!("No se pudo abrir log en {}", path_str))?;
                let stdout = Stdio::from(log_file.try_clone()?);
                let stderr = Stdio::from(log_file);
                cmd.stdout(stdout);
                cmd.stderr(stderr);
            }
            (None, true) => {
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::null());
            }
            _ => {
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());
            }
        }
        let child = cmd.spawn().context("No se pudo iniciar trae-server")?;
        let pid = child
            .id()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "desconocido".to_string());
        println ! ("{}" , format ! ("✅ trae-server iniciado (pid {}) en background. Usa Ctrl+C para detener el CLI; el server sigue activo." , pid) . green ());
        Ok(())
    }
}
