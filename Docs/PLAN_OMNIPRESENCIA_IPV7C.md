# Plan de Omnipresencia IPv7C: Despliegue Universal

> **Estado actual:** Ejecutado — Ver sección de progreso al final.

Este documento establece la hoja de ruta estratégica para hacer que la malla soberana de IPv7C funcione en absolutamente cualquier dispositivo, abarcando desde microcontroladores IoT hasta centros de datos de alto rendimiento.

## 1. El Bloqueador Actual y la Solución
Actualmente, IPv7C está fuertemente acoplado a Python. Aunque excelente para prototipado rápido y PC/Notebooks, Python es inviable para Routers (por CPU/RAM), IoT (sin intérprete) y Smartphones (drenaje de batería y dependencias).

**Solución Radical:** **"Oxidación" total del núcleo.** El protocolo completo debe portarse al lenguaje **Rust**. Rust ofrece seguridad de memoria, ausencia de recolector de basura (GC), huella de RAM minúscula (< 5MB) y compilación cruzada nativa a cualquier arquitectura (ARM, MIPS, x86).

---

## 2. Fases de Despliegue Estratégico

### ✅ Fase 1: Scaffolding Monorepo
Creación de la topología de directorios universal: `core/`, `bindings/`, `cli-daemon/`, `mobile/android/`, `mobile/ios/`, `desktop/`, `hardware/`.

### ✅ Fase 2: Núcleo Rust (`ipv7c_core`)
- Proyecto Cargo inicializado en `/core/` con `pyo3` y `sha2` + `tokio`.
- `get_self_integrity_hash_rust()`: hash SHA-256 real leyendo el binario en Rust.
- `init_tokio_runtime()`: scaffold del motor asíncrono para +100k túneles.

### ✅ Fase 3: Enlace FFI Python ↔ Rust
- `maturin` compiló e instaló `ipv7c_core` en el entorno virtual `.venv`.
- `ipv7c.py` prioriza el hash desde Rust con fallback a Python puro.
- Verificado: `f5e52b3ef2c2660fa0a79bc4881a7ceec758bb68d62a1ba734f395a0a4c232c1`

### ✅ Fase 4: IoT Extremo — Hardware Bare Metal (`no_std`)
- `/hardware/esp32_sensor/`: crate `no_std` con perfil `release` de mínimo tamaño.
- Protocolo `SosFrame` de 16 bytes con checksum XOR: compatible con BLE/LoRa/UDP.
- Función `is_sos_compliant()`: aplica la misma restricción MTU de 512B de `ipv7c.py`.

### ✅ Fase 5: Centros de Datos — Daemon para Routers (`ipv7cd`)
- `/cli-daemon/`: binario independiente con `tokio::main` asíncrono.
- Hereda `ipv7c_core` como dependencia directa de Cargo.
- Listo para compilación cruzada a `aarch64-linux-musl` y `armv7-musleabihf`.

### ✅ Fase 6: Conquista Móvil — Scaffolds Nativos
- **Android** (`/mobile/android/IPv7cVpnService.kt`): `VpnService` con foreground service, interfaz TUN `10.7.0.0/24` y modo `intermittent` para conservar batería.
- **iOS** (`/mobile/ios/IPv7cPacketTunnelProvider.swift`): `NEPacketTunnelProvider` con `async/await`, MTU 1400 y DNS soberano `10.7.0.53`.

### ✅ Fase 7: CI/CD Multiplataforma
- `.github/workflows/omnipresencia_ci.yml`:
  - Matriz: Windows / Linux / macOS (Rust core)
  - Cross-compile: ARM64 + ARMv7 para routers OpenWrt
  - Python FFI: test de integridad del hash y del runtime Tokio
  - Hardware: `cargo check` del crate `no_std` para `thumbv7em-none-eabihf`

### ✅ Fase 8: Documentación de Bindings
- `/bindings/README.md`: tabla de estado por plataforma y guías de generación para Python, C, Android JNI, y iOS UniFFI.

---

## 3. Topología de Repositorio (Implementada)

```text
ipv7c/
├── core/                    ✅ Núcleo 100% Rust (SHA256, Tokio FFI)
├── bindings/                ✅ Documentación de puentes FFI
│   └── README.md
├── cli-daemon/              ✅ Demonio para Linux/Routers (Tokio async)
│   └── src/main.rs
├── mobile/
│   ├── android/             ✅ VpnService nativo Kotlin
│   └── ios/                 ✅ NEPacketTunnelProvider Swift
├── desktop/                 🚧 Tauri/Soul Vision (próxima fase)
├── hardware/
│   └── esp32_sensor/        ✅ Firmware no_std para IoT
└── .github/workflows/
    └── omnipresencia_ci.yml ✅ CI/CD matriz multiplataforma

Próximos pasos recomendados:
- desktop/: Migrar Soul Vision a Tauri (Rust + WebView, 5MB vs Electron 150MB)
- Android: Compilar libipv7c_core.so con NDK y conectar via JNI
- iOS: Generar Swift Package con uniffi-bindgen
- core: Implementar el Hole-Punching y enrutamiento P2P en Rust puro
```

