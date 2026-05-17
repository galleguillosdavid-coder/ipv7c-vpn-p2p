# IPv7C CI/CD Strategy (100% Rust)

> **Version:** 1.0.0-dev | **Date:** 2026-05-17

## Philosophy

IPv7C is a decentralized protocol — its build pipeline must be equally resilient. If GitHub Actions fails, local compilation is always a first-class citizen.

---

## 1. GitHub Actions Workflows

### `ci-core.yml` — On every push/PR
```yaml
# Tests and lints for all Rust crates in the workspace
- cargo fmt --check
- cargo clippy --workspace -- -D warnings
- cargo test --workspace
- cargo bench --workspace --no-run  # Verify benchmarks compile
```

### `ci-build.yml` — On push to main
```yaml
# Cross-platform release builds
matrix:
  - target: x86_64-pc-windows-msvc    # Windows
  - target: x86_64-unknown-linux-musl  # Linux (static)
  - target: aarch64-unknown-linux-musl # Linux ARM64 (routers)
  - target: x86_64-apple-darwin        # macOS Intel
  - target: aarch64-apple-darwin        # macOS Apple Silicon
```

### `ci-nightly.yml` — Cron (weekly)
```yaml
# Expensive targets: Android NDK, iOS, embedded, MIPS
- Android: cargo ndk -t aarch64-linux-android -- build --release
- Embedded: cargo build --target thumbv7em-none-eabihf (no_std check)
```

### Caching Strategy
- `Swatinem/rust-cache` for Cargo registry + target directory
- Separate cache keys per platform and per-crate hash
- Cache invalidation on Cargo.lock changes only

---

## 2. Local Build (The Fail-Safe)

### All platforms
```bash
cargo build --workspace --release
```

### Windows installer (Tauri)
```bash
cd platform/desktop && npm run tauri build
```

### Linux static binary
```bash
cargo build --release --target x86_64-unknown-linux-musl
# Result: target/x86_64-unknown-linux-musl/release/ipv7c-cli
```

### Android .so
```bash
cargo ndk -t aarch64-linux-android -o platform/android/app/src/main/jniLibs -- build --release
cd platform/android && ./gradlew assembleRelease
```

---

## 3. Release Process

1. Update `VERSION` file and all `Cargo.toml` versions
2. `cargo test --workspace` passes
3. Tag: `git tag v1.0.0 && git push --tags`
4. CI builds release binaries for all platforms
5. GitHub Release with attached binaries

**Versioning:** Semantic Versioning. Single source of truth: root `VERSION` file.
