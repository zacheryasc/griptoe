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

pub fn read_file_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String> {
    let mut buf = String::new();
    std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map(|mut f| std::io::Read::read_to_string(&mut f, &mut buf))
        .map_err(Error::Io)??;

    Ok(buf)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
