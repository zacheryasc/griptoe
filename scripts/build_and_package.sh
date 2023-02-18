#!/bin/bash
#
# Compiles and packages binaries for export
#

set -euxo pipefail

mkdir -p build
mkdir -p build/bin

## Rust related
# cargo build --target x86_64-unknown-linux-musl
# cp target/x86_64-unknown-linux-musl/debug/main build/griptoe-cli

# ## Go related
# GAIAD_PATH=$(which gaiad)
# cp $GAIAD_PATH build/bin/gaiad
# cp target/geth/build/bin/geth build/bin/geth

## Python/Conda related
mkdir -p build/conda
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
mv Miniconda3-latest-Linux-x86_64.sh build/conda/miniconda.sh

# conda list --explicit > tmp_specfile.txt
# cat tmp_specfile.txt | grep "^http" | xargs -I xxx wget -P build/conda/pkgs xxx; rm tmp_specfile.txt;

# echo "@EXPLICIT" > build/conda/specfile.txt
# tree -i build/conda/pkgs >> build/conda/specfile.txt
# grep -v 'build/conda' build/conda/specfile.txt > tmp.txt
# grep -v 'directories' tmp.txt > build/conda/specfile.txt; rm tmp.txt;
