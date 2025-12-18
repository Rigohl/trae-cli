#![doc = " # Cargo Executor - Enhanced cargo command execution"]
#![doc = ""]
#![doc = " Executor mejorado para comandos cargo con métricas y análisis"]
use anyhow::Result;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::task;
#[derive(Debug, Clone, Copy)]
pub enum CargoStream {
    Stdout,
    Stderr,
}
#[doc = "Struct documentation added by AI refactor"]
pub struct CargoExecutor {
    working_dir: Option<std::path::PathBuf>,
}
impl CargoExecutor {
    #[doc = "Method documentation added by AI refactor"]
    pub const fn new() -> Self {
        Self { working_dir: None }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn with_working_dir<P: Into<std::path::PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute_with_output(
        &self,
        args: &[impl AsRef<std::ffi::OsStr>],
    ) -> Result<String> {
        let mut cmd = TokioCommand::new("cargo");
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let output = cmd.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = if stdout.is_empty() {
            stderr.to_string()
        } else if stderr.is_empty() {
            stdout.to_string()
        } else {
            format!("{stdout}\n{stderr}")
        };
        if output.status.success() {
            Ok(combined)
        } else {
            Err(anyhow::anyhow!("Cargo command failed:\n{combined}"))
        }
    }
    #[doc = " Ejecuta cargo mostrando stdout/stderr en vivo (streaming)."]
    pub async fn execute_streaming(&self, args: &[impl AsRef<std::ffi::OsStr>]) -> Result<()> {
        let mut cmd = TokioCommand::new("cargo");
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(args);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let status = cmd.status().await?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Cargo command failed with exit code: {:?}",
                status.code()
            ))
        }
    }
    #[doc = " Ejecuta cargo streameando stdout/stderr y captura el output combinado."]
    pub async fn execute_streaming_capture(
        &self,
        args: &[impl AsRef<std::ffi::OsStr>],
    ) -> Result<String> {
        let mut cmd = TokioCommand::new("cargo");
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut child = cmd.spawn()?;
        let mut combined = String::new();
        let mut handles = Vec::new();
        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout);
            handles.push(task::spawn(async move {
                let mut buffer = String::new();
                let mut output = String::new();
                while reader.read_line(&mut buffer).await? != 0 {
                    print!("{}", buffer);
                    output.push_str(&buffer);
                    buffer.clear();
                }
                Ok::<String, anyhow::Error>(output)
            }));
        }
        if let Some(stderr) = child.stderr.take() {
            let mut reader = BufReader::new(stderr);
            handles.push(task::spawn(async move {
                let mut buffer = String::new();
                let mut output = String::new();
                while reader.read_line(&mut buffer).await? != 0 {
                    eprint!("{}", buffer);
                    output.push_str(&buffer);
                    buffer.clear();
                }
                Ok::<String, anyhow::Error>(output)
            }));
        }
        for handle in handles {
            if let Ok(result) = handle.await {
                combined.push_str(&result?);
            }
        }
        let status = child.wait().await?;
        if status.success() {
            Ok(combined)
        } else {
            Err(anyhow::anyhow!(
                "Cargo command failed with exit code: {:?}\n{combined}",
                status.code()
            ))
        }
    }
    #[doc = " Ejecuta cargo con stdout/stderr piped, permitiendo manejar cada línea (para progreso/UX)."]
    pub async fn execute_streaming_capture_with_handler<F>(
        &self,
        args: &[impl AsRef<std::ffi::OsStr>],
        mut on_line: F,
    ) -> Result<String>
    where
        F: FnMut(CargoStream, &str) + Send,
    {
        let mut cmd = TokioCommand::new("cargo");
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut child = cmd.spawn()?;
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let mut out_lines = stdout.map(|s| BufReader::new(s).lines());
        let mut err_lines = stderr.map(|s| BufReader::new(s).lines());
        let mut combined = String::new();
        let mut stdout_done = out_lines.is_none();
        let mut stderr_done = err_lines.is_none();
        while !(stdout_done && stderr_done) {
            tokio::select! { out = async { if let Some (lines) = & mut out_lines { lines . next_line () . await } else { Ok (None) } } , if ! stdout_done => { match out ? { Some (line) => { on_line (CargoStream :: Stdout , & line) ; combined . push_str (& line) ; combined . push ('\n') ; } None => stdout_done = true , } } err = async { if let Some (lines) = & mut err_lines { lines . next_line () . await } else { Ok (None) } } , if ! stderr_done => { match err ? { Some (line) => { on_line (CargoStream :: Stderr , & line) ; combined . push_str (& line) ; combined . push ('\n') ; } None => stderr_done = true , } } }
        }
        let status = child.wait().await?;
        if status.success() {
            Ok(combined)
        } else {
            Err(anyhow::anyhow!(
                "Cargo command failed with exit code: {:?}\n{combined}",
                status.code()
            ))
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute_interactive(&self, args: &[impl AsRef<std::ffi::OsStr>]) -> Result<()> {
        let mut cmd = TokioCommand::new("cargo");
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(args);
        let status = cmd.status().await?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Cargo command failed with exit code: {:?}",
                status.code()
            ))
        }
    }
}
