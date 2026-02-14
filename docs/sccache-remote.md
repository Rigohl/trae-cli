# sccache remote cache (S3 / Redis) — example

This document shows how to enable **remote sccache** backing (S3 or Redis) for faster, shared CI caching.

## 1) S3-backed sccache (recommended)
- Create an S3 bucket (private).
- Add these repository secrets in GitHub: `SCCACHE_BUCKET`, `SCCACHE_REGION`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`.
- CI (already configured) will read these secrets and sccache will use S3 for storing cache objects.

Notes:
- Objects are keyed by compilation fingerprint; multiple runners can share the cache.
- Ensure your IAM policy allows `s3:GetObject`, `s3:PutObject`, `s3:ListBucket` for the bucket prefix.

## 2) Redis-backed sccache (low-latency alternative)
- Run a Redis instance accessible from CI (host/port + optional password).
- Set `SCCACHE_REDIS` secret to `redis://:[password]@host:port` (or `redis://host:port`).
- CI will use Redis as the sccache backend when `SCCACHE_REDIS` is present.

## 3) Local developer setup
- Install `sccache`:
  - `cargo install sccache` or your OS package manager.
- Enable locally:
  - `export RUSTC_WRAPPER=$(which sccache)` (Linux/macOS)
  - `setx RUSTC_WRAPPER "C:\\Users\\<you>\\.cargo\\bin\\sccache.exe"` (Windows)
- Optional: put `rustc-wrapper = "sccache"` in `.cargo/config.toml`.

## 4) CI / GitHub Actions configuration (we already add this example)
- The CI job sets `RUSTC_WRAPPER=sccache` and supports optional `SCCACHE_BUCKET` / `SCCACHE_REDIS` secrets.
- CI collects `sccache --show-stats` and uploads it as artifact; when run on PRs it posts the stats as a PR comment.

## 5) Troubleshooting
- `sccache --show-stats` to inspect hits/misses.
- If remote fails, sccache falls back to local cache; check network/credentials.

## 6) Validate remote from CI (quick check)
- A validation workflow has been added: `.github/workflows/sccache-validate.yml` (run it from the Actions tab or via `workflow_dispatch`).
- What it does:
  - Installs `sccache` in the runner and prints `sccache --show-stats`.
  - If `SCCACHE_BUCKET` + AWS credentials are present it will run `aws s3api head-bucket` to verify access.
  - If `SCCACHE_REDIS` is set the workflow reports the value is configured (Redis connectivity must be validated separately).

How to use
1. Add the repository secrets (see section above).
2. Open the repository Actions → `sccache-validate` → `Run workflow`.
3. Inspect the job logs — the workflow will fail if S3 credentials cannot access the bucket.

Notes
- This workflow is purely a validation aid and does not change cache contents.
- If validation fails, check IAM permissions, bucket region, and that the secrets are set correctly.
