use clap::Parser;
use cmd::AccountCmd;
use thiserror::Error;

mod account;
mod cmd;

lazy_static::lazy_static! {
    pub static ref DEFAULT_ACCOUNTS_PATH: String = format!("{}/.accounts", shellexpand::env("$PWD").unwrap());
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Actions related to accounts
    #[clap(subcommand)]
    Account(AccountCmd),
}

pub fn run(cmd: Command) -> Result<()> {
    match cmd {
        Command::Account(cmd) => cmd.run(),
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
