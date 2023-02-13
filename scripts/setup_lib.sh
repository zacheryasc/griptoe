#!this_is_supposed_to_be_sourced,_not_executed_directly

GO_VER=1.18.2

build-packages() {
	sudo apt update
	sudo apt install --assume-yes build-essential
	sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler gcc
}

rust-setup() {
  # rust base
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $(pwd)/rustup.sh
	sh $(pwd)/rustup.sh -y
	rm $(pwd)/rustup.sh
	source $HOME/.cargo/env
}

## WARNING: modifies your bash profile
go-setup() {
	set +e
	mkdir target
	set -e

	curl -OL https://golang.org/dl/go$GO_VER.linux-amd64.tar.gz
	sudo tar -C /usr/local -xvf go$GO_VER.linux-amd64.tar.gz
	rm go$GO_VER.linux-amd64.tar.gz

	LINE='export PATH=$PATH:/usr/local/go/bin'
	SOURCE=/home/$USER/.bashrc
	grep -qF -- "$LINE" "$SOURCE" || echo "$LINE" >> "$SOURCE"
	. $SOURCE

	LINE='export PATH=$PATH:$(go env GOPATH)/bin'
	SOURCE=/home/$USER/.bashrc
	grep -qF -- "$LINE" "$SOURCE" || echo "$LINE" >> "$SOURCE"
	. $SOURCE
}