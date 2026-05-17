$ErrorActionPreference = "Stop"

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

& $PythonExe .\vpn_p2p_simple.py --auto-update @args
exit $LASTEXITCODE
