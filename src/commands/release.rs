#![doc = " # Release Command"]
#![doc = ""]
#![doc = " Pipeline moderna: fmt check, clippy -D warnings, tests, build/package y SBOM opcional."]
use crate::{
    core::cargo::CargoExecutor,
    utils::ui::{print_step_table, StepSummary},
};
use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use std::time::Instant;
#[derive(Args, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct ReleaseCommand {
    #[doc = " Omitir tests (no recomendado)"]
    #[arg(long)]
    pub no_tests: bool,
    #[doc = " Omitir empaquetado (cargo package)"]
    #[arg(long)]
    pub no_package: bool,
    #[doc = " Generar reporte SBOM usando cyclonedx/cargo-deny si están disponibles"]
    #[arg(long)]
    pub sbom: bool,
    #[doc = " Ejecutar limpieza previa (cargo clean)"]
    #[arg(long)]
    pub clean: bool,
    #[doc = " Ejecutar build release completo al final"]
    #[arg(long)]
    pub build: bool,
}
impl ReleaseCommand {
    #[doc = "Method documentation added by AI refactor"]
    pub async fn execute(&self) -> Result<()> {
        let executor = CargoExecutor::new();
        let start = Instant::now();
        let mut steps = Vec::new();
        if self.clean {
            self.run_step(&executor, "Clean workspace", &["clean"], &mut steps)
                .await?;
        } else {
            steps.push(StepSummary::skipped("Clean workspace"));
        }
        self.run_step(
            &executor,
            "Fmt check",
            &["fmt", "--", "--check"],
            &mut steps,
        )
        .await?;
        self.run_step(
            &executor,
            "Clippy -D warnings",
            &["clippy", "--", "-D", "warnings"],
            &mut steps,
        )
        .await?;
        if self.no_tests {
            steps.push(StepSummary::skipped("Tests (cargo test --no-run)"));
        } else {
            self.run_step(
                &executor,
                "Tests (cargo test --no-run)",
                &["test", "--no-run"],
                &mut steps,
            )
            .await?;
        }
        if self.build {
            self.run_step(
                &executor,
                "Build release",
                &["build", "--release"],
                &mut steps,
            )
            .await?;
        } else {
            steps.push(StepSummary::skipped("Build release"));
        }
        if self.no_package {
            steps.push(StepSummary::skipped("cargo package"));
        } else {
            self.run_step(
                &executor,
                "cargo package",
                &["package", "--allow-dirty"],
                &mut steps,
            )
            .await?;
        }
        if self.sbom {
            steps.push(self.run_sbom_step(&executor).await?);
        } else {
            steps.push(StepSummary::skipped("SBOM report"));
        }
        print_step_table("Release Summary", &steps, start.elapsed());
        Ok(())
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_step(
        &self,
        executor: &CargoExecutor,
        label: &str,
        args: &[&str],
        steps: &mut Vec<StepSummary>,
    ) -> Result<()> {
        let step_start = Instant::now();
        match executor.execute_streaming(args).await {
            Ok(()) => {
                steps.push(StepSummary::success(label, step_start.elapsed()));
                Ok(())
            }
            Err(e) => {
                steps.push(StepSummary::failed(
                    label,
                    step_start.elapsed(),
                    e.to_string(),
                ));
                Err(e)
            }
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn run_sbom_step(&self, executor: &CargoExecutor) -> Result<StepSummary> {
        let start = Instant::now();
        if which::which("cyclonedx-cargo").is_ok() {
            executor
                .execute_streaming(&["cyclonedx", "--output", "sbom.json"])
                .await
                .context("cyclonedx-cargo falló")?;
            return Ok(StepSummary::success("SBOM (cyclonedx)", start.elapsed()));
        }
        if which::which("cargo-deny").is_ok() {
            executor
                .execute_streaming(&["deny", "check", "licenses"])
                .await
                .context("cargo-deny falló")?;
            return Ok(StepSummary::success("SBOM (cargo-deny)", start.elapsed()));
        }
        println!(
            "{}",
            "⚠️  No se encontró cyclonedx-cargo ni cargo-deny, omitiendo SBOM".yellow()
        );
        Ok(StepSummary::skipped("SBOM report"))
    }
}
