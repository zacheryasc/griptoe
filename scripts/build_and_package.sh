#!/bin/bash
#
# Compiles and packages binaries for export
#

set -euxo pipefail

mkdir -p build
mkdir -p build/bin

## Rust related
cargo build --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/debug/main build/griptoe-cli

## Go related
GAIAD_PATH=$(which gaiad)
cp $GAIAD_PATH build/bin/gaiad
cp target/geth/build/bin/geth build/bin/geth
