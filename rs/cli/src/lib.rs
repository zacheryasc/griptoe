use clap::Parser;
use cmd::{AccountCmd, KeygenCmd};
use thiserror::Error;

mod account;
mod cmd;
mod config;

pub static DEFAULT_CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(long, short = 'p')]
    pub config_path: Option<String>,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Actions related to accounts
    #[clap(subcommand)]
    Account(AccountCmd),

    /// Actions for deriving keys
    #[clap(subcommand)]
    Keygen(KeygenCmd),
}

pub fn run(cli: Cli) -> Result<()> {
    let config = config::Config::parse(cli.config_path.unwrap_or(DEFAULT_CONFIG_PATH.into()))?;
    match cli.command {
        Command::Account(cmd) => cmd.run(config),
        Command::Keygen(cmd) => cmd.run(),
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

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error(transparent)]
    TomlDeserializer(#[from] toml::de::Error),

    #[error("bip39 related error. Msg: {0}")]
    Bip39(String),

    #[error("Failed to parse input. Msg: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
