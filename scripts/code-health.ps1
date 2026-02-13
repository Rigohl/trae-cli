<#
Code health scanner (PowerShell)
- Scanea archivos .rs en src/ y tests/
- Detecta: unwrap\(, expect\(, panic!, unsafe, TODO, unwrap_or_else
- Salida: `[ISSUE] file:line:col - message` (compatible con problemMatcher)
- Exit code 1 si se encuentran issues
#>
param(
    [switch]$Json,
    [switch]$Open
)

Write-Host "🔎 Ejecutando code-health scan..." -ForegroundColor Cyan
$patterns = @(
    @{name='unwrap'; regex='\bunwrap\s*\('; severity='warning'},
    @{name='expect'; regex='\bexpect\s*\('; severity='warning'},
    @{name='panic'; regex='\bpanic!\s*\('; severity='warning'},
    @{name='unsafe'; regex='\bunsafe\b'; severity='critical'},
    @{name='TODO'; regex='\bTODO\b'; severity='info'},
    @{name='unwrap_or_else'; regex='\bunwrap_or_else\s*\('; severity='warning'}
)

$files = Get-ChildItem -Recurse -Path . -Include *.rs -File | Where-Object { $_.FullName -notmatch "(^|/)target/" }
$issues = @()
foreach ($file in $files) {
    $lines = Get-Content -Raw -Path $file.FullName -ErrorAction SilentlyContinue -Encoding UTF8
    if (-not $lines) { continue }
    $idx = 0
    $lines -split "`n" | ForEach-Object {
        $idx++
        $line = $_
        foreach ($p in $patterns) {
            if ($line -match $p.regex) {
                $col = ([regex]::Match($line, $p.regex)).Index + 1
                $item = [pscustomobject]@{
                    file = $file.FullName
                    line = $idx
                    column = $col
                    pattern = $p.name
                    severity = $p.severity
                    text = $line.Trim()
                }
                $issues += $item
                Write-Host "[ISSUE] $($file.FullName):$idx:$col - $($p.name): $($line.Trim())" -ForegroundColor Yellow
                if ($Open) { & code -g "$($file.FullName):$idx:$col" 2>$null }
            }
        }
    }
}

if ($Json) {
    $issues | ConvertTo-Json -Depth 4 | Write-Output
}

if ($issues.Count -gt 0) { exit 1 } else { Write-Host "✅ No code-health issues found" -ForegroundColor Green; exit 0 }
