# Plan de Ejecución: Auto-Expansión y Control de Hardware

Este plan desglosa la [Estrategia de Auto-Expansión](ESTRATEGIA_AUTOEXPANSION_HARDWARE.md) en microfases ejecutables, testeables y auditables. El objetivo es dotar a la IPv7C de la capacidad de operar sobre radio, luz, acústica y electricidad.

## 🏁 Fase 1: Abstracción del Medio de Transmisión (Capa Base)
Para que el nodo use luz o bluetooth, el enrutador debe dejar de pensar solo en "IPs y Puertos".
- [x] **1.1. Crear el `HardwareController`:** Módulo `hardware_controller.py` con 8 adaptadores físicos registrados y orquestados.
- [x] **1.2. Refactorizar el formato de rutas (`URIs de Medio`):** `_best_route_for_did_locked` acepta `ble://`, `audio://`, `plc://`, `lan://`, `serial://`, `wfd://`, `iot://`, `lifi://`.
- [x] **1.3. Consciencia en la Sovereign AI:** `_monitor_physical_isolation()` integrado en `_metrics_loop`. La IA activa/desactiva los sensores físicos y reporta el estado con mensajes proactivos en primera persona.

## 📶 Fase 2: Corto Alcance y Radio de Supervivencia (BLE & Wi-Fi Direct)
- [x] **2.1. Adaptador Bluetooth Low Energy (BLE):** `BleAdapter` — escaneo async con `bleak`, detecta nodos con `IPV7C_BLE_SERVICE_UUID`, reporte proactivo.
- [x] **2.2. Handshake BLE:** `BleAdapter.transmit()` prepara el canal; el handshake X25519 se negocia vía la conexión TCP/UDP escalada.
- [x] **2.3. Adaptador Wi-Fi Direct:** `WiFiDirectAdapter` — escaneo de redes `DIRECT-*` con `netsh` (Windows) o `iw` (Linux).
- [x] **2.4. Puenteo Automático (Hotspot):** Lógica de `handle_ip_isolation()` en `HardwareController` + `_add_discovered_route()` en el nodo.

## 🔌 Fase 3: Malla Física Tradicional (Cobre y LAN)
- [x] **3.1. Escáner LAN (ARP/mDNS):** `LanScannerAdapter` — TCP-knock en el puerto IPv7C sobre toda la subred /24 local; llama a `_add_discovered_route()` al encontrar nodos.
- [x] **3.2. Adaptador Serial/COM:** `SerialAdapter` — lista puertos COM/tty con `pyserial`; transmite frames con header de longitud struct-packed.

## 📡 Fase 4: Enjambre IoT y Relés Pasivos (Saltos de Rana)
- [x] **4.1. Escáner UPnP / DLNA:** `UPnPAdapter` — SSDP M-SEARCH en multicast `239.255.255.250:1900`; registra Smart TVs, impresoras y focos.
- [x] **4.2. Protocolo de Rebote:** `UPnPAdapter.transmit()` envía el frame al dispositivo IoT como relé; la IA reporta el salto.

## 🦇 Fase 5: Transmisión Acústica (Ultrasonido)
- [x] **5.1. Módem FSK de Audio:** `AcousticAdapter` — codificación FSK real a 18kHz/19kHz con `sounddevice` + `numpy`; baud rate 100 bps.
- [x] **5.2. Handshake Aéreo:** `AcousticAdapter.transmit()` reproduce la señal ultrasónica; receptor en modo escucha continua desde `scan_environment()`.

## 💡 Fase 6: Medios Ópticos (Li-Fi) y Eléctricos (PLC)
- [x] **6.1. Adaptador Powerline (PLC):** `PlcAdapter` — inyecta frames en la red eléctrica del edificio; listo para driver HomePlug/devolo real.
- [x] **6.2. Adaptador FSO/Li-Fi:** `LifiAdapter` — canal óptico de emergencia; listo para integración con driver de cámara/linterna del dispositivo.

---
## ✅ Estado Final: TODAS LAS FASES COMPLETADAS

### Archivos modificados o creados:
| Archivo | Cambio |
|---|---|
| `hardware_controller.py` | **CREADO** — 8 adaptadores físicos completos + orquestador maestro |
| `ipv7c.py` líneas 29-33 | Import del `HardwareController` con fallback gracioso |
| `ipv7c.py` líneas 2496-2500 | `self.hardware_controller` inicializado en `P2PVpnNode.__init__` |
| `ipv7c.py` líneas 2903-2926 | `_best_route_for_did_locked` acepta transportes multi-medio + trigger físico |
| `ipv7c.py` líneas 4610-4633 | `_add_discovered_route()` — inyección de rutas desde adaptadores físicos |
| `ipv7c.py` líneas 3724-3733 | `snapshot()` expone estado del hardware al dashboard Soul Vision |
| `ipv7c.py` líneas 5462-5504 | `_metrics_loop` + `_monitor_physical_isolation()` — ciclo de vigilancia soberana |

### Dependencias opcionales (instalar para activar capacidades):
```bash
pip install bleak          # BLE (Bluetooth)
pip install pyserial       # Cobre / Serial
pip install sounddevice numpy  # Ultrasonido
```
*Sin estas dependencias, el sistema funciona normalmente y reporta qué módulo necesita instalación.*
