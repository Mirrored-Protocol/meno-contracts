#!/usr/bin/env bash
set -e

NETWORK="${1:-testnet}"
IDENTITY="${2:-meno-deployer}"
WASM="target/wasm32-unknown-unknown/release/meno_contracts.wasm"

echo "Building contract..."
cargo build --target wasm32-unknown-unknown --release

echo "Deploying to $NETWORK as $IDENTITY..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo "Deployed: $CONTRACT_ID"
echo "$CONTRACT_ID" > ".contract-id.$NETWORK"
echo "Contract ID saved to .contract-id.$NETWORK"
