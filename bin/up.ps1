$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath ".git")) {
    Write-Error "Este directorio no es un repo Git. Clona primero: git clone https://github.com/galleguillosdavid-coder/ipv7c-vpn-p2p.git"
    exit 1
}

function Get-IPv7CPython {
    if ($env:IPV7C_PYTHON -and (Test-IPv7CPython $env:IPV7C_PYTHON)) {
        return $env:IPV7C_PYTHON
    }

    $python = Get-Command python -ErrorAction SilentlyContinue
    if ($python -and (Test-IPv7CPython $python.Source)) { return $python.Source }

    $py = Get-Command py -ErrorAction SilentlyContinue
    if ($py -and (Test-IPv7CPython $py.Source)) { return $py.Source }

    $codexPython = Join-Path $env:USERPROFILE ".cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe"
    if ((Test-Path -LiteralPath $codexPython) -and (Test-IPv7CPython $codexPython)) {
        return $codexPython
    }

    Write-Error "No se encontro python ni py en PATH."
    exit 1
}

function Test-IPv7CPython {
    param([string]$CommandPath)
    if (-not $CommandPath) { return $false }
    $resolved = Get-Command $CommandPath -ErrorAction SilentlyContinue
    if (-not $resolved) { return $false }
    & $resolved.Source -c "import sys" *> $null
    return $LASTEXITCODE -eq 0
}

$PythonExe = Get-IPv7CPython

git pull --ff-only
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe -m py_compile ipv7c.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Actualizado y verificado."
Write-Host "Para iniciar con auto-update en cada arranque: .\iniciar_actualizado.ps1"
