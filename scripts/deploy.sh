#!/bin/sh

echo ">> Building contract"

make build

if [ $? -ne 0 ]; then
  echo ">> Error building"
  exit 1
fi

echo ">> Deploying contract"

near dev-deploy --wasmFile ./build/nswap.wasm
