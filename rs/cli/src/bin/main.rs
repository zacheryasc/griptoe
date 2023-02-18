use clap::Parser;

fn main() {
    griptoe_cli::run(griptoe_cli::Cli::parse()).unwrap()
}
