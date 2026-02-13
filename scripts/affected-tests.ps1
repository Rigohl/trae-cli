<#
Run tests only for affected packages (fast-path)
Usage: pwsh ./scripts/affected-tests.ps1 -Staged | -Ci
#>
param(
    [switch]$Staged,
    [switch]$Ci
)

Write-Host "🔎 Determinando archivos cambiados para tests..." -ForegroundColor Cyan
if ($Staged) { $files = git diff --name-only --cached } elseif ($Ci) { $base = $env:GITHUB_BASE_REF; if ([string]::IsNullOrEmpty($base)) { $base = 'main' }; git fetch origin $base --depth=1; $files = git diff --name-only origin/$base...HEAD } else { git fetch origin main --depth=1; $files = git diff --name-only origin/main...HEAD }
$files = $files | Where-Object { $_ -match '\.rs$' -or $_ -match 'Cargo\.toml$' }
if (-not $files) { Write-Host "No relevant files changed for tests."; exit 0 }

# Map to packages using cargo metadata
$meta = cargo metadata --format-version=1 --no-deps | ConvertFrom-Json
$packages = $meta.packages
$affected = @{}
foreach ($f in $files) {
    $full = (Resolve-Path -LiteralPath $f).ProviderPath
    foreach ($p in $packages) {
        $dir = [System.IO.Path]::GetDirectoryName($p.manifest_path)
        if ($full.StartsWith($dir)) { $affected[$p.name] = $true }
    }
}
if ($affected.Keys.Count -eq 0) { Write-Host "No packages mapped from changed files — running no tests."; exit 0 }

Write-Host "📦 Running tests for: $($affected.Keys -join ', ')" -ForegroundColor Green
$ok = $true
foreach ($pkg in $affected.Keys) {
    Write-Host "⚙️  cargo test -p $pkg" -ForegroundColor Cyan
    cargo test -p $pkg
    if ($LASTEXITCODE -ne 0) { $ok = $false }
}
if (-not $ok) { exit 1 } else { exit 0 }