## Resumen Ejecutivo
IPv7C es **un motor de red en Rust con interfaces conectadas a cada plataforma**. El núcleo es de código bajo nivel, ultra-rápido y portable. Cada dispositivo usa las APIs de túnel nativas (TUN/VpnService/NetworkExtension) de su ecosistema para inyectar tráfico en la malla soberana.


## 1. El Bloqueador Actual y la Solución
Actualmente, IPv7C "Oxidación" total del núcleo.** El protocolo completo debe portarse al lenguaje **Rust**. Rust ofrece seguridad de memoria, ausencia de recolector de basura (GC), huella de RAM minúscula (< 5MB) y compilación cruzada nativa a cualquier arquitectura (ARM, MIPS, x86).

---

## 2. Fases de Despliegue Estratégico

### Fase 1: Desacoplamiento y Núcleo Agnóstico (Rust Core)
La primera misión es extraer toda la lógica de `ipv7c.py` (criptografía ChaCha20, enrutamiento, hole punching, LTM) hacia una librería pura en Rust (`libipv7c`).
*   **API FFI (Foreign Function Interface):** Se utilizarán herramientas como `uniffi` para generar "puentes" automáticos. Esto permite que el núcleo en Rust sea invocado como si fuera código nativo desde Python, Kotlin (Android), Swift (iOS) o C.

### Fase 2: Conquista Móvil (Smartphones y Tablets)
Los móviles imponen dos retos: gestión de batería y aislamiento del SO (Sandboxing).
*   **Android / iOS:** Se construirán interfaces ligeras (UI) nativas que envuelvan la `libipv7c`.
*   **Integración VPN Nativa:** En lugar de `Wintun`, el nodo se conectará a las APIs oficiales del SO móvil (`VpnService` en Android, `NetworkExtension` en Apple). Esto captura el tráfico del teléfono a nivel sistema sin requerir root.
*   **Hibernación Inteligente:** Se forzará el perfil `--network-type intermittent`. El nodo entrará en sueño profundo cuando la pantalla se apague, usando *Push Notifications* invisibles o "Latidos" UDP de 1 byte para mantener viva la malla sin consumir batería.

### Fase 3: Infraestructura de Borde (Routers, NAS, Netbooks)
Los routers son los guardianes de las redes caseras, ejecutando hardware muy limitado (ej. ARM, MIPS con 64MB RAM).
*   **OpenWrt / pfSense:** Compilación del núcleo Rust como un binario independiente y estático (cero dependencias). 
*   **Daemon (`ipv7cd`):** Se creará un servicio de fondo manejado por `systemd` o `procd`. IPv7C funcionará en modo "Gateway", enrutando automáticamente todo el tráfico de la casa a través de la malla oscura sin que los dispositivos individuales (Smart TVs, consolas) necesiten tener IPv7C instalado.

### Fase 4: IoT Extremo (Microcontroladores y Sensores)
Dispositivos como cámaras, cerraduras inteligentes o sensores remotos (ESP32, RP2040).
*   **Compatibilidad `no_std`:** El núcleo Rust se adaptará para compilar sin la biblioteca estándar del sistema operativo (Bare Metal).
*   **SOS Protocol Permanente:** Operarán bajo el perfil `--network-type sparse` con MTU limitado. En lugar de internet, usarán transportes alternativos (Bluetooth Low Energy, LoRa) transmitiendo datos a un nodo IPv7C "madre" (un celular o router cercano) a través del `--external-bridge`.

### Fase 5: Centros de Datos (Alto Rendimiento)
Para Nodos Soberanos "Super-Relays" operando en servidores troncales.
*   **eBPF / XDP (Express Data Path):** En Linux, en lugar de pasar los paquetes por el espacio de usuario (TUN/TAP), IPv7C inyectará micro-programas directamente en la tarjeta de red (NIC) del servidor. Esto permite inspeccionar, cifrar y enrutar millones de paquetes por segundo en el espacio del kernel con latencia casi nula (microsegundos).
*   **Concurrencia Asíncrona:** Uso de `tokio` (el motor asíncrono de Rust) para sostener +100,000 túneles simultáneos por máquina sin agotar los hilos del procesador.

---

## 3. Topología de Repositorio Futuro

Para sostener esta omnipresencia, el repositorio deberá mutar a un formato "Monorepo":

```text
ipv7c-monorepo/
├── core/            # Núcleo 100% Rust (Rutas, Crypto, Hole-Punching)
├── bindings/        # Wrappers auto-generados para otros lenguajes
├── cli-daemon/      # Ejecutable para Linux/Windows/Mac/Routers
├── mobile/
│   ├── android/     # App Kotlin + Jetpack Compose
│   └── ios/         # App Swift + SwiftUI
├── desktop/         # Electron / Tauri UI (Soul Vision)
└── hardware/        # Firmwares básicos para ESP32/IoT
```

## Resumen Ejecutivo
Para hacer IPv7C omnipresente, el proyecto debe dejar de ser "Un programa de Python" para convertirse en **"Un motor de red en Rust con interfaces conectadas a cada plataforma"**. El núcleo se vuelve de código bajo nivel, ultra-rápido y portable, mientras que cada dispositivo usa las APIs de túnel (TUN/VpnService) nativas de su ecosistema para inyectar tráfico.
