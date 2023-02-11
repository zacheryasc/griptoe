#!this_is_supposed_to_be_sourced,_not_executed_directly

build-packages() {
	sudo apt update
	sudo apt install --assume-yes build-essential
	sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
}

rust-setup() {
  # rust base
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $(pwd)/rustup.sh
	sh $(pwd)/rustup.sh -y
	rm $(pwd)/rustup.sh
	source $HOME/.cargo/env
}

