<#
Format only staged Rust files and re-add them to the index.
Falls back to cargo fmt if rustfmt not present.
#>
$staged = git diff --name-only --cached --diff-filter=ACM
$rs = $staged | Where-Object { $_ -match '\.rs$' }
if (-not $rs) { Write-Host "No staged Rust files to format."; exit 0 }

$fmtFound = (Get-Command rustfmt -ErrorAction SilentlyContinue) -ne $null
foreach ($f in $rs) {
    if ($fmtFound) {
        Write-Host "Formatting (rustfmt): $f" -ForegroundColor Cyan
        rustfmt --edition 2021 $f
    }
    else {
        Write-Host "Formatting (cargo fmt fallback)" -ForegroundColor Yellow
        cargo fmt -- $f
    }
    git add $f
}
Write-Host "Formatted and re-staged files." -ForegroundColor Green
exit 0
