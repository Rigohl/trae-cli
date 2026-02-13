<#
Affected-only checks script
- Detecta paquetes afectados por los archivos cambiados (staged o entre ramas)
- Ejecuta `cargo check -p <pkg>` para cada paquete afectado
- Ejecuta `trae code-health` y `scripts/code-health.ps1`
- Opciones: -Staged, -Ci, -IncludeDependents, -RunClippy, -RunTests
#>
param(
    [switch]$Staged,        # use staged files (pre-commit)
    [switch]$Ci,            # run in CI: compute diff against base ref
    [switch]$IncludeDependents = $true,
    [switch]$RunClippy = $false,
    [switch]$RunTests = $false
)

function Get-ChangedFiles {
    param([switch]$Staged, [switch]$Ci)
    if ($Staged) {
        git diff --name-only --cached 2>$null
    } elseif ($Ci) {
        $base = $env:GITHUB_BASE_REF
        if ([string]::IsNullOrEmpty($base)) { $base = 'main' }
        git fetch origin $base --depth=1 2>$null
        git diff --name-only origin/$base...HEAD
    } else {
        git fetch origin main --depth=1 2>$null | Out-Null
        git diff --name-only origin/main...HEAD
    }
}

Write-Host "🔎 Determinando archivos cambiados..." -ForegroundColor Cyan
$changed = Get-ChangedFiles -Staged:$Staged -Ci:$Ci | Where-Object { $_ -ne '' }
if (-not $changed) { Write-Host "No changed files detected."; exit 0 }
$changed | ForEach-Object { Write-Host "  • $_" }

# Filter relevant files
$relevant = $changed | Where-Object { $_ -match '\.rs$' -or $_ -match 'Cargo\.toml$' -or $_ -match 'build\.rs$' }
if (-not $relevant) { Write-Host "No relevant Rust files changed."); exit 0 }

# Load cargo metadata
$metaJson = cargo metadata --format-version=1 --no-deps 2>$null | ConvertFrom-Json
$packages = $metaJson.packages
$manifest_map = @{}
foreach ($p in $packages) {
    $dir = [System.IO.Path]::GetDirectoryName($p.manifest_path)
    $manifest_map[$p.name] = @{ id = $p.id; dir = $dir }
}

# Map changed files to packages
$affected = @{}
foreach ($f in $relevant) {
    $full = (Resolve-Path -LiteralPath $f).ProviderPath
    foreach ($name in $manifest_map.Keys) {
        $dir = $manifest_map[$name].dir
        if ($full.StartsWith($dir)) { $affected[$name] = $true }
    }
}

# If Cargo.toml at workspace root changed, mark all workspace members
if ($changed -match '^Cargo\.toml$') {
    foreach ($p in $packages) { $affected[$p.name] = $true }
}

if ($affected.Keys.Count -eq 0) {
    Write-Host "No packages mapped from changed files — running workspace check as fallback." -ForegroundColor Yellow
    exit 0
}

Write-Host "\n📦 Affected packages: $($affected.Keys -join ', ')" -ForegroundColor Green

# Optionally include dependents via resolve graph
if ($IncludeDependents) {
    $resolve = cargo metadata --format-version=1 2>$null | ConvertFrom-Json
    $nodes = @{}
    foreach ($n in $resolve.resolve.nodes) { $nodes[$n.id] = $n.deps }
    # build reverse map
    $rev = @{}
    foreach ($k in $nodes.Keys) {
        foreach ($d in $nodes[$k]) {
            $pkg = $d.pkg
            if (-not $rev.ContainsKey($pkg)) { $rev[$pkg] = @() }
            $rev[$pkg] += $k
        }
    }
    # BFS to add dependents
    $queue = [System.Collections.Generic.Queue[string]]::new()
    foreach ($name in $affected.Keys) { $queue.Enqueue((Get-Item ($packages | Where-Object { $_.name -eq $name }).id).ToString()) }
    while ($queue.Count -gt 0) {
        $cur = $queue.Dequeue()
        if ($rev.ContainsKey($cur)) {
            foreach ($depId in $rev[$cur]) {
                $pkg = ($packages | Where-Object { $_.id -eq $depId }).name
                if (-not $affected.ContainsKey($pkg)) { $affected[$pkg] = $true; $queue.Enqueue($depId) }
            }
        }
    }
    Write-Host "Included dependent packages: $($affected.Keys -join ', ')" -ForegroundColor Cyan
}

# Run code-health quickly (scans repo, fast)
Write-Host "\n🔎 Running code-health script..." -ForegroundColor Cyan
pwsh -NoProfile -NoLogo -ExecutionPolicy Bypass -File ./scripts/code-health.ps1
if ($LASTEXITCODE -ne 0) { Write-Host "code-health issues detected" -ForegroundColor Red; exit $LASTEXITCODE }

# Run cargo check per-package (no full workspace compile)
$allOk = $true
foreach ($pkg in $affected.Keys) {
    Write-Host "\n⚙️  cargo check -p $pkg" -ForegroundColor Cyan
    cargo check -p $pkg
    if ($LASTEXITCODE -ne 0) { $allOk = $false; Write-Host "cargo check failed for $pkg" -ForegroundColor Red }
    if ($RunClippy) {
        Write-Host "⚙️  cargo clippy -p $pkg" -ForegroundColor Cyan
        cargo clippy -p $pkg -- -D warnings
        if ($LASTEXITCODE -ne 0) { $allOk = $false }
    }
    if ($RunTests) {
        Write-Host "⚙️  cargo test -p $pkg" -ForegroundColor Cyan
        cargo test -p $pkg
        if ($LASTEXITCODE -ne 0) { $allOk = $false }
    }
}

if (-not $allOk) { Write-Host "Some checks failed" -ForegroundColor Red; exit 1 } else { Write-Host "All affected-only checks passed" -ForegroundColor Green; exit 0 }
