# ipv7c_core

Backend Rust opcional para IPv7C. El nodo Python funciona sin este modulo; si esta
compilado e instalado como `ipv7c_core`, `ipv7c.py` lo detecta automaticamente y usa
las rutas aceleradas disponibles.

## Desarrollo

```powershell
cd rust/ipv7c_core
python -m pip install maturin
maturin develop --release
cargo test
```

La primera fase acelera operaciones puras (XOR diferencial, codec de frames y
ChaCha20-Poly1305). El crate tambien incluye un smoke test de socket UDP con Tokio
para dejar armado el camino hacia un socket loop nativo.
