# PatentChain — On-Chain Patent License Manager

> A Soroban smart contract on the Stellar blockchain for registering patents, issuing licenses, and recording royalty payments — fully on-chain, transparent, and tamper-proof.

---

## Project Description

**PatentChain** is a Soroban smart contract that brings intellectual property licensing onto the Stellar blockchain. Patent holders can register their patents, set licensing terms, and issue licenses to third parties — all without intermediaries. Licensees can purchase access rights and record royalty payments directly through the contract, creating an immutable, auditable trail of IP usage.

Traditional patent licensing relies heavily on lawyers, manual invoicing, and trust between parties. PatentChain replaces that friction with smart contract logic: terms are encoded in code, payments are recorded on-chain, and license validity can be queried in real time by any party.

---

## What It Does

| Actor | Action |
|---|---|
| **Patent Owner** | Register a patent with a title, flat license fee, and royalty rate |
| **Patent Owner** | Update terms, deactivate patents, or revoke licenses |
| **Licensee** | Purchase a time-bound license by patent ID |
| **Licensee** | Record usage-based royalty payments against a patent |
| **Anyone** | Query patent details, license validity, or payment history |

The contract operates in two billing modes:

1. **Flat License Fee** — A one-time fee paid at license purchase, valid for a configurable number of ledgers (time period).
2. **Usage-Based Royalties** — Licensees self-report revenue and the contract calculates the royalty due based on a basis-point rate set by the patent owner.

License validity is checked against the current Stellar ledger sequence number, making expiry automatic and objective.

---

## Features

### 🗂 Patent Registry
- Register patents with a human-readable title, license fee (in stroops), and royalty rate (basis points)
- Each patent is assigned a unique auto-incremented ID
- Owners can deactivate patents at any time — existing licenses remain valid, new purchases are blocked
- Owners can update pricing terms at any time

### 📄 License Issuance & Management
- Purchase time-bound licenses with a configurable duration (measured in Stellar ledger sequences)
- Licenses are stored on-chain and linked to both the licensee address and the patent
- Patent owners can revoke any license (e.g. for non-payment or breach of terms)
- Query all licenses held by a wallet address or issued under a specific patent

### 💸 Royalty Tracking
- Licensees submit usage revenue amounts; the contract calculates and records royalties automatically
- Royalty rate is set per-patent in basis points (1 bps = 0.01%), supporting rates from 0.01% to 100%
- Running `total_collected` tallied per patent across both flat fees and royalties

### ✅ On-Chain Validity Checks
- `is_license_valid()` returns a boolean in real time
- Considers both manual revocation and ledger-based expiry
- Any application, wallet, or contract can integrate this check

### 📡 Event Emission
- Key lifecycle actions emit on-chain events: `REGISTERED`, `LICENSED`, `REVOKED`, `ROYALTY`
- Events can be indexed off-chain for dashboards, notifications, or analytics

### 🔐 Authorization
- All state-mutating calls require the caller's signature via `require_auth()`
- Only the patent owner can update terms, deactivate, or revoke
- Licensees authorize their own purchases and royalty reports

---

## Project Structure

```
patent-license/
├── Cargo.toml                              # Workspace manifest
└── contracts/
    └── patent_license/
        ├── Cargo.toml                      # Contract dependencies
        └── src/
            ├── lib.rs                      # Contract logic
            └── test.rs                     # Unit tests
```

---

## Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
stellar contract build
```

The compiled `.wasm` will be at:
```
target/wasm32-unknown-unknown/release/patent_license.wasm
```

### Run Tests

```bash
cargo test
```

### Deploy to Testnet

```bash
# Configure testnet identity
stellar keys generate --global alice --network testnet

# Fund with Friendbot
stellar keys fund alice --network testnet

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/patent_license.wasm \
  --source alice \
  --network testnet
```

### Initialize & Interact

```bash
# Initialize with admin
stellar contract invoke \
  --id <CONTRACT_ID> --source alice --network testnet \
  -- initialize --admin <ALICE_ADDRESS>

# Register a patent (500 bps = 5% royalty, 10 XLM flat fee)
stellar contract invoke \
  --id <CONTRACT_ID> --source alice --network testnet \
  -- register_patent \
     --owner <ALICE_ADDRESS> \
     --title "My Invention" \
     --license_fee 100000000 \
     --royalty_rate_bps 500

# Purchase a license (~1 year ≈ 6048000 ledgers)
stellar contract invoke \
  --id <CONTRACT_ID> --source bob --network testnet \
  -- purchase_license \
     --licensee <BOB_ADDRESS> \
     --patent_id 1 \
     --duration_ledgers 6048000

# Check if license is valid
stellar contract invoke \
  --id <CONTRACT_ID> --network testnet \
  -- is_license_valid --license_id 1
```

---

## Key Constants

| Constant | Value | Meaning |
|---|---|---|
| `1 XLM` | `10,000,000 stroops` | Base currency unit |
| `10,000 bps` | `100%` | Maximum royalty rate |
| `~6,048,000 ledgers` | `~1 year` | Based on ~5s per ledger |
| `~1,051,200 ledgers` | `~60 days` | Common short-term license |

---

## License

MIT

---   

wallet address : GAFPHXX7Y37KO27KFAHPBD5DCO4VZUUIN2S2MXYULVICGZTITMJWSYBE

contact address : CCPSGOIW3TGR6YBUCV4PUPEYTZZVLAGLPW4E4BGCZQWITM3IJYTJDYCT

https://stellar.expert/explorer/testnet/contract/CCPSGOIW3TGR6YBUCV4PUPEYTZZVLAGLPW4E4BGCZQWITM3IJYTJDYCT

<img width="1917" height="1033" alt="{0986FEE4-1BEF-4CC6-8DD8-7E8AE60A7EFD}" src="https://github.com/user-attachments/assets/5cd01ed2-46f4-469e-b5e8-b65103be5f01" />



> Built with [Soroban SDK](https://soroban.stellar.org) · Runs on [Stellar](https://stellar.org)
