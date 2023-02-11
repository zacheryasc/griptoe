use clap::Parser;

fn main() {
    griptoe_cli::run(griptoe_cli::Command::parse()).unwrap()
}
