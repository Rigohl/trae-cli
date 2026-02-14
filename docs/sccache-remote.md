# sccache remote cache (S3 / Redis) — example

This document shows how to enable **remote sccache** backing (S3 or Redis) for faster, shared CI caching.

## 1) Redis-backed sccache (recommended — low-latency)
- Use a managed Redis (Elasticache, Azure Cache) or a hosted Redis reachable from CI.
- Set the repository secret `SCCACHE_REDIS` to `redis://[:password@]host:port` (example: `redis://:hunter2@cache.example.internal:6379`).
- Benefits: low-latency shared cache across runners, simpler object lifecycle and faster hit rates for typical CI runs.

Notes:
- Ensure the Redis instance is accessible from the GitHub Actions runner network (VPC peering / public endpoint + firewall rules as needed).
- If you provide credentials in the URL, keep them secret (use GitHub Secrets).

## 2) S3-backed sccache (alternate — durable object store)
- Create an S3 bucket (private) and set these repository secrets: `SCCACHE_BUCKET`, `SCCACHE_REGION`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`.
- Use S3 when you prefer durable storage of cache objects or need long-term retention.

Notes:
- Objects are keyed by compilation fingerprint; multiple runners can share the cache.
- Ensure your IAM policy allows `s3:GetObject`, `s3:PutObject`, `s3:ListBucket` for the bucket prefix.

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

## 6) No-remote strategy (recommended alternative)
If you do **not** want S3 or any remote service, use `cargo-chef` together with `actions/cache` to get fast, repeatable CI builds without external backends.

Why this helps
- No external secrets or services required — all caching uses GitHub Actions cache and the repo's `target`/dependency layers.
- `cargo-chef` extracts the dependency build plan so CI can cache and reuse compiled dependencies reliably.

How to enable (CI already configured)
1. CI now includes `cargo-chef` steps: `cargo chef prepare` + `cargo chef cook` and caches `target`/`recipe.json`.
2. No repository secrets are needed — just push your PR and CI will reuse cached dependency builds.
3. Locally you can reproduce the cached layer with:

```bash
cargo chef prepare --recipe-path recipe.json
cargo chef cook --recipe-path recipe.json
```

When to still use sccache
- `sccache` remains useful locally for repeated incremental compiles, but remote sccache (S3/Redis) is not required with the `cargo-chef + actions/cache` strategy.

---

## 7) Validate remote from CI (quick check)
- A validation workflow has been added: `.github/workflows/sccache-validate.yml` (run it from the Actions tab or via `workflow_dispatch`).
- What it does:
  - Installs `sccache` in the runner and prints `sccache --show-stats`.
  - If `SCCACHE_BUCKET` + AWS credentials are present it will run `aws s3api head-bucket` to verify access.
  - If `SCCACHE_REDIS` is set the workflow runs a TCP connectivity check to validate Redis reachability.

How to use
1. Add the repository secrets (see section above) only if you plan to use remote backends.
2. Open the repository Actions → `sccache-validate` → `Run workflow`.
3. Inspect the job logs — the workflow will fail if S3 credentials cannot access the bucket or the Redis TCP check fails.

Notes
- This workflow is purely a validation aid and does not change cache contents.
- If validation fails, check IAM permissions, bucket region, and network/firewall rules.
