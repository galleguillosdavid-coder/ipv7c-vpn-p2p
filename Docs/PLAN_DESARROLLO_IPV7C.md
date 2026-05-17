# Plan de Desarrollo IPv7C — Ingeniería 100% Rust

> **Versión:** 1.0.0-dev | **Fecha:** 2026-05-17

---

## 1. Roles de Desarrollo

- **David (Arquitecto):** Decisiones de diseño macro. Aprueba arquitectura. Valida seguridad.
- **Agentes IA (Orquestadores):** Razonamiento continuo, refactorización, documentación, implementación autónoma.

## 2. Metodología: Razonar → Validar → Ejecutar

1. **Razonar:** Estrategia fragmentada por crate. Nunca escribir más de 300 líneas sin validar.
2. **Validar:** `cargo test` + `cargo clippy` obligatorios antes de commit.
3. **Ejecutar:** Un crate a la vez, de hoja a raíz (crypto → identity → transport → ... → node).

## 3. Leyes Inmutables

1. **PROHIBIDO Python.** Todo el código de producción es Rust.
2. **PROHIBIDO simular (mock).** Toda funcionalidad es empírica y verificable.
3. **PROHIBIDO emular.** Si existe implementación real (sockets, crypto), se usa.
4. **Validación empírica.** Tests con datos reales, benchmarks con Criterion.
5. **Minimalismo funcional.** Crate ≤ 1,500 líneas. Si crece, se fragmenta.
6. **Atomicidad.** Commits atómicos por funcionalidad, nunca regenerar archivos completos.
7. **100% async.** Todo I/O usa tokio. Nunca bloquear.

## 4. Estándares de Código

```rust
// Ejemplo de estándar:
// - Todos los errores usan thiserror
// - Todos los logs usan tracing
// - Todos los tipos públicos implementan Debug
// - Sin unwrap() en producción (solo en tests)
// - Documentación rustdoc en toda API pública
```

- **Formateo:** `cargo fmt` (rustfmt defaults)
- **Linting:** `cargo clippy -- -D warnings` (cero warnings)
- **Tests:** `#[cfg(test)]` en cada módulo, cobertura ≥ 80%
- **Benchmarks:** Criterion para funciones críticas (crypto, codec, routing)

## 5. Flujo de Trabajo Git

```
main ← feature/fase-N-crate-name
```

- Cada fase = una rama feature
- PR con descripción de cambios + resultado de `cargo test --workspace`
- Squash merge a main
- Tag semántico al completar cada fase

## 6. Orden de Implementación (Dependencias)

```text
ipv7c-crypto (Fase 1) [x] ─────┐
                            ├──► ipv7c-identity (Fase 2) [x]
                            ├──► ipv7c-transport (Fase 3) [x] ──► ipv7c-routing (Fase 4) [x]
                            │                              └─► ipv7c-tunnel (Fase 5) [x]
                            ├──► ipv7c-protocol (Fase 7) [x]
                            │
ipv7c-discovery (Fase 6) [x] ◄──┤
ipv7c-governance (Fase 8) [x] ◄─┤
ipv7c-config (Fase 9) [x] ◄─────┘
                            │
ipv7c-node (Fase 10) [x] ◄──────┘
bins/ (Fase 11) [x] ◄── ipv7c-node
Purga Python (Fase 12) [x]
```
