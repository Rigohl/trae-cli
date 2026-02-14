# Contributing to TRAE-CLI

Guidelines

- Fork the repo and work on feature branches named `feat/<short-desc>` or `fix/<short-desc>`.
- Run `cargo test` and `cargo clippy -- -D warnings` before opening a PR.
- Use fast local checks: `pwsh ./scripts/affected-checks.ps1 -Staged` and `trae code-health` for quick feedback.
- For faster local builds, install `sccache` and enable it as Rust wrapper (recommended):

  - Install: `cargo install sccache` or use your package manager.
  - Enable for your shell: `export RUSTC_WRAPPER=$(which sccache)` (Linux/macOS) or `setx RUSTC_WRAPPER "C:\\Users\\<you>\\.cargo\\bin\\sccache.exe"` (Windows).
  - Or add to workspace via `.cargo/config.toml` (optional):

    ```toml
    [build]
    rustc-wrapper = "sccache"
    ```
  - Remote sccache in CI: set repository secrets `SCCACHE_BUCKET`, `SCCACHE_REGION`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` (or `SCCACHE_REDIS`) and the CI will use S3/Redis-backed caching. CI prints `sccache --show-stats` after each run and uploads the stats artifact for inspection. See `docs/sccache-remote.md` for examples and troubleshooting.

    # Quick: add repository secrets using GitHub CLI
    echo -n "my-bucket" | gh secret set SCCACHE_BUCKET --repo OWNER/REPO
    echo -n "us-east-1" | gh secret set SCCACHE_REGION --repo OWNER/REPO
    echo -n "AKIA..." | gh secret set AWS_ACCESS_KEY_ID --repo OWNER/REPO
    echo -n "...secret..." | gh secret set AWS_SECRET_ACCESS_KEY --repo OWNER/REPO

  - After adding secrets you can run the validation workflow: `Actions → sccache-validate` (or run `workflow_dispatch`). The workflow will verify S3 access when configured.

  - Write tests for new behavior and document public API changes.
- Keep commits small and focused.

Code style

- Run `cargo fmt`.
- Avoid `unwrap()` in library code; prefer `anyhow::Result`.

Review process

- PR must include tests and pass CI (tests + clippy -D warnings).
