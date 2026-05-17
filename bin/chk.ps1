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

& $PythonExe -m py_compile ipv7c.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --mock --nodes 4
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --codec-test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --port-fallback-test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --log-prune-test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --route-selection-test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --adapter-bridge-test --duration 5
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --wintun-config-test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --route-cache-test --duration 5
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --route-bundle-test --duration 5
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --web-dashboard-test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --real-test --nodes 4 --duration 8
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --process-test --duration 8
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

& $PythonExe ipv7c.py --service-test --duration 6
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
