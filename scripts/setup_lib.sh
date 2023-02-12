#!this_is_supposed_to_be_sourced,_not_executed_directly

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
	mkdir build
	set -e

	curl -OL https://golang.org/dl/go1.18.2.linux-amd64.tar.gz
	sudo tar -C /usr/local -xvf go1.18.2.linux-amd64.tar.gz
	rm go1.18.2.linux-amd64.tar.gz

	cd ..
	echo "# GO path vars"
	echo "export PATH=$PATH:/usr/local/go/bin" &> .bashrc
	. .bashrc
	echo "export PATH=$PATH:$(go env GOPATH)/bin" &> .bashrc
}