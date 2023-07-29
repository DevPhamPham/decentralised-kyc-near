#!/bin/sh

# echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release

# echo ">> Building Token contract"
# rustup target add wasm32-unknown-unknown
# cargo build --target wasm32-unknown-unknown --package Token --release

# echo ">> Building TokenMarketplace contract"
# rustup target add wasm32-unknown-unknown
# cargo build --target wasm32-unknown-unknown --package TokenMarketplace --release