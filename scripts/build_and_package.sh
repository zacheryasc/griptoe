#!/bin/bash
#
# Compiles and packages binaries for export
#

set -euxo pipefail

mkdir -p build

cargo build --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/debug/main build/griptoe-cli

GAIAD_PATH=$(which gaiad)
cp $GAIAD_PATH build/gaiad
