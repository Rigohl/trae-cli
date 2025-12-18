#![doc = " # Repair Command - Automatic code repair and fixing"]
#![doc = ""]
#![doc = " Comando para reparar automáticamente issues comunes en proyectos Rust"]
use crate::{
    cli::TraeCli,
    core::{analyzer::ProjectAnalyzer, cargo::CargoExecutor},
    jarvix::client::JarvixClient,
    metrics::collector::MetricsCollector,
    utils::ui::{print_step_table, StepSummary},
};
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::{Duration, Instant};
use which::which;
#[doc = " Six Sigma Repair Command - Sistema de reparación automática de defectos"]
#[doc = ""]
#[doc = " Implementa metodología DMAIC (Define, Measure, Analyze, Improve, Control)"]
#[doc = " para reparación automática de issues detectados en código Rust."]
#[doc = ""]
#[doc = " # Six Sigma Principles Applied:"]
#[doc = " - **Define**: Identifica tipos específicos de defectos"]
#[doc = " - **Measure**: Cuantifica issues antes y después"]
#[doc = " - **Analyze**: Determina causa raíz de problemas"]
#[doc = " - **Improve**: Aplica reparaciones automáticas"]
#[doc = " - **Control**: Valida que las mejoras persistan"]
#[doc = ""]
#[doc = " # Quality Assurance:"]
#[doc = " - 0 `unwrap()` calls"]
#[doc = " - Comprehensive error handling"]
#[doc = " - Rollback capability on failures"]
#[doc = " - Metrics collection for continuous improvement"]
#[derive(Args, Debug, Default)]
pub struct RepairCommand {
    #[doc = " Run all automatic repairs (Six Sigma: comprehensive process improvement)"]
    #[arg(long)]
    pub auto: bool,
    #[doc = " Run cargo clippy fixes (Six Sigma: defect detection and correction)"]
    #[arg(long)]
    pub clippy: bool,
    #[doc = " Run cargo fmt (Six Sigma: standardization for quality)"]
    #[arg(long)]
    pub fmt: bool,
    #[doc = " Update and fix dependencies (Six Sigma: supply chain optimization)"]
    #[arg(long)]
    pub deps: bool,
    #[doc = " Fix Cargo.toml issues (Six Sigma: configuration management)"]
    #[arg(long)]
    pub manifest: bool,
    #[doc = " Clean and rebuild"]
    #[arg(long)]
    pub clean: bool,
    #[doc = " Fix documentation issues"]
    #[arg(long)]
    pub docs: bool,
    #[doc = " Fix test issues"]
    #[arg(long)]
    pub tests: bool,
    #[doc = " Export repair summary as JSON (for CI)"]
    #[arg(long, value_name = "PATH")]
    pub export: Option<String>,
    #[doc = " Run cargo check after repairs to validate"]
    #[arg(long, default_value = "true")]
    pub check: bool,
    #[doc = " Try to detect outdated deps with cargo-outdated if installed"]
    #[arg(long)]
    pub outdated: bool,
    #[doc = " Dry run - show what would be fixed"]
    #[arg(long)]
    pub dry_run: bool,
    #[doc = " Force repairs without confirmation"]
    #[arg(long, default_value = "true")]
    pub force: bool,
    #[doc = "Repair level: safe, balanced, aggressive"]
    #[arg(long, value_name = "LEVEL")]
    pub level: Option<String>,
    #[doc = "Create backup and rollback on failure"]
    #[arg(long)]
    pub rollback: bool,
    #[doc = "Run `cargo update` to update lockfile/deps"]
    #[arg(long)]
    pub update: bool,
    #[doc = "Run `cargo upgrade` (requires cargo-edit) to bump dependency versions"]
    #[arg(long)]
    pub upgrade: bool,
    #[doc = "Create a git branch before applying changes"]
    #[arg(long, value_name = "BRANCH")]
    pub git_branch: Option<String>,
    #[doc = "Create a git commit with message after repairs"]
    #[arg(long, value_name = "MSG")]
    pub git_commit: Option<String>,
}
impl RepairCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self, cli: &TraeCli) -> Result<()> {
        info!("?? Iniciando proceso de reparaci¢n autom tica");
        let total_start = Instant::now();
        let mut metrics = MetricsCollector::new("repair".to_string());
        let mut steps = Vec::new();
        let mut fatal_error: Option<anyhow::Error> = None;
        let mut repair_results: Vec<RepairResult> = Vec::new();
        let mut category_durations: HashMap<IssueCategory, Duration> = HashMap::new();
        let mut repair_stage_duration = Duration::default();
        let mut post_check: Option<PostCheckOutcome> = None;
        let mut repairs_planned = false;
        let mut repairs_executed = false;
        self.show_repair_config();
        // Ensure we run from the workspace root so repairs work from any subdir
        let orig_cwd = std::env::current_dir()?;
        let mut root = orig_cwd.clone();
        let mut found = false;
        while !root.join("Cargo.toml").exists() {
            if !root.pop() {
                break;
            }
        }
        if root.join("Cargo.toml").exists() {
            found = true;
        }
        if found {
            let _ = std::env::set_current_dir(&root);
        }
        let detection_start = Instant::now();
        let issues = match self.detect_issues().await {
            Ok(list) => {
                steps.push(StepSummary::success(
                    "Detecci¢n de issues",
                    detection_start.elapsed(),
                ));
                list
            }
            Err(e) => {
                let msg = e.to_string();
                steps.push(StepSummary::failed(
                    "Detecci¢n de issues",
                    detection_start.elapsed(),
                    msg,
                ));
                fatal_error = Some(e);
                Vec::new()
            }
        };
        if fatal_error.is_none() && issues.is_empty() {
            println!(
                "{}",
                "? No se encontraron issues que reparar".green().bold()
            );
        }
        if fatal_error.is_none() && !issues.is_empty() {
            self.show_detected_issues(&issues);
            repairs_planned = true;
        }
        let confirm_label = "Confirmaci¢n de reparaci¢n";
        if fatal_error.is_none() {
            if issues.is_empty() {
                steps.push(StepSummary::skipped(confirm_label));
            } else if self.force || self.dry_run {
                steps.push(StepSummary::success(
                    "Confirmaci¢n autom tica",
                    Duration::default(),
                ));
            } else {
                let confirm_start = Instant::now();
                match self.confirm_repairs(&issues) {
                    Ok(true) => {
                        steps.push(StepSummary::success(confirm_label, confirm_start.elapsed()))
                    }
                    Ok(false) => {
                        println!("{}", "? Reparaci¢n cancelada por el usuario".yellow());
                        steps.push(StepSummary::failed(
                            confirm_label,
                            confirm_start.elapsed(),
                            "Cancelado por el usuario".to_string(),
                        ));
                        repairs_planned = false;
                    }
                    Err(e) => {
                        let msg = e.to_string();
                        steps.push(StepSummary::failed(
                            confirm_label,
                            confirm_start.elapsed(),
                            msg,
                        ));
                        fatal_error = Some(e);
                        repairs_planned = false;
                    }
                }
            }
        } else {
            steps.push(StepSummary::skipped(confirm_label));
        }
        let repair_label = if self.dry_run {
            "Simulaci¢n de reparaciones"
        } else {
            "Aplicar reparaciones"
        };
        if fatal_error.is_none() && repairs_planned && !issues.is_empty() {
            let repair_start = Instant::now();
            if self.dry_run {
                match self.simulate_repairs(&issues) {
                    Ok(results) => {
                        repair_results = results;
                        repair_stage_duration = repair_start.elapsed();
                        steps.push(StepSummary::success(repair_label, repair_stage_duration));
                        repairs_executed = true;
                    }
                    Err(e) => {
                        let msg = e.to_string();
                        steps.push(StepSummary::failed(
                            repair_label,
                            repair_start.elapsed(),
                            msg,
                        ));
                        fatal_error = Some(e);
                    }
                }
            } else {
                match self.execute_repairs(&issues).await {
                    Ok((results, durations)) => {
                        repair_stage_duration = repair_start.elapsed();
                        repair_results = results;
                        category_durations = durations;
                        steps.push(StepSummary::success(repair_label, repair_stage_duration));
                        repairs_executed = true;
                    }
                    Err(e) => {
                        let msg = e.to_string();
                        steps.push(StepSummary::failed(
                            repair_label,
                            repair_start.elapsed(),
                            msg,
                        ));
                        fatal_error = Some(e);
                    }
                }
            }
        } else {
            steps.push(StepSummary::skipped(repair_label));
        }
        if fatal_error.is_none() {
            self.append_phase_steps(
                &mut steps,
                &issues,
                &repair_results,
                if self.dry_run || category_durations.is_empty() {
                    None
                } else {
                    Some(&category_durations)
                },
            );
        }
        if fatal_error.is_none() && repairs_executed {
            self.show_results(&repair_results, repair_stage_duration);
        }
        // Optionally update/upgrade dependencies and commit changes
        if fatal_error.is_none() && repairs_executed {
            let executor = CargoExecutor::new();
            if self.update {
                let upd_start = Instant::now();
                match executor.execute_streaming(&["update"]).await {
                    Ok(_) => steps.push(StepSummary::success("Actualizar dependencias (cargo update)", upd_start.elapsed())),
                    Err(e) => steps.push(StepSummary::failed("Actualizar dependencias (cargo update)", upd_start.elapsed(), e.to_string())),
                }
            }
            if self.upgrade {
                let upg_start = Instant::now();
                match executor.execute_streaming(&["upgrade"]).await {
                    Ok(_) => steps.push(StepSummary::success("Upgrade deps (cargo upgrade)", upg_start.elapsed())),
                    Err(e) => steps.push(StepSummary::failed("Upgrade deps (cargo upgrade)", upg_start.elapsed(), e.to_string())),
                }
            }
            // Git operations: create branch and commit
            if let Some(branch) = &self.git_branch {
                let git_start = Instant::now();
                match std::process::Command::new("git").args(["checkout", "-b", branch]).output() {
                    Ok(o) if o.status.success() => steps.push(StepSummary::success(format!("Crear branch git: {}", branch), git_start.elapsed())),
                    Ok(o) => steps.push(StepSummary::failed(format!("Crear branch git: {}", branch), git_start.elapsed(), String::from_utf8_lossy(&o.stderr).to_string())),
                    Err(e) => steps.push(StepSummary::failed(format!("Crear branch git: {}", branch), git_start.elapsed(), e.to_string())),
                }
            }
            if let Some(msg) = &self.git_commit {
                let git_start = Instant::now();
                let add = std::process::Command::new("git").args(["add", "-A"]).output();
                if let Ok(a) = add {
                    if a.status.success() {
                        match std::process::Command::new("git").args(["commit", "-m", msg]).output() {
                            Ok(c) if c.status.success() => steps.push(StepSummary::success("Git commit", git_start.elapsed())),
                            Ok(c) => steps.push(StepSummary::failed("Git commit", git_start.elapsed(), String::from_utf8_lossy(&c.stderr).to_string())),
                            Err(e) => steps.push(StepSummary::failed("Git commit", git_start.elapsed(), e.to_string())),
                        }
                    } else {
                        steps.push(StepSummary::failed("Git add", git_start.elapsed(), String::from_utf8_lossy(&a.stderr).to_string()));
                    }
                } else if let Err(e) = add {
                    steps.push(StepSummary::failed("Git add", git_start.elapsed(), e.to_string()));
                }
            }
        }
        let check_label = "Cargo check";
        if self.check && !self.dry_run && fatal_error.is_none() && repairs_executed {
            let check_start = Instant::now();
            match self.run_post_check().await {
                Ok(outcome) => {
                    steps.push(StepSummary::success(check_label, check_start.elapsed()));
                    self.show_post_check(&outcome);
                    post_check = Some(outcome);
                }
                Err(e) => {
                    let msg = e.to_string();
                    steps.push(StepSummary::failed(check_label, check_start.elapsed(), msg));
                }
            }
        } else {
            steps.push(StepSummary::skipped(check_label));
        }
        let export_label = self.export_step_label();
        if let Some(path) = &self.export {
            if fatal_error.is_none() {
                let export_start = Instant::now();
                match self.export_report(
                    path,
                    &repair_results,
                    repair_stage_duration,
                    post_check.as_ref(),
                ) {
                    Ok(()) => {
                        steps.push(StepSummary::success(
                            export_label.clone(),
                            export_start.elapsed(),
                        ));
                        println!("? Reporte de reparaci¢n exportado a {}", path);
                    }
                    Err(e) => {
                        let msg = e.to_string();
                        steps.push(StepSummary::failed(
                            export_label.clone(),
                            export_start.elapsed(),
                            msg,
                        ));
                    }
                }
            } else {
                steps.push(StepSummary::skipped(export_label.clone()));
            }
        } else {
            steps.push(StepSummary::skipped(export_label));
        }
        metrics.record_repair_time(repair_stage_duration);
        metrics.record_repairs_applied(&repair_results);
        metrics.finish();
        if cli.no_jarvix {
            steps.push(StepSummary::skipped("Jarvix report"));
        } else if fatal_error.is_none() {
            let jarvix_start = Instant::now();
            match self.report_metrics(metrics.clone()).await {
                Ok(()) => steps.push(StepSummary::success(
                    "Jarvix report",
                    jarvix_start.elapsed(),
                )),
                Err(e) => {
                    let msg = e.to_string();
                    steps.push(StepSummary::failed(
                        "Jarvix report",
                        jarvix_start.elapsed(),
                        msg,
                    ));
                }
            }
        } else {
            steps.push(StepSummary::skipped("Jarvix report"));
        }
        let total_duration = total_start.elapsed();
        print_step_table("Repair Summary", &steps, total_duration);
        if let Some(err) = fatal_error {
            let _ = std::env::set_current_dir(orig_cwd);
            Err(err)
        } else {
            let _ = std::env::set_current_dir(orig_cwd);
            Ok(())
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn show_repair_config(&self) {
        println!("{}", "🔧 Configuración de Reparación:".cyan().bold());
        if self.auto {
            println!("  • Modo: {}", "Automático Completo".green());
        } else {
            let mut repairs = Vec::new();
            if self.clippy {
                repairs.push("Clippy");
            }
            if self.fmt {
                repairs.push("Formato");
            }
            if self.deps {
                repairs.push("Dependencias");
            }
            if self.manifest {
                repairs.push("Manifiesto");
            }
            if self.clean {
                repairs.push("Limpieza");
            }
            if self.docs {
                repairs.push("Documentación");
            }
            if self.tests {
                repairs.push("Tests");
            }
            if repairs.is_empty() {
                println!("  • Reparaciones: {}", "Ninguna seleccionada".red());
            } else {
                println!("  • Reparaciones: {}", repairs.join(", ").green());
            }
        }
        println!(
            "  • Post-check (cargo check): {}",
            if self.check {
                "Sí".green()
            } else {
                "No".yellow()
            }
        );
        println!(
            "  • Detectar outdated (cargo-outdated): {}",
            if self.outdated {
                "Sí".green()
            } else {
                "No".yellow()
            }
        );
        if let Some(path) = &self.export {
            println!("  • Exportar reporte: {}", path);
        }
        println!(
            "  • Dry Run: {}",
            if self.dry_run {
                "Sí".yellow()
            } else {
                "No".blue()
            }
        );
        println!(
            "  • Forzar: {}",
            if self.force {
                "Sí".red()
            } else {
                "No".green()
            }
        );
        println!();
    }
    #[doc = "Method documentation added by AI refactor"]
    fn export_step_label(&self) -> String {
        self.export
            .as_ref()
            .map(|path| format!("Export report ({path})"))
            .unwrap_or_else(|| "Export report".to_string())
    }
    #[doc = "Method documentation added by AI refactor"]
    fn append_phase_steps(
        &self,
        steps: &mut Vec<StepSummary>,
        issues: &[RepairIssue],
        results: &[RepairResult],
        durations: Option<&HashMap<IssueCategory, Duration>>,
    ) {
        for category in self.summary_categories(issues) {
            let label = Self::phase_label(category);
            if self.dry_run {
                steps.push(StepSummary::skipped(format!("{label} (dry-run)")));
                continue;
            }
            let cat_results: Vec<&RepairResult> = results
                .iter()
                .filter(|r| r.issue.category == category)
                .collect();
            if cat_results.is_empty() {
                steps.push(StepSummary::skipped(label));
                continue;
            }
            let duration = durations
                .and_then(|map| map.get(&category))
                .copied()
                .unwrap_or_default();
            if cat_results.iter().all(|r| r.success) {
                steps.push(StepSummary::success(label, duration));
            } else {
                let msg = cat_results
                    .into_iter()
                    .find(|r| !r.success)
                    .map(|r| r.message.clone())
                    .unwrap_or_else(|| "Fallo en reparacion".to_string());
                steps.push(StepSummary::failed(label, duration, msg));
            }
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn summary_categories(&self, issues: &[RepairIssue]) -> Vec<IssueCategory> {
        let mut wanted: HashSet<IssueCategory> = HashSet::new();
        for category in ISSUE_CATEGORY_ORDER {
            if self.phase_enabled(category) {
                wanted.insert(category);
            }
        }
        for issue in issues {
            wanted.insert(issue.category);
        }
        ISSUE_CATEGORY_ORDER
            .iter()
            .copied()
            .filter(|cat| wanted.contains(cat))
            .collect()
    }
    #[doc = "Method documentation added by AI refactor"]
    fn phase_enabled(&self, category: IssueCategory) -> bool {
        match category {
            IssueCategory::Clippy => self.auto || self.clippy,
            IssueCategory::Format => self.auto || self.fmt,
            IssueCategory::Dependencies => self.auto || self.deps,
            IssueCategory::Manifest => self.auto || self.manifest,
            IssueCategory::Documentation => self.auto || self.docs,
            IssueCategory::Tests => self.auto || self.tests,
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn phase_label(category: IssueCategory) -> &'static str {
        match category {
            IssueCategory::Clippy => "Clippy fixes",
            IssueCategory::Format => "Formato (cargo fmt)",
            IssueCategory::Dependencies => "Dependencias",
            IssueCategory::Manifest => "Manifest",
            IssueCategory::Documentation => "Documentacion",
            IssueCategory::Tests => "Tests",
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn detect_issues(&self) -> Result<Vec<RepairIssue>> {
        println!("{}", "🔍 Detectando issues...".cyan());
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Failed to set repair spinner template"),
        );
        spinner.set_message("Analizando proyecto...");
        let _analyzer = ProjectAnalyzer::new();
        let mut issues = Vec::new();
        if self.auto || self.clippy {
            spinner.set_message("Detectando issues de clippy...");
            issues.extend(self.detect_clippy_issues().await?);
        }
        if self.auto || self.fmt {
            spinner.set_message("Detectando issues de formato...");
            issues.extend(self.detect_format_issues().await?);
        }
        if self.auto || self.deps {
            spinner.set_message("Detectando issues de dependencias...");
            issues.extend(self.detect_dependency_issues()?);
        }
        if self.auto || self.manifest {
            spinner.set_message("Detectando issues del manifiesto...");
            issues.extend(self.detect_manifest_issues()?);
        }
        if self.auto || self.docs {
            spinner.set_message("Detectando issues de documentación...");
            issues.extend(self.detect_docs_issues()?);
        }
        if self.auto || self.tests {
            spinner.set_message("Detectando issues de tests...");
            issues.extend(self.detect_test_issues()?);
        }
        spinner.finish_with_message(format!("Detectados {} issues ✓", issues.len()));
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn detect_clippy_issues(&self) -> Result<Vec<RepairIssue>> {
        let executor = CargoExecutor::new();
        let output = executor
            .execute_with_output(&["clippy", "--", "-D", "warnings"])
            .await;
        let mut issues = Vec::new();
        if let Err(e) = output {
            if e.to_string().contains("warnings") || e.to_string().contains("error") {
                issues.push(RepairIssue {
                    category: IssueCategory::Clippy,
                    description: "Clippy warnings/errors detectados".to_string(),
                    severity: IssueSeverity::Warning,
                    fixable: true,
                    command: "cargo clippy --fix --allow-dirty --allow-no-vcs".to_string(),
                });
            }
        }
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn detect_format_issues(&self) -> Result<Vec<RepairIssue>> {
        let executor = CargoExecutor::new();
        let output = executor.execute_with_output(&["fmt", "--check"]).await;
        let mut issues = Vec::new();
        if output.is_err() {
            issues.push(RepairIssue {
                category: IssueCategory::Format,
                description: "Archivos con formato incorrecto detectados".to_string(),
                severity: IssueSeverity::Info,
                fixable: true,
                command: "cargo fmt".to_string(),
            });
        }
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn detect_dependency_issues(&self) -> Result<Vec<RepairIssue>> {
        let mut issues = Vec::new();
        if std::path::Path::new("Cargo.toml").exists() {
            let mut cmd = "cargo update".to_string();
            if self.outdated && which("cargo-outdated").is_ok() {
                cmd = "cargo outdated --root-deps-only".to_string();
            }
            let issue = RepairIssue {
                category: IssueCategory::Dependencies,
                description: "Dependencias desactualizadas - Revisar dependencias en Cargo.toml"
                    .to_string(),
                severity: IssueSeverity::Warning,
                fixable: true,
                command: cmd,
            };
            issues.push(issue);
        }
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn detect_manifest_issues(&self) -> Result<Vec<RepairIssue>> {
        let mut issues = Vec::new();
        if std::path::Path::new("Cargo.toml").exists() {
            if let Ok(content) = std::fs::read_to_string("Cargo.toml") {
                if !content.contains("[package]") {
                    let issue = RepairIssue {
                        category: IssueCategory::Manifest,
                        description: "Manifest incompleto - Falta sección [package] en Cargo.toml"
                            .to_string(),
                        severity: IssueSeverity::Critical,
                        fixable: false,
                        command: "echo 'Revisar Cargo.toml manualmente'".to_string(),
                    };
                    issues.push(issue);
                }
            }
        }
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn detect_docs_issues(&self) -> Result<Vec<RepairIssue>> {
        let mut issues = Vec::new();
        if !std::path::Path::new("README.md").exists() {
            let issue = RepairIssue {
                category: IssueCategory::Documentation,
                description: "Documentación faltante - No se encontró README.md".to_string(),
                severity: IssueSeverity::Warning,
                fixable: true,
                command: "echo '# Proyecto' > README.md".to_string(),
            };
            issues.push(issue);
        }
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn detect_test_issues(&self) -> Result<Vec<RepairIssue>> {
        let mut issues = Vec::new();
        if !std::path::Path::new("tests").exists() && !std::path::Path::new("src/lib.rs").exists() {
            let issue = RepairIssue {
                category: IssueCategory::Tests,
                description: "Tests faltantes - No se encontraron directorios de tests".to_string(),
                severity: IssueSeverity::Warning,
                fixable: true,
                command: "cargo test --no-run".to_string(),
            };
            issues.push(issue);
        }
        Ok(issues)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn show_detected_issues(&self, issues: &[RepairIssue]) {
        println!("{}", "📋 Issues Detectados:".yellow().bold());
        println!();
        for (i, issue) in issues.iter().enumerate() {
            let severity_icon = match issue.severity {
                IssueSeverity::Critical => "🔴",
                IssueSeverity::Warning => "🟡",
                IssueSeverity::Info => "🔵",
            };
            println!(
                "  {}. {} {} - {}",
                i + 1,
                severity_icon,
                format!("{:?}", issue.category).cyan(),
                issue.description
            );
            if issue.fixable {
                println!("     {}: {}", "Comando".green(), issue.command.blue());
            } else {
                println!("     {}", "No reparable automáticamente".red());
            }
            println!();
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn confirm_repairs(&self, issues: &[RepairIssue]) -> Result<bool> {
        use std::io::{self, Write};
        print!(
            "¿Proceder con la reparación de {} issues? (s/N): ",
            issues.len()
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_lowercase() == "s" || input.trim().to_lowercase() == "sí")
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn execute_repairs(
        &self,
        issues: &[RepairIssue],
    ) -> Result<(Vec<RepairResult>, HashMap<IssueCategory, Duration>)> {
        println!("{}", "🚀 Ejecutando reparaciones...".cyan());
        let style = match ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("⚠️  progress template failed: {:?}", e);
                ProgressStyle::default_bar()
            }
        };
        let progress = ProgressBar::new(issues.len() as u64);
        progress.set_style(style);
        let executor = CargoExecutor::new();
        let mut results = Vec::new();
        let mut durations: HashMap<IssueCategory, Duration> = HashMap::new();
        for issue in issues {
            progress.set_message(format!("Reparando: {:?}", issue.category));
            let issue_start = Instant::now();
            let result = if issue.fixable {
                let command_parts: Vec<&str> = issue.command.split_whitespace().collect();
                if command_parts.len() > 1 {
                    match executor.execute_streaming(&command_parts[1..]).await {
                        Ok(_) => RepairResult {
                            issue: issue.clone(),
                            success: true,
                            message: "Reparado exitosamente".to_string(),
                        },
                        Err(e) => RepairResult {
                            issue: issue.clone(),
                            success: false,
                            message: format!("Error: {e}"),
                        },
                    }
                } else {
                    RepairResult {
                        issue: issue.clone(),
                        success: false,
                        message: "Comando inválido".to_string(),
                    }
                }
            } else {
                RepairResult {
                    issue: issue.clone(),
                    success: false,
                    message: "No reparable automáticamente".to_string(),
                }
            };
            results.push(result);
            let elapsed = issue_start.elapsed();
            durations
                .entry(issue.category)
                .and_modify(|total| *total += elapsed)
                .or_insert(elapsed);
            progress.inc(1);
        }
        progress.finish_with_message("Reparaciones completadas ✓".to_string());
        Ok((results, durations))
    }
    #[doc = "Method documentation added by AI refactor"]
    fn simulate_repairs(&self, issues: &[RepairIssue]) -> Result<Vec<RepairResult>> {
        println!("{}", "🔍 Simulando reparaciones (dry run)...".yellow());
        let results = issues
            .iter()
            .map(|issue| RepairResult {
                issue: issue.clone(),
                success: issue.fixable,
                message: if issue.fixable {
                    format!("Se ejecutaría: {}", issue.command)
                } else {
                    "No reparable automáticamente".to_string()
                },
            })
            .collect();
        Ok(results)
    }
    #[doc = "Method documentation added by AI refactor"]
    fn show_results(&self, results: &[RepairResult], duration: std::time::Duration) {
        println!();
        println!("{}", "📊 Resultados de Reparación:".green().bold());
        println!();
        let successful = results.iter().filter(|r| r.success).count();
        let failed = results.len() - successful;
        println!(
            "  • Reparaciones exitosas: {}",
            successful.to_string().green()
        );
        println!("  • Reparaciones fallidas: {}", failed.to_string().red());
        println!("  • Tiempo total: {:.2}s", duration.as_secs_f64());
        println!();
        for result in results {
            let status_icon = if result.success { "✅" } else { "❌" };
            println!(
                "  {} {:?}: {}",
                status_icon, result.issue.category, result.message
            );
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_post_check(&self) -> Result<PostCheckOutcome> {
        let executor = CargoExecutor::new();
        let output = executor.execute_streaming_capture(&["check"]).await?;
        let warnings = output.matches("warning:").count();
        let errors = output.matches("error:").count();
        Ok(PostCheckOutcome {
            success: errors == 0,
            warnings,
            errors,
        })
    }
    #[doc = "Method documentation added by AI refactor"]
    fn show_post_check(&self, outcome: &PostCheckOutcome) {
        println!();
        println!(
            "{}",
            "✅ Validación post-repair (cargo check)".green().bold()
        );
        println!(
            "  ⚙️  Exitosa: {}",
            if outcome.success { "sí" } else { "no" }
        );
        println!("  ⚠️  Warnings: {}", outcome.warnings);
        println!("  ❌ Errores: {}", outcome.errors);
    }
    #[doc = "Method documentation added by AI refactor"]
    fn export_report(
        &self,
        path: &str,
        results: &[RepairResult],
        duration: std::time::Duration,
        post_check: Option<&PostCheckOutcome>,
    ) -> Result<()> {
        let report = json ! ({ "duration_seconds" : duration . as_secs_f64 () , "results" : results . iter () . map (| r | json ! ({ "category" : issue_category_name (& r . issue . category) , "severity" : issue_severity_name (& r . issue . severity) , "description" : r . issue . description , "command" : r . issue . command , "success" : r . success , "message" : r . message })) . collect ::< Vec < _ >> () , "post_check" : post_check . map (| c | json ! ({ "success" : c . success , "warnings" : c . warnings , "errors" : c . errors , })) });
        fs::write(path, serde_json::to_string_pretty(&report)?)?;
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn report_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        match JarvixClient::new() {
            Ok(Some(client)) => {
                client.report_repair_metrics(metrics).await?;
                println!(
                    "{}",
                    "📊 Métricas de reparación reportadas a JARVIXSERVER".green()
                );
            }
            Ok(None) => {
                println!("{}", "⚠️ JARVIXSERVER no configurado".yellow());
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Error conectando a JARVIXSERVER: {e}"));
            }
        }
        Ok(())
    }

}

/// Options for programmatic repair API.
#[derive(Debug, Clone, Default)]
pub struct RepairOptions {
    pub auto: bool,
    pub clippy: bool,
    pub fmt: bool,
    pub deps: bool,
    pub dry_run: bool,
    pub no_jarvix: bool,
    pub level: Option<String>,
    pub rollback: bool,
    pub update: bool,
    pub upgrade: bool,
    pub git_branch: Option<String>,
    pub git_commit: Option<String>,
}

impl RepairCommand {
    /// API-friendly wrapper to run repair flow programmatically.
    pub async fn run_simple(opts: RepairOptions) -> Result<()> {
        // Map level to flags if provided
        let (auto, clippy, fmt, deps) = if let Some(l) = opts.level.as_deref() {
            match l {
                "safe" => (false, true, true, false),
                "balanced" => (false, true, true, true),
                "aggressive" => (true, true, true, true),
                _ => (opts.auto, opts.clippy, opts.fmt, opts.deps),
            }
        } else {
            (opts.auto, opts.clippy, opts.fmt, opts.deps)
        };

        let cmd = RepairCommand {
            auto,
            clippy,
            fmt,
            deps,
            manifest: false,
            clean: false,
            docs: false,
            tests: false,
            export: None,
            check: true,
            outdated: false,
            dry_run: opts.dry_run,
            force: true,
            level: opts.level.clone(),
            rollback: opts.rollback,
            update: opts.update,
            upgrade: opts.upgrade,
            git_branch: opts.git_branch.clone(),
            git_commit: opts.git_commit.clone(),
        };

        // If rollback requested, create a simple backup copy of the workspace
        let backup_dir = if opts.rollback {
            let ts = chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
            let backup = std::path::Path::new(".trae").join("backups").join(format!("repair_{}", ts));
            if let Err(e) = std::fs::create_dir_all(&backup) {
                eprintln!("⚠️ No se pudo crear backup dir: {e}");
                None
            } else {
                // copy files recursively (skip .trae and target)
                fn copy_recursively(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                    for entry in std::fs::read_dir(src)? {
                        let entry = entry?;
                        let path = entry.path();
                        let rel = path.strip_prefix(src).unwrap_or(&path);
                        if rel.starts_with(".trae") || rel.starts_with("target") {
                            continue;
                        }
                        let dest_path = dst.join(rel);
                        if path.is_dir() {
                            std::fs::create_dir_all(&dest_path)?;
                            copy_recursively(&path, &dest_path)?;
                        } else if path.is_file() {
                            if let Some(parent) = dest_path.parent() {
                                std::fs::create_dir_all(parent)?;
                            }
                            std::fs::copy(&path, &dest_path)?;
                        }
                    }
                    Ok(())
                }
                let _ = copy_recursively(std::path::Path::new("."), &backup);
                Some(backup)
            }
        } else {
            None
        };

        // Build a minimal TraeCli to reuse the full execute flow
        let cli = crate::cli::TraeCli {
            verbose: false,
            config: None,
            no_jarvix: opts.no_jarvix,
            command: crate::cli::Commands::Repair(cmd),
        };
        // Execute the full flow by calling the command's execute directly to avoid recursion
        if let crate::cli::Commands::Repair(cmd_inner) = &cli.command {
            let res = cmd_inner.execute(&cli).await;
            if res.is_err() {
                if let Some(backup) = backup_dir {
                    eprintln!("⚠️ Error en reparacion, intentando rollback desde backup...");
                    // restore files (copy from backup over current)
                    fn restore_recursively(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                        for entry in std::fs::read_dir(src)? {
                            let entry = entry?;
                            let path = entry.path();
                            let rel = path.strip_prefix(src).unwrap_or(&path);
                            let dest_path = dst.join(rel);
                            if path.is_dir() {
                                std::fs::create_dir_all(&dest_path)?;
                                restore_recursively(&path, &dest_path)?;
                            } else if path.is_file() {
                                if let Some(parent) = dest_path.parent() {
                                    std::fs::create_dir_all(parent)?;
                                }
                                std::fs::copy(&path, &dest_path)?;
                            }
                        }
                        Ok(())
                    }
                    if let Err(e) = restore_recursively(&backup, std::path::Path::new(".")) {
                        eprintln!("⚠️ Rollback failed: {e}");
                    } else {
                        eprintln!("✅ Rollback completed from backup: {}", backup.to_string_lossy());
                    }
                }
            }
            res
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
#[doc = "Struct documentation added by AI refactor"]
pub struct RepairIssue {
    pub category: IssueCategory,
    pub description: String,
    pub severity: IssueSeverity,
    pub fixable: bool,
    pub command: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IssueCategory {
    Clippy,
    Format,
    Dependencies,
    Manifest,
    Documentation,
    Tests,
}
#[derive(Debug, Clone)]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}
#[derive(Debug, Clone)]
#[doc = "Struct documentation added by AI refactor"]
pub struct RepairResult {
    pub issue: RepairIssue,
    pub success: bool,
    pub message: String,
}
#[derive(Debug, Clone)]
#[doc = "Struct documentation added by AI refactor"]
pub struct PostCheckOutcome {
    pub success: bool,
    pub warnings: usize,
    pub errors: usize,
}
#[doc = "Function documentation added by AI refactor"]
fn issue_category_name(cat: &IssueCategory) -> &'static str {
    match cat {
        IssueCategory::Clippy => "clippy",
        IssueCategory::Format => "format",
        IssueCategory::Dependencies => "dependencies",
        IssueCategory::Manifest => "manifest",
        IssueCategory::Documentation => "documentation",
        IssueCategory::Tests => "tests",
    }
}
#[doc = "Function documentation added by AI refactor"]
fn issue_severity_name(sev: &IssueSeverity) -> &'static str {
    match sev {
        IssueSeverity::Critical => "critical",
        IssueSeverity::Warning => "warning",
        IssueSeverity::Info => "info",
    }
}
const ISSUE_CATEGORY_ORDER: [IssueCategory; 6] = [
    IssueCategory::Clippy,
    IssueCategory::Format,
    IssueCategory::Dependencies,
    IssueCategory::Manifest,
    IssueCategory::Documentation,
    IssueCategory::Tests,
];
