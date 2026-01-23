#![doc = " # Math Command - Mathematical Analysis with Julia Workers"]
#![doc = ""]
#![doc = " Comando de análisis matemático usando workers Julia de JARVIXSERVER"]
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use serde_json::json;
#[doc = " Mathematical analysis command"]
#[derive(Args, Debug)]
pub struct MathCommand {
    #[doc = " Type of mathematical analysis"]
    #[arg(short, long, default_value = "optimization")]
    analysis_type: String,
    #[doc = " Input data file"]
    #[arg(short, long)]
    input: Option<String>,
    #[doc = " Output format"]
    #[arg(short, long, default_value = "json")]
    format: String,
}
impl MathCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self, jarvix_cli: &crate::cli::JarvixCli) -> Result<()> {
        println!(
            "{}",
            "🔬 TRAE MATH ANALYSIS - Análisis Matemático con Julia"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "==============================================\n".cyan()
        );
        let jarvix_client = if jarvix_cli.no_jarvix {
            println!("❌ JARVIXSERVER requerido para análisis matemático");
            return Ok(());
        } else {
            crate::jarvix::client::JarvixClient::new().ok().flatten()
        };
        let client = match jarvix_client {
            Some(c) => c,
            None => {
                println!("❌ No se pudo conectar a JARVIXSERVER");
                return Ok(());
            }
        };
        let input_data = if let Some(input_file) = &self.input {
            if let Ok(content) = std::fs::read_to_string(input_file) {
                json ! ({ "file_content" : content , "file_path" : input_file })
            } else {
                json ! ({ "sample_data" : [1.0 , 2.0 , 3.0 , 4.0 , 5.0] })
            }
        } else {
            json ! ({ "sample_data" : [1.0 , 2.0 , 3.0 , 4.0 , 5.0] })
        };
        let job_data = json ! ({ "analysis_type" : self . analysis_type , "input_data" : input_data , "output_format" : self . format });
        println!("📤 Enviando análisis matemático a worker Julia...");
        println!("🔢 Tipo: {}", self.analysis_type);
        println!("📊 Datos: {} elementos", input_data.to_string().len());
        match client
            .submit_parallel_analysis_job("math_optimization", job_data)
            .await
        {
            Ok(job_id) => {
                println!("✅ Job enviado: {job_id}");
                println!("⏳ Esperando resultado del worker Julia...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                match client.get_job_result(&job_id).await {
                    Ok(Some(result)) => {
                        println!("🎯 Resultado recibido:");
                        println!("{}", serde_json::to_string_pretty(&result)?);
                        if let Some(output_file) = &self.input {
                            let output_path = format!("{}_result.{}", output_file, self.format);
                            std::fs::write(&output_path, serde_json::to_string_pretty(&result)?)?;
                            println!("💾 Resultado guardado en: {output_path}");
                        }
                    }
                    Ok(None) => {
                        println!("⏳ Job aún en proceso...");
                    }
                    Err(e) => {
                        println!("❌ Error obteniendo resultado: {e}");
                    }
                }
            }
            Err(e) => {
                println!("❌ Error enviando job: {e}");
            }
        }
        Ok(())
    }
}
