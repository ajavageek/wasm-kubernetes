#!/bin/sh
docker build -f Dockerfile.native -t wasm-kubernetes:native .
docker build --platform=linux/arm64 -f Dockerfile.embed -t wasm-kubernetes:embed .
docker build --platform=wasi/wasm,linux/arm64  -f Dockerfile.runtime -t wasm-kubernetes:runtime .
