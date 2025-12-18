#![doc = " # Documentation Utils - Helper functions for documentation"]
#![doc = ""]
#![doc = " Utilidades para mostrar documentación y ayuda"]
use anyhow::Result;
use colored::Colorize;
#[doc = "Function documentation added by AI refactor"]
pub async fn show_cargo_commands() -> Result<()> {
    println!(
        "{}",
        "📖 CARGO COMMANDS - Documentación Oficial de Rust"
            .cyan()
            .bold()
    );
    println!();
    let cargo_commands_content = include_str!("../../CARGO_COMMANDS.md");
    let lines: Vec<&str> = cargo_commands_content.lines().collect();
    for line in lines.iter().take(50) {
        if line.starts_with("# ") {
            println!("{}", line.cyan().bold());
        } else if line.starts_with("## ") {
            println!("{}", line.yellow().bold());
        } else if line.starts_with("### ") {
            println!("{}", line.green().bold());
        } else if line.starts_with("- **") || line.starts_with("* **") {
            println!("{}", line.blue());
        } else {
            println!("{line}");
        }
    }
    if lines.len() > 50 {
        println!();
        println!(
            "{}",
            "📄 Documentación completa disponible en: CARGO_COMMANDS.md".yellow()
        );
        println!(
            "{}",
            "💡 Use 'trae help-cargo | less' para navegación completa".blue()
        );
    }
    Ok(())
}
