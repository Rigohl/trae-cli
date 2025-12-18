#![doc = " # System Doctor - System health check"]
#![doc = ""]
#![doc = " Verificador de salud del sistema y dependencias"]
use anyhow::Result;
use colored::Colorize;
use which::which;
#[doc = "Function documentation added by AI refactor"]
pub async fn run_system_check() -> Result<()> {
    println!(
        "{}",
        "🩺 TRAE System Doctor - Verificación del Sistema"
            .cyan()
            .bold()
    );
    println!();
    let mut all_ok = true;
    all_ok &= check_rust_installation();
    all_ok &= check_cargo_installation();
    all_ok &= check_additional_tools();
    all_ok &= check_jarvix_connection().await?;
    println!();
    if all_ok {
        println!(
            "{}",
            "✅ Todos los checks pasaron exitosamente".green().bold()
        );
    } else {
        println!(
            "{}",
            "⚠️ Algunos checks fallaron. Ver detalles arriba."
                .yellow()
                .bold()
        );
    }
    Ok(())
}
#[doc = "Function documentation added by AI refactor"]
fn check_rust_installation() -> bool {
    print!("🦀 Verificando instalación de Rust... ");
    if let Ok(path) = which("rustc") {
        println!("{}", "✓".green());
        println!("   Ruta: {}", path.display().to_string().blue());
        if let Ok(output) = std::process::Command::new("rustc")
            .arg("--version")
            .output()
        {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("   Versión: {}", version.trim().blue());
        }
        true
    } else {
        println!("{}", "✗ No encontrado".red());
        println!("   💡 Instalar desde: https://rustup.rs/");
        false
    }
}
#[doc = "Function documentation added by AI refactor"]
fn check_cargo_installation() -> bool {
    print!("📦 Verificando instalación de Cargo... ");
    if let Ok(path) = which("cargo") {
        println!("{}", "✓".green());
        println!("   Ruta: {}", path.display().to_string().blue());
        if let Ok(output) = std::process::Command::new("cargo")
            .arg("--version")
            .output()
        {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("   Versión: {}", version.trim().blue());
        }
        true
    } else {
        println!("{}", "✗ No encontrado".red());
        false
    }
}
#[doc = "Function documentation added by AI refactor"]
fn check_additional_tools() -> bool {
    let tools = vec![
        ("clippy", "cargo install clippy"),
        ("rustfmt", "rustup component add rustfmt"),
    ];
    let mut all_ok = true;
    for (tool, install_cmd) in tools {
        print!("🔧 Verificando {tool}... ");
        let found = if tool == "clippy" || tool == "rustfmt" {
            std::process::Command::new("cargo")
                .args([tool, "--help"])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        } else {
            which(tool).is_ok()
        };
        if found {
            println!("{}", "✓".green());
        } else {
            println!("{}", "✗ No encontrado".red());
            println!("   💡 Instalar: {}", install_cmd.yellow());
            all_ok = false;
        }
    }
    all_ok
}
#[doc = "Function documentation added by AI refactor"]
async fn check_jarvix_connection() -> Result<bool> {
    print!("🌐 Verificando conexión a JARVIXSERVER... ");
    match crate::jarvix::client::JarvixClient::new() {
        Ok(Some(client)) => {
            let test_metrics =
                crate::metrics::collector::MetricsCollector::new("health_check".to_string());
            match client.report_build_metrics(test_metrics).await {
                Ok(()) => {
                    println!("{}", "✅ Conectado y respondiendo".green());
                    Ok(true)
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!("⚠️ Configurado pero sin respuesta: {e}").yellow()
                    );
                    println!("   💡 Verificar que JARVIXSERVER esté ejecutándose");
                    Ok(true)
                }
            }
        }
        Ok(None) => {
            println!("{}", "⚠️ No configurado".yellow());
            println!("   💡 Ejecutar: trae metrics --configure");
            Ok(true)
        }
        Err(e) => {
            println!("{}", format!("❌ Error de conexión: {e}").red());
            println!("   💡 Verificar configuración en ~/.trae/config.toml");
            Ok(false)
        }
    }
}
