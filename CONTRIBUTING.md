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
  - Fast CI without external services: we **recommend `cargo-chef` + `actions/cache`** as the default strategy (no S3/Redis required).

    - Why: `cargo-chef` precomputes dependency build steps so CI can cache compiled dependency layers (`target`) and reuse them across runs — fast, reliable, and no secrets needed.
    - CI already includes `cargo-chef` steps and caches `target`/`recipe.json`.

    # Quick local reproduction
    cargo install cargo-chef --locked
    cargo chef prepare --recipe-path recipe.json
    cargo chef cook --recipe-path recipe.json

  - `sccache` is still supported for local incremental builds (optional) but **no remote backend is required** with the `cargo-chef + actions/cache` approach. See `docs/sccache-remote.md` for alternatives.

  - After making changes you can run the validation workflow: `Actions → sccache-validate` (or run `workflow_dispatch`).

  - Write tests for new behavior and document public API changes.
- Keep commits small and focused.

Code style

- Run `cargo fmt`.
- Avoid `unwrap()` in library code; prefer `anyhow::Result`.

Review process

- PR must include tests and pass CI (tests + clippy -D warnings).
