#!/usr/bin/env bash
set -e
cargo build --release --package nft-wallet --target wasm32-unknown-unknown
if ! command -v ic-cdk-optimizer &> /dev/null; then
    cargo install --root target ic-cdk-optimizer
    PATH="$PATH:$PWD/target/bin"
fi
cd target/wasm32-unknown-unknown/release
ic-cdk-optimizer nft_wallet.wasm -o nft_wallet-opt.wasm
