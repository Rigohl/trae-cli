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

pub async fn repair(
    auto: bool,
    clippy: bool,
    fmt: bool,
    deps: bool,
    dry_run: bool,
    no_jarvix: bool,
    level: Option<String>,
    rollback: bool,
    update: bool,
    upgrade: bool,
    git_branch: Option<String>,
    git_commit: Option<String>,
) -> Result<()> {
    crate::commands::repair::RepairCommand::run_simple(
        auto,
        clippy,
        fmt,
        deps,
        dry_run,
        no_jarvix,
        level,
        rollback,
        update,
        upgrade,
        git_branch,
        git_commit,
    )
    .await
}

pub async fn test_cmd(release: bool, coverage: bool, bench: bool, test: Option<String>, package: Option<String>, verbose: bool, no_jarvix: bool) -> Result<()> {
    crate::commands::test::TestCommand::run_simple(release, coverage, bench, test, package, verbose, no_jarvix).await
}

pub async fn cargo_run(command: &str, args: &[String], interactive: bool, verbose: bool, no_jarvix: bool) -> Result<()> {
    crate::commands::cargo::CargoCommand::run_simple(command, args, interactive, verbose, no_jarvix).await
}
