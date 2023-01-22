#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo update --aggressive
marine build --release

mkdir -p artifacts
rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/fevm_json_rpc.wasm artifacts/
marine aqua artifacts/fevm_json_rpc.wasm -s fevm_json_rpc -i Fevm_json_rpc > ./aqua/fevm_json_rpc.aqua

RUST_LOG="info" mrepl --quiet Config.toml