#!/bin/bash
#
# Fetches and builds external libraries

. scripts/go_profile

GAIA_VERSION="v7.1.1"

build-gaiad() {
    git clone -b $GAIA_VERSION https://github.com/cosmos/gaia.git build/gaia
    cd build/gaia && make install
    cd ../..
}

build-gaiad