#!/bin/bash
#
# Installs all the necessary system and package dependencies
#
#

set -uexo pipefail

GO_VER=1.19

build-packages() {
	sudo apt update
	sudo apt install --assume-yes build-essential
	sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler gcc musl-dev tree
}

rust-setup() {
  # rust base
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $(pwd)/rustup.sh
	sh $(pwd)/rustup.sh -y
	rm $(pwd)/rustup.sh
	source $HOME/.cargo/env
	rustup component add rust-std-x86_64-unknown-linux-musl
}

## WARNING: modifies your bash profile
go-setup() {
	mkdir -p target

	curl -OL https://golang.org/dl/go$GO_VER.linux-amd64.tar.gz
	sudo tar -C /usr/local -xvf go$GO_VER.linux-amd64.tar.gz
	rm go$GO_VER.linux-amd64.tar.gz

	LINE='export PATH=$PATH:/usr/local/go/bin'
	SOURCE=/home/$USER/.bashrc
	grep -qF -- "$LINE" "$SOURCE" || echo "$LINE" >> "$SOURCE"
	. $SOURCE

	LINE='export PATH=$PATH:$(go env GOPATH)/bin'
	grep -qF -- "$LINE" "$SOURCE" || echo "$LINE" >> "$SOURCE"
	. $SOURCE

	LINE='export CGO_ENABLED=0'
	grep -qF -- "$LINE" "$SOURCE" || echo "$LINE" >> "$SOURCE"
	. $SOURCE
}

python-setup() {
	wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
	chmod +x Miniconda3-latest-Linux-x86_64.sh
	./Miniconda3-latest-Linux-x86_64.sh -b || true
	rm Miniconda3-latest-Linux-x86_64.sh

	LINE='export PATH=$PATH:/home/$USER/miniconda3/bin'
	SOURCE=/home/$USER/.bashrc
	grep -qF -- "$LINE" "$SOURCE" || echo "$LINE" >> "$SOURCE"
	. $SOURCE

	conda create --name griptoe-env python=3.6.7
	conda activate myenv
	conda install -y pillow pip
}

setup-all() {
    build-packages
    rust-setup
    go-setup
}
