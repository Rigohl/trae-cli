<#
PowerShell helper: parse `cargo check/build --message-format=json` and print clickable file:line:col locations.
Usage:
  ./cargo-diagnostics.ps1          # run `cargo check` and print diagnostics
  ./cargo-diagnostics.ps1 -Open    # also open each primary span in VS Code via `code -g`
  ./cargo-diagnostics.ps1 -Build   # run `cargo build` instead of `cargo check`
  ./cargo-diagnostics.ps1 -Editor 'code-insiders' -Open

Notes:
- Uses ConvertFrom-Json to parse `--message-format=json` output (PowerShell 6+ recommended).
- Requires `code` (VS Code) CLI in PATH to use -Open option.
#>
param(
    [switch]$Open,
    [switch]$Build,
    [string]$Editor = 'code'  # pass e.g. 'code-insiders' if you use that
)

Write-Host "🔎 Ejecutando cargo $([bool]$Build -eq $true ? 'build' : 'check') --message-format=json" -ForegroundColor Cyan
$cmd = if ($Build) { 'build' } else { 'check' }

# Run cargo with JSON messages and capture output lines
$procInfo = @()
try {
    $lines = & cargo $cmd --message-format=json 2>&1
} catch {
    Write-Error "Error launching cargo: $_"
    exit 2
}

$errors = 0
$warnings = 0

foreach ($line in $lines) {
    if (-not $line.Trim()) { continue }
    $obj = $null
    try { $obj = $line | ConvertFrom-Json -ErrorAction Stop } catch { $obj = $null }
    if ($null -ne $obj -and $obj.reason -eq 'compiler-message') {
        $m = $obj.message
        if ($m.level -in @('error','warning')) {
            foreach ($span in $m.spans) {
                if ($span.is_primary -eq $true) {
                    $file = $span.file_name
                    $lineNo = $span.line_start
                    $col = $span.column_start
                    $msgText = ($m.message -replace "\r|\n"," ").Trim()
                    $output = "[$($m.level.ToUpper())] $file`:$lineNo`:$col — $msgText"
                    if ($m.level -eq 'error') { Write-Host $output -ForegroundColor Red; $errors++ }
                    else { Write-Host $output -ForegroundColor Yellow; $warnings++ }
                    if ($Open) {
                        # open in editor at exact position
                        & $Editor -g "${file}:${lineNo}:${col}" 2>$null
                    }
                }
            }
        }
    } else {
        # Print cargo summary / progress lines (optional)
        if ($line -match 'error:|warning:') { Write-Host $line }
    }
}

Write-Host "\n✅ Resumen: $errors errores, $warnings warnings" -ForegroundColor Cyan
if ($errors -gt 0) { exit 1 } else { exit 0 }
