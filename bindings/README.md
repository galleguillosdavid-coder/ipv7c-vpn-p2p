# IPv7C Bindings - Puentes FFI para cada Plataforma

Este directorio contiene los bindings (puentes) que conectan el núcleo
nativo `ipv7c_core` (Rust) con cada ecosistema de plataforma.

## Estado por Plataforma

| Plataforma | Binding | Estado |
|---|---|---|
| Python | `pyo3` via `maturin` | ✅ Operativo |
| Android (Kotlin/JVM) | `jni-rs` + JNI | 🚧 Pendiente |
| iOS / macOS (Swift) | `uniffi` + Swift Package | 🚧 Pendiente |
| C / OpenWrt | `cbindgen` (header .h) | 🚧 Pendiente |

## Guía de Generación

### Python (ya operativo)
```bash
cd core && maturin develop
python -c "import ipv7c_core; print(ipv7c_core.get_self_integrity_hash_rust())"
```

### C Header (para integración con firmware de Routers)
```bash
cd core
cargo install cbindgen
cbindgen --config cbindgen.toml --crate ipv7c_core --output ../bindings/ipv7c_core.h
```

### Android JNI
El núcleo debe compilarse como `dylib` con target `aarch64-linux-android`
usando el NDK de Android. El archivo `.so` resultante se incluye en `jniLibs/`.

### iOS / Swift Package
Usar `uniffi-bindgen` para generar automáticamente las clases Swift a partir
de los archivos `ipv7c_core.udl` que describen la interfaz pública del núcleo.
