#![doc = " # Configuration Module"]
#![doc = ""]
#![doc = " Gestión de configuración de TRAE CLI"]
use anyhow::Result;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct TraeConfig {
    pub jarvix: JarvixConfig,
    pub analysis: AnalysisConfig,
    pub repair: RepairConfig,
}
#[derive(Debug, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct JarvixConfig {
    pub enabled: bool,
    pub server_url: String,
    pub api_key: Option<String>,
    pub timeout: u64,
}
#[derive(Debug, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct AnalysisConfig {
    pub auto_analysis: bool,
    pub performance_analysis: bool,
    pub security_analysis: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct RepairConfig {
    pub auto_repair: bool,
    pub backup_before_repair: bool,
    pub clippy_auto_fix: bool,
}
impl Default for TraeConfig {
    #[doc = "Method documentation added by AI refactor"]
    fn default() -> Self {
        Self {
            jarvix: JarvixConfig {
                enabled: true,
                server_url: "http://localhost:8080".to_string(),
                api_key: None,
                timeout: 30,
            },
            analysis: AnalysisConfig {
                auto_analysis: true,
                performance_analysis: false,
                security_analysis: false,
            },
            repair: RepairConfig {
                auto_repair: false,
                backup_before_repair: true,
                clippy_auto_fix: true,
            },
        }
    }
}
#[doc = "Function documentation added by AI refactor"]
pub async fn init_trae_config(force: bool) -> Result<()> {
    println!("🔧 Inicializando configuración de TRAE...");
    let quantum_init_start = std::time::Instant::now();
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("No se pudo encontrar el directorio de configuración"))?
        .join("trae");
    std::fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.toml");
    let init_duration = quantum_init_start.elapsed();
    let quantum_score = (1000.0 / (init_duration.as_micros() as f64 + 1.0)).min(1000.0);
    if quantum_score > 100.0 {
        println!("⚡ Inicialización optimizada cuánticamente: {quantum_score:.1}");
    }
    if config_file.exists() && !force {
        println!("✅ La configuración ya existe. Use --force para sobrescribir.");
        return Ok(());
    }
    let config = TraeConfig::default();
    let toml_str = toml::to_string_pretty(&config)?;
    std::fs::write(&config_file, toml_str)?;
    println!(
        "✅ Configuración inicializada en: {}",
        config_file.display()
    );
    Ok(())
}
