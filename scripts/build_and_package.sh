#!/bin/bash
#
# Compiles and packages binaries for export
#

set -euxo pipefail

mkdir -p build

cargo build --release
cp target/release/main build/griptoe-cli

GAIAD_PATH=$(which gaiad)
cp $GAIAD_PATH build/gaiad
