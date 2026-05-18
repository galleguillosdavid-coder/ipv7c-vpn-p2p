# IPv7C — Sovereign P2P VPN · 100% Rust

> **A zero-config, post-quantum, self-governing mesh VPN written entirely in Rust.**  
> Decentralized. Device-agnostic. Multi-profile. Auto-discovered. Unstoppable.

![Version](https://img.shields.io/badge/version-1.0.0--dev-blue.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)
![Rust](https://img.shields.io/badge/language-100%25_Rust-orange.svg)
![PQC](https://img.shields.io/badge/crypto-Post--Quantum-purple.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

---

## What is IPv7C?

IPv7C is a **sovereign mesh VPN** that creates encrypted peer-to-peer tunnels between any devices — without servers, without accounts, without configuration.

- **Zero-Config**: Run `ipv7c up` and you're connected. No setup wizards, no cloud accounts.
- **Post-Quantum**: Hybrid X25519 + ML-KEM-768 (Kyber) handshakes protect against quantum threats today.
- **Self-Discovering**: Finds peers automatically via mDNS on LAN and Kademlia DHT on WAN.
- **Self-Governing**: Decentralized trust scoring and consensus — no central authority decides who participates.
- **Multi-Profile**: Switch between `work`, `home`, `gaming`, or `anonymous` network profiles instantly.
- **Device-Agnostic**: Runs on Windows, Linux, macOS, Android, iOS, routers, and embedded IoT.
- **100% Rust**: Memory-safe, zero-GC, single static binary. No Python, no Node.js, no runtime dependencies.

---

## ⚡ Quick Start

### Pre-built binaries (Windows)

```powershell
# Clone the repo — binaries are included
git clone https://github.com/galleguillosdavid-coder/ipv7c-vpn-p2p.git
cd ipv7c-vpn-p2p

# Start the daemon (background network node)
Start-Process .\target\release\ipv7c-daemon.exe

# Launch the Soul Vision UI dashboard
.\target\release\ipv7c-soul-vision.exe

# Or use the CLI directly
.\target\release\ipv7c.exe up
.\target\release\ipv7c.exe peers
.\target\release\ipv7c.exe profile use gaming
.\target\release\ipv7c.exe identity show
```

### Build from source

```bash
git clone https://github.com/galleguillosdavid-coder/ipv7c-vpn-p2p.git
cd ipv7c-vpn-p2p
cargo build --workspace --release

# Run the node
./target/release/ipv7c up
```

---

## 🏗️ Architecture

IPv7C is a modular Cargo workspace. Each crate has a single responsibility and can be compiled independently.

```text
ipv7c/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── ipv7c-crypto/             # ChaCha20-Poly1305 + X25519 + ML-KEM (PQC) + Noise
│   ├── ipv7c-identity/           # DID keypairs, multi-profile wallet, alias generation
│   ├── ipv7c-transport/          # UDP/TCP/QUIC sockets, hole-punching, multiplexing
│   ├── ipv7c-routing/            # Dynamic route table, gossip protocol, multi-hop relay
│   ├── ipv7c-tunnel/             # TUN adapter (wintun/tun2), IP packet capture
│   ├── ipv7c-discovery/          # mDNS, Kademlia DHT, PEX, bootstrap nodes
│   ├── ipv7c-protocol/           # Binary wire format, frame types, handshake FSM
│   ├── ipv7c-governance/         # Trust scoring, reputation, distributed ban list
│   ├── ipv7c-config/             # Zero-config defaults, TOML overrides, platform detection
│   └── ipv7c-node/               # Node orchestrator, lifecycle state machine, event bus
├── bins/
│   ├── ipv7c-cli/                # User-facing CLI (clap): up, peers, profile, identity
│   └── ipv7c-daemon/             # Background service (systemd / Windows Service)
├── desktop/                      # Tauri GUI — Soul Vision dashboard
│   ├── src/                      # Frontend (HTML/JS/CSS)
│   └── src-tauri/                # Rust backend for Tauri
├── hardware/
│   └── esp32_sensor/             # Embedded IoT firmware
├── mobile/                       # Android/iOS (planned)
└── Docs/                         # Architecture, operations, and development guides
```

---

## 🔐 Cryptographic Design

IPv7C uses a **hybrid post-quantum handshake** to establish sessions:

```
┌──────────┐                          ┌──────────┐
│  Node A  │                          │  Node B  │
└────┬─────┘                          └────┬─────┘
     │  INIT: ephemeral X25519 pubkey       │
     │  + ML-KEM-768 encapsulation key      │
     ├─────────────────────────────────────►│
     │                                      │
     │  CHALLENGE: X25519 pubkey            │
     │  + ML-KEM ciphertext                 │
     │  + signed challenge                  │
     │◄─────────────────────────────────────┤
     │                                      │
     │  CONFIRM: HKDF-derived session key   │
     │  + encrypted confirmation            │
     ├─────────────────────────────────────►│
     │                                      │
     │  ══════ ChaCha20-Poly1305 tunnel ══  │
```

- **Key Exchange**: X25519 (classical) + ML-KEM-768 (post-quantum, NIST FIPS 203)
- **Session Encryption**: ChaCha20-Poly1305 AEAD with automatic key rotation
- **Identity**: Ed25519 keypairs → DID (`did:ipv7c:<base58>`)
- **Framework**: Noise Protocol (IK pattern) for forward secrecy

---

## 🌐 Network Features

### Auto-Discovery (Zero Config)
| Method | Scope | Protocol |
|---|---|---|
| mDNS / DNS-SD | LAN | Multicast `_ipv7c._udp.local` |
| Kademlia DHT | WAN | Distributed hash table |
| PEX (Peer Exchange) | Mesh | Peers share their peer lists |
| Bootstrap Nodes | Initial | Hardcoded seed nodes |
| BLE Beacon | Proximity | Bluetooth Low Energy (mobile) |

### Multi-Profile
```bash
ipv7c profile create work --exit-node=office-server
ipv7c profile create gaming --low-latency --direct-only
ipv7c profile create anon --max-hops=3 --no-direct

ipv7c profile use work     # Switch instantly
ipv7c profile list          # Show all profiles
```

Each profile has its own keypair, routing preferences, and trust settings.

### Self-Governance
- **Trust Scoring**: Every peer earns reputation based on uptime, relay quality, and latency
- **Penalty System**: Misbehaving nodes (spam, drops, manipulation) lose trust automatically
- **Distributed Ban List**: Network consensus to eject confirmed malicious actors
- **No Central Authority**: All governance decisions are emergent from peer interactions

---

## 📦 Platform Support

| Platform | Binary | Transport | Status |
|---|---|---|---|
| Windows 10/11 | `ipv7c.exe` / `ipv7c-soul-vision.exe` | Wintun TUN | ✅ Built |
| Linux (x86_64, ARM) | `ipv7c` | `/dev/net/tun` | 🔨 Building |
| macOS (Apple Silicon) | `ipv7c` | `utun` | 🔨 Building |
| Android 8+ | `.apk` (JNI) | `VpnService` | 📋 Planned |
| iOS 15+ | `.ipa` (UniFFI) | `NetworkExtension` | 📋 Planned |
| OpenWrt / Routers | `ipv7c-daemon` | Raw UDP | 📋 Planned |
| ESP32 / RP2040 | Firmware | BLE + UDP relay | 📋 Planned |

---

## 🛠️ Development

### Prerequisites
- Rust 1.80+ (`rustup install stable`)
- On Windows: Wintun driver (included in `platform/desktop/`)

### Build & Test
```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace

# Check formatting & lints
cargo fmt --check && cargo clippy --workspace -- -D warnings
```

### Project Principles

1. **ZERO external runtime dependencies.** Single static binary.
2. **ZERO mocks in production code.** All validation is empirical.
3. **ZERO central points of failure.** No mandatory servers or clouds.
4. **100% async I/O.** Everything uses `tokio` — never block the main thread.
5. **Extreme modularity.** Each crate ≤ 1,500 lines. If it grows, it splits.
6. **Tests first.** Every crate has `#[cfg(test)]` with ≥ 80% coverage.
7. **Cross-platform from day 1.** Every crate compiles on Windows, Linux, and macOS.

---

## 📄 Documentation

| Document | Description |
|---|---|
| [Architecture Plan](Docs/ARCHITECTURE.md) | Crate responsibilities, data flow, and design decisions |
| [Operations Manual](Docs/MANUAL_DE_OPERACIONES.md) | Deployment, monitoring, and incident response |
| [Development Guide](Docs/PLAN_DESARROLLO_IPV7C.md) | Multi-agent workflow and engineering standards |
| [CI/CD Strategy](Docs/CI_CD_STRATEGY.md) | Build pipeline and local packaging |

---

## 📜 License

MIT — Free as in freedom.

---

**IPv7C v1.0.0-dev** — The VPN that needs no server, no account, no configuration, and no trust.
