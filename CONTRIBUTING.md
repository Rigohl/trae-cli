# Contributing to TRAE-CLI

Guidelines

- Fork the repo and work on feature branches named `feat/<short-desc>` or `fix/<short-desc>`.
- Run `cargo test` and `cargo clippy -- -D warnings` before opening a PR.
- Write tests for new behavior and document public API changes.
- Keep commits small and focused.

Code style

- Run `cargo fmt`.
- Avoid `unwrap()` in library code; prefer `anyhow::Result`.

Review process

- PR must include tests and pass CI (tests + clippy -D warnings).
