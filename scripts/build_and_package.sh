#!/bin/bash
#
# Compiles and packages binaries for export
#

set -euxo pipefail

mkdir -p build
mkdir -p build/bin

cargo build --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/debug/main build/griptoe-cli

GAIAD_PATH=$(which gaiad)
cp $GAIAD_PATH build/bin/gaiad
cp target/geth/build/geth build/bin/geth
