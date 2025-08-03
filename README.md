# 🌀 Solana Development on Android via Termux

## 📌 Overview
This document outlines the process and solutions for **building and running the Solana CLI and Test Validator directly on Android devices** using [Termux](https://termux.dev/). It provides a detailed reference for developers interested in working with Solana locally on ARM64 Android devices **without needing a traditional Linux or desktop setup**.

---

## ⚠️ Disclaimer!!

> This project provides an **unofficial build** of the Solana CLI and Test Validator for **Android via Termux**.  
>
> It is **not maintained, endorsed, or supported** by the official [Solana Labs](https://solana.com) team.  
>
> Use at your own risk — especially for tools that interact with real or testnet funds.

### ✅ Intended for:
- Local development
- Smart contract testing
- Educational and learning purposes

### ❌ Not recommended for:
- Running mainnet validators
- Managing real funds
- Production deployment

> Termux provides a POSIX-like environment on Android, but **differences in OS-level behavior** (filesystem, syscalls, networking, signal handling) may cause subtle or major runtime issues.

---

## 📱 Motivation
Solana development tooling (like `solana` and `solana-test-validator`) is designed for x86_64 Linux/macOS/Windows systems. Android's environment differs due to:

- Different architecture (`aarch64-linux-android`)
- Incomplete or incompatible `libc` bindings (e.g., `sys-info`)
- Filesystem and socket permission limitations
- Dependencies like `QUIC` behaving unexpectedly

---

## ✅ Achievements Summary

### 🔧 1. **Compiled Solana CLI on Android**
- Built the entire CLI toolchain inside Termux (`solana` binary)
- Applied patches to:
  - Fix `remote-wallet` and `sys-info` incompatibilities
  - Replace problematic crates and logic assumptions
  - Work within Termux’s sandboxed environment

### ✅ CLI Commands Confirmed Working

```bash
solana config get
solana address
solana-keygen new
solana-keygen pubkey
solana config set --keypair ...
solana airdrop
solana logs
solana transfer
solana program deploy
```

---

### 🧪 2. **Custom `solana-test-validator` Binary**
- Created a new `main.rs` under `test-validator/`
- Enabled clean CLI interface using `clap`
- Persisted keypairs for reuse
- Controlled ledger lifecycle with `--reset-ledger`
- Exposed validator metadata via stdout

---

## 🧰 `solana-test-validator` Features

- ✅ Custom `--mint-input` and `--mint-output` support
- ✅ File-based tower storage for local consistency
- ✅ Lightweight mode with reduced shred count
- ✅ Graceful shutdown on `Ctrl+C`
- ✅ Default persistent ledger in `~/.solana-test-validator/ledger`

---

## 🔐 Keypair Reusability

```bash
./solana-test-validator   --mint-input ~/.solana-test-validator/ledger/mint.json
```

Or to generate and save a new one:

```bash
./solana-test-validator   --mint-output ~/.solana-test-validator/ledger/mint.json
```

---

## 📁 Directory Structure

```
target/release/
├── solana
└── solana-test-validator

~/.solana-test-validator/
├── ledger/
│   ├── tower
│   └── mint-keypair.json
```

---

## 🚀 Example Commands

```bash
# Start a fresh validator
./solana-test-validator   --reset-ledger   --rpc-port 8899   --mint-output ~/.solana-test-validator/ledger/mint.json

# Use the CLI with the validator
solana config set --url http://127.0.0.1:8899
solana config set --keypair ~/.solana-test-validator/ledger/mint.json
solana airdrop 100
solana balance
```

---

## 🔍 Use Cases Enabled

- Smart contract development and deployment on Android
- Local program testing without cloud infrastructure
- Educational blockchain tutorials from a mobile device
- Embedded validator usage in Android apps

---

## 🧪 Future Work

- `.deb` or `.pkg` packaging for Termux binary distribution
- Automated build + publish via GitHub Actions
- Binary cache/mirror for easier installation
- Mobile-optimized Solana tutorials and tooling

---

## 🙌 Appreciation

Huge thanks to [Solana Labs](https://solana.com) and the open-source community.

Forked from: [github.com/solana-labs/solana](https://github.com/solana-labs/solana)