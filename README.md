# meno-contracts

Soroban smart contracts for the Meno marketplace on Stellar.

## Contract: Marketplace

Handles on-chain listing and purchase of digital assets.

**Functions:**
- `list_asset(seller, asset_id, price)` — creates a persistent listing, requires seller auth
- `buy_asset(buyer, asset_id)` — deactivates listing, requires buyer auth
- `cancel_listing(seller, asset_id)` — cancels own listing, requires seller auth
- `get_listing(asset_id)` — returns listing state (seller, price, active)
- `listing_count()` — total listings created

## Setup

**Requirements:** Rust + `wasm32-unknown-unknown` target + Stellar CLI

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --version "22.0.1"
```

**Configure testnet identity (one-time):**
```bash
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

stellar keys generate meno-deployer --network testnet
stellar keys fund meno-deployer --network testnet
```

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Test

```bash
cargo test
```

## Deploy

```bash
# Testnet
./deploy.sh testnet meno-deployer

# Contract ID is saved to .contract-id.testnet
```

## Contract ID

After deployment, the contract ID is written to `.contract-id.<network>`. Set this in the backend `.env`:

```
MENO_CONTRACT_ID=<contract-id>
STELLAR_NETWORK=testnet
```
