# IPv7C VPN P2P - Manager Unificado
# Unifica actualizar, lanzar y verificar en un solo script

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

function Show-Menu {
    Clear-Host
    Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║                    IPv7C VPN P2P Manager                      ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "🚀 Opciones:" -ForegroundColor Yellow
    Write-Host "1. Iniciar VPN (interfaz gráfica)"
    Write-Host "2. Iniciar VPN (modo consola)"
    Write-Host "3. Iniciar VPN con auto-update"
    Write-Host "4. Actualizar desde GitHub"
    Write-Host "5. Verificar instalación (checks completos)"
    Write-Host "6. Probar Wintun (preflight)"
    Write-Host "7. Probar detección NAT"
    Write-Host "8. Salir"
    Write-Host ""
    Write-Host "💡 Tips:" -ForegroundColor Green
    Write-Host "- Usa 1-2 para pruebas rápidas"
    Write-Host "- Usa 3 para producción con actualización"
    Write-Host "- Usa 5 para verificar todo funciona"
    Write-Host "- Usa 7 para probar conexión WAN"
    Write-Host ""
}

function Start-VPN {
    param([string]$Mode, [switch]$AutoUpdate)
    
    $PythonExe = Get-IPv7CPython
    $args = @()
    
    if ($AutoUpdate) {
        $args += "--auto-update"
    }
    
    if ($Mode -eq "console") {
        $args += "--headless"
    }
    
    Write-Host "🚀 Iniciando IPv7C VPN..." -ForegroundColor Green
    Write-Host "   Python: $PythonExe"
    Write-Host "   Args: $($args -join ' ')"
    Write-Host ""
    
    & $PythonExe vpn_p2p_simple.py @args
}

function Update-FromGitHub {
    if (-not (Test-Path -LiteralPath ".git")) {
        Write-Error "❌ Este directorio no es un repo Git."
        Write-Host "💡 Clona primero: git clone https://github.com/galleguillosdavid-coder/ipv7c-vpn-p2p.git"
        return
    }

    $PythonExe = Get-IPv7CPython
    
    Write-Host "📦 Actualizando desde GitHub..." -ForegroundColor Yellow
    
    try {
        git pull --ff-only
        if ($LASTEXITCODE -ne 0) { throw "Git pull failed" }
        
        & $PythonExe -m py_compile vpn_p2p_simple.py
        if ($LASTEXITCODE -ne 0) { throw "Compilation failed" }
        
        Write-Host "✅ Actualizado y verificado." -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Error en actualización: $_" -ForegroundColor Red
    }
}

function Run-Checks {
    $PythonExe = Get-IPv7CPython
    
    Write-Host "🔍 Ejecutando verificación completa..." -ForegroundColor Yellow
    
    $checks = @(
        @{Name="Compilación"; Cmd="python -m py_compile vpn_p2p_simple.py"},
        @{Name="Mock (4 nodos)"; Cmd="python vpn_p2p_simple.py --mock --nodes 4"},
        @{Name="Codec"; Cmd="python vpn_p2p_simple.py --codec-test"},
        @{Name="Auto-negociación puerto"; Cmd="python vpn_p2p_simple.py --port-fallback-test"},
        @{Name="Purga logs"; Cmd="python vpn_p2p_simple.py --log-prune-test"},
        @{Name="Selección rutas"; Cmd="python vpn_p2p_simple.py --route-selection-test"},
        @{Name="Puente adaptador"; Cmd="python vpn_p2p_simple.py --adapter-bridge-test --duration 5"},
        @{Name="Config Wintun"; Cmd="python vpn_p2p_simple.py --wintun-config-test"},
        @{Name="Cache rutas"; Cmd="python vpn_p2p_simple.py --route-cache-test --duration 5"},
        @{Name="Bundles rutas"; Cmd="python vpn_p2p_simple.py --route-bundle-test --duration 5"},
        @{Name="Dashboard"; Cmd="python vpn_p2p_simple.py --web-dashboard-test"},
        @{Name="Test real (4 nodos)"; Cmd="python vpn_p2p_simple.py --real-test --nodes 4 --duration 8"},
        @{Name="Test multiproceso"; Cmd="python vpn_p2p_simple.py --process-test --duration 8"},
        @{Name="Test servicio"; Cmd="python vpn_p2p_simple.py --service-test --duration 6"}
    )
    
    $passed = 0
    $total = $checks.Count
    
    foreach ($check in $checks) {
        Write-Host "   🧪 $($check.Name)..." -NoNewline
        
        $startTime = Get-Date
        & $check.Cmd.Split(" ")
        $duration = (Get-Date) - $startTime
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host " ✅ ($([int]$duration.TotalSeconds)s)" -ForegroundColor Green
            $passed++
        } else {
            Write-Host " ❌ ($([int]$duration.TotalSeconds)s)" -ForegroundColor Red
        }
    }
    
    Write-Host ""
    Write-Host "📊 Resultado: $passed/$total tests pasados" -ForegroundColor $(($passed -eq $total) ? 'Green' : 'Yellow')
    
    if ($passed -eq $total) {
        Write-Host "🎉 ¡Todo funciona correctamente!" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Algunos tests fallaron. Revisa los logs." -ForegroundColor Yellow
    }
}

function Test-Wintun {
    $PythonExe = Get-IPv7CPython
    
    Write-Host "🔧 Probando Wintun..." -ForegroundColor Yellow
    Write-Host "   Esto validará el entorno sin crear adaptador."
    Write-Host ""
    
    & $PythonExe vpn_p2p_simple.py --wintun-preflight --wintun-dll .\wintun\wintun.dll --tun-address 10.77.0.1/24 --tun-route 10.77.0.0/24
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Wintun validado correctamente" -ForegroundColor Green
    } else {
        Write-Host "❌ Error en validación Wintun" -ForegroundColor Red
    }
}

function Test-NAT {
    if (Test-Path ".\tests\stun_turn_client.py") {
        Write-Host "🌐 Probando detección NAT..." -ForegroundColor Yellow
        & (Get-IPv7CPython) .\tests\stun_turn_client.py
    } else {
        Write-Host "❌ Módulo STUN/TURN no encontrado" -ForegroundColor Red
        Write-Host "💡 Asegúrate de tener el archivo tests\stun_turn_client.py"
    }
}

# Programa principal
do {
    Show-Menu
    $choice = Read-Host "Selecciona una opción (1-8)"
    
    switch ($choice) {
        "1" { 
            Start-VPN -Mode "gui"
            Read-Host "Presiona Enter para continuar"
        }
        "2" { 
            Start-VPN -Mode "console"
            Read-Host "Presiona Enter para continuar"
        }
        "3" { 
            Start-VPN -Mode "console" -AutoUpdate
            Read-Host "Presiona Enter para continuar"
        }
        "4" { 
            Update-FromGitHub
            Read-Host "Presiona Enter para continuar"
        }
        "5" { 
            Run-Checks
            Read-Host "Presiona Enter para continuar"
        }
        "6" { 
            Test-Wintun
            Read-Host "Presiona Enter para continuar"
        }
        "7" { 
            Test-NAT
            Read-Host "Presiona Enter para continuar"
        }
        "8" { 
            Write-Host "👋 ¡Hasta luego!" -ForegroundColor Green
            exit
        }
        default {
            Write-Host "❌ Opción no válida. Intenta de nuevo." -ForegroundColor Red
            Start-Sleep 2
        }
    }
} while ($true)
