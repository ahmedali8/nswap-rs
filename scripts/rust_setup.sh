#/bin/bash

# https://gist.github.com/vncsna/64825d5609c146e80de8b1fd623011ca
set -euo pipefail

VER=2021-11-01
rustup toolchain install stable-$VER
rustup default stable-$VER
rustup target add wasm32-unknown-unknown
