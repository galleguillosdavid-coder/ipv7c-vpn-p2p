# Manual de Operaciones: Red Soberana IPv7C (100% Rust)

> **Versión:** 1.0.0-dev | **Última actualización:** 2026-05-17

---

## 1. Arquitectura de Despliegue

La red IPv7C opera como una malla P2P soberana donde cada nodo es autónomo.

- **Nodo completo:** Binario único `ipv7c-cli` (Rust, ~5 MB)
- **Daemon de fondo:** `ipv7c-daemon` (systemd / Windows Service)
- **Motor criptográfico:** Post-Quantum híbrido (X25519 + ML-KEM-768)
- **Interfaz de gestión:** Soul Vision (Tauri) o CLI

## 2. Inicialización del Nodo

### Arranque rápido (Zero-Config)
```bash
ipv7c up
```
Esto automáticamente:
1. Genera identidad (Ed25519 keypair + DID) si no existe
2. Detecta tipo de red (LAN/NAT/CGNAT)
3. Inicia descubrimiento mDNS + DHT
4. Establece túneles con peers descubiertos
5. Crea interfaz TUN virtual

### Arranque como servicio
```bash
# Linux
sudo systemctl enable ipv7c-daemon
sudo systemctl start ipv7c-daemon

# Windows (PowerShell como admin)
ipv7c-daemon install
ipv7c-daemon start
```

### Estados del Nodo
| Estado | Descripción |
|---|---|
| `INIT` | Cargando configuración e identidad |
| `DISCOVERING` | Buscando peers (mDNS, DHT, bootstrap) |
| `CONNECTING` | Estableciendo handshakes con peers |
| `MESHED` | Conectado y participando en la malla |
| `DEGRADED` | Conectado pero con peers inestables |
| `ISOLATED` | Sin peers alcanzables |
| `SHUTDOWN` | Cierre ordenado en progreso |

## 3. Gestión de Perfiles

```bash
ipv7c profile list                    # Ver perfiles
ipv7c profile create work             # Crear perfil
ipv7c profile use gaming              # Cambiar perfil activo
ipv7c profile delete old-profile      # Eliminar perfil
```

Cada perfil mantiene: keypair propio, preferencias de routing, trust settings, y alias.

## 4. Monitoreo

### CLI
```bash
ipv7c status                          # Estado general del nodo
ipv7c peers                           # Lista de peers conectados
ipv7c routes                          # Tabla de rutas activas
ipv7c metrics                         # Métricas de rendimiento
```

### API REST local (si habilitada)
- `GET /api/status` — Estado del nodo
- `GET /api/peers` — Peers conectados con métricas
- `GET /api/routes` — Tabla de rutas
- `GET /api/metrics` — Métricas Prometheus

### Indicadores Críticos
1. **RTT por peer**: Latencia estable indica salud. Degradación sostenida dispara re-routing automático.
2. **Trust Score**: Score < 0.3 indica peer poco confiable; el nodo lo depriorizará automáticamente.
3. **Packet Loss**: > 5% pérdida dispara búsqueda de ruta alternativa.
4. **Key Rotation**: Las claves de sesión rotan cada 10 min o 1 GB. Fallas en rotación generan reconexión.

## 5. Logs y Diagnóstico

```bash
# Ver logs en tiempo real
ipv7c logs --follow

# Logs con nivel específico
ipv7c logs --level debug --follow

# Exportar diagnóstico
ipv7c diag export > diagnostico.json
```

- Los logs usan `tracing` con formato estructurado.
- Rotación automática: máximo 50 MB por archivo, 5 archivos históricos.
- Ubicación: `~/.ipv7c/logs/`

## 6. Respuesta a Incidentes

### 6.1 Falla de Hole-Punching UDP
- **Síntoma**: Peers visibles en DHT pero sin túnel directo.
- **Acción automática**: El nodo intenta TCP fallback, luego relay via peer intermediario.
- **Acción manual**: `ipv7c peer <did> --force-relay`

### 6.2 NAT Simétrico detectado
- **Síntoma**: `ipv7c status` muestra `nat_type: symmetric`.
- **Acción automática**: Transporte cambia a TCP o QUIC.
- **Acción manual**: Configurar port forwarding en router.

### 6.3 Peer malicioso detectado
- **Síntoma**: Trust score de un peer cae a 0.
- **Acción automática**: Ban local + propuesta de ban distribuido.
- **Acción manual**: `ipv7c peer <did> --ban`

### 6.4 Pérdida de identidad
- **Síntoma**: Archivos en `~/.ipv7c/wallet/` corruptos o eliminados.
- **Acción**: Restaurar desde backup `ipv7c identity import backup.ipv7c`
- **Sin backup**: `ipv7c identity generate` — crea nueva identidad (pierde reputación previa).

## 7. Configuración Avanzada (Opcional)

Archivo: `~/.ipv7c/config.toml`

```toml
[node]
listen_port = 57341
max_peers = 64
log_level = "info"

[tunnel]
mtu = 1400
dns = "10.7.0.53"
split_tunnel = false

[crypto]
key_rotation_interval_secs = 600
pqc_enabled = true

[discovery]
mdns_enabled = true
dht_enabled = true
bootstrap_nodes = [
    "seed1.ipv7c.net:57341",
    "seed2.ipv7c.net:57341",
]

[profile.default]
routing_preference = "lowest_latency"
max_relay_hops = 3
```

## 8. Gobernanza Operativa
- Toda modificación a procesos de despliegue se documenta en este archivo.
- Las decisiones de red se toman por consenso distribuido, no por autoridad central.
- La seguridad se valida empíricamente antes de despliegue (benchmarks + integration tests).
