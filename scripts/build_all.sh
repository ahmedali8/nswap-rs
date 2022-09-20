#!/bin/sh

echo ">> Building all rust files"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release
