# Run bend externally and save JSON output for analysis by trae-cli
param(
    [int]$Runs = 50000,
    [string]$OutDir = ".\.trae\bench"
)

if (-not (Get-Command bend -ErrorAction SilentlyContinue)) {
    Write-Error "'bend' no está en PATH. Instala bend o añade su ruta al PATH antes de ejecutar este script."
    exit 2
}

if (-not (Test-Path $OutDir)) { New-Item -ItemType Directory -Path $OutDir -Force | Out-Null }
$ts = Get-Date -Format "yyyyMMdd_HHmmss"
$outFile = Join-Path $OutDir "bend_$ts.json"

Write-Host "Ejecutando bend --runs $Runs -> $outFile"
bend --runs $Runs --format json > $outFile 2>&1
$exit = $LASTEXITCODE
if ($exit -ne 0) {
    Write-Error "bend terminó con código $exit. Revisa $outFile para ver la salida."
    exit $exit
}
Write-Host "Salida guardada en: $outFile"
Write-Host "Puedes analizar el JSON con 'trae simulate --complex' o herramientas externas."
exit 0
