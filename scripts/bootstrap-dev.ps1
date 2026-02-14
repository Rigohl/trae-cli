<#
Bootstrap developer environment for TRAE-CLI
- Installs: rustfmt, sccache, cargo-edit, cargo-audit, cargo-outdated
- Sets RUSTC_WRAPPER for the session to sccache if available
- Optionally runs `cargo build` if -Build is supplied

Usage:
  pwsh ./scripts/bootstrap-dev.ps1           # install tools only
  pwsh ./scripts/bootstrap-dev.ps1 -Build    # install tools and build the project
#>
param(
    [switch]$Build
)

function Ensure-Command($name, $checkCmd, $installScript) {
    Write-Host "Checking $name..." -ForegroundColor Cyan
    try {
        if (Get-Command $checkCmd -ErrorAction SilentlyContinue) {
            Write-Host "$name already installed" -ForegroundColor Green
            return $true
        }
    }
    catch { }
    Write-Host "$name not found — installing..." -ForegroundColor Yellow
    try {
        iex $installScript
        return $true
    }
    catch {
        Write-Host ("Failed to install {0}: {1}" -f $name, $_) -ForegroundColor Red
        return $false
    }
}

# Ensure rustfmt
Write-Host "\n== Ensuring Rust components and tools ==" -ForegroundColor Cyan
pwsh -NoProfile -Command {
    try {
        rustup component add rustfmt
        Write-Host "rustfmt installed" -ForegroundColor Green
    }
    catch {
        Write-Host "rustfmt install failed or already present" -ForegroundColor Yellow
    }
}

function Install-CrateFallback($crateName) {
    Write-Host "Attempting to install $crateName (primary: --locked)" -ForegroundColor Cyan
    $ok = $false
    try {
        Write-Host "cargo install $crateName --locked" -ForegroundColor DarkGray
        cargo install $crateName --locked
        $ok = $true
    }
    catch {
        Write-Host "cargo install --locked failed for $crateName, trying without --locked" -ForegroundColor Yellow
        try { cargo install $crateName; $ok = $true } catch { $ok = $false }
    }
    if (-not $ok) {
        Write-Host "Fallback: trying cargo install from git for $crateName" -ForegroundColor Yellow
        try { cargo install --git https://github.com/mozilla/sccache.git $crateName; $ok = $true } catch { $ok = $false }
    }
    return $ok
}

# Ensure sccache
if (-not (Get-Command sccache -ErrorAction SilentlyContinue)) {
    Write-Host "sccache not found, attempting installation..." -ForegroundColor Yellow
    if (-not (Install-CrateFallback 'sccache')) {
        Write-Host "Warning: failed to install sccache automatically. Please install it manually (cargo install sccache)" -ForegroundColor Red
    }
}
else { Write-Host "sccache present" -ForegroundColor Green }

# Ensure cargo-edit (cargo add/upgrade)
if (-not (Get-Command cargo-add -ErrorAction SilentlyContinue) -and -not (Get-Command cargo-upgrade -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-edit not found, attempting installation..." -ForegroundColor Yellow
    if (-not (Install-CrateFallback 'cargo-edit')) {
        Write-Host "Warning: failed to install cargo-edit automatically. Please install it manually (cargo install cargo-edit)" -ForegroundColor Red
    }
}
else { Write-Host "cargo-edit present" -ForegroundColor Green }

# Ensure cargo-audit (optional)
if (-not (Get-Command cargo-audit -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-audit not found, attempting installation..." -ForegroundColor Yellow
    if (-not (Install-CrateFallback 'cargo-audit')) {
        Write-Host "Warning: failed to install cargo-audit automatically. You can skip it or install manually." -ForegroundColor Red
    }
}
else { Write-Host "cargo-audit present" -ForegroundColor Green }

# Ensure cargo-outdated (optional)
if (-not (Get-Command cargo-outdated -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-outdated not found, attempting installation..." -ForegroundColor Yellow
    if (-not (Install-CrateFallback 'cargo-outdated')) {
        Write-Host "Warning: failed to install cargo-outdated automatically. You can skip it or install manually." -ForegroundColor Red
    }
}
else { Write-Host "cargo-outdated present" -ForegroundColor Green }

# Ensure cargo-chef (used by CI for no-remote caching strategy)
if (-not (Get-Command cargo-chef -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-chef not found — installing (optional, used to reproduce CI cache locally)..." -ForegroundColor Yellow
    if (-not (Install-CrateFallback 'cargo-chef')) {
        Write-Host "Warning: failed to install cargo-chef automatically. Install manually with 'cargo install cargo-chef --locked'" -ForegroundColor Red
    }
}
else { Write-Host "cargo-chef present" -ForegroundColor Green }

# Set RUSTC_WRAPPER to sccache for the current session if available
if (Get-Command sccache -ErrorAction SilentlyContinue) {
    $sccachePath = (Get-Command sccache).Source
    $env:RUSTC_WRAPPER = $sccachePath
    Write-Host "RUSTC_WRAPPER set to $sccachePath" -ForegroundColor Green
    sccache --show-stats || Write-Host "(sccache stats not available)" -ForegroundColor Yellow
}
else {
    Write-Host "sccache not available, skip setting RUSTC_WRAPPER" -ForegroundColor Yellow
}

# Optionally build
if ($Build) {
    Write-Host "\n== Building project (cargo build) ==" -ForegroundColor Cyan
    cargo build
    if ($LASTEXITCODE -eq 0) { Write-Host "Build succeeded" -ForegroundColor Green } else { Write-Host "Build failed" -ForegroundColor Red; exit $LASTEXITCODE }
}

Write-Host "\nBootstrap complete. Recommended: restart your shell to persist RUSTC_WRAPPER if you want it global." -ForegroundColor Cyan
