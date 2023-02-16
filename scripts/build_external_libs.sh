#!/bin/bash
#
# Fetches and builds external libraries

GAIA_VERSION="v7.1.1"

build-gaiad() {
    mkdir -p target
    git clone -b $GAIA_VERSION https://github.com/cosmos/gaia.git target/gaia
    cd target/gaia && make install
    cd ../..
}

build-geth() {
    git clone https://github.com/ethereum/go-ethereum.git target/geth
    cd target/geth && make all
    cd ../..
}
build-gaiad
build-geth