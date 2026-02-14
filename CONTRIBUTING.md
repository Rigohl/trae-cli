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

- Write tests for new behavior and document public API changes.
- Keep commits small and focused.

Code style

- Run `cargo fmt`.
- Avoid `unwrap()` in library code; prefer `anyhow::Result`.

Review process

- PR must include tests and pass CI (tests + clippy -D warnings).
