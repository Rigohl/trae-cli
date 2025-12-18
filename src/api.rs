use anyhow::Result;

/// API-friendly thin wrappers for common TRAE operations.
pub async fn analyze(
    performance: bool,
    security: bool,
    quality: bool,
    no_jarvix: bool,
    profile: Option<String>,
    force_refresh: bool,
    output: Option<String>,
) -> Result<()> {
    crate::commands::analyze::AnalyzeCommand::run_simple(
        performance,
        security,
        quality,
        no_jarvix,
        profile,
        force_refresh,
        output,
    )
    .await
}

pub async fn repair(opts: crate::commands::repair::RepairOptions) -> Result<()> {
    crate::commands::repair::RepairCommand::run_simple(opts).await
}

pub async fn test_cmd(release: bool, coverage: bool, bench: bool, test: Option<String>, package: Option<String>, verbose: bool, no_jarvix: bool) -> Result<()> {
    crate::commands::test::TestCommand::run_simple(release, coverage, bench, test, package, verbose, no_jarvix).await
}

pub async fn cargo_run(command: &str, args: &[String], interactive: bool, verbose: bool, no_jarvix: bool) -> Result<()> {
    crate::commands::cargo::CargoCommand::run_simple(command, args, interactive, verbose, no_jarvix).await
}
