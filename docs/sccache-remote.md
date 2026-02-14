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
