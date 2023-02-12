#!/bin/bash
#
# Installs all the necessary system and package dependencies
#
#

set -uexo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/setup_lib.sh"

build-packages
rust-setup
go-setup
