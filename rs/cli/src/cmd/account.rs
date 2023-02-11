use std::io::Read;

use clap::Parser;

use crate::{account::CosmosAccount, Error, Result, DEFAULT_ACCOUNTS_PATH};

#[derive(Debug, Parser)]
pub enum AccountCmd {
    Display,
}

impl AccountCmd {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Display => display_accounts(),
        }
    }
}

fn read_file_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String> {
    let mut buf = String::new();
    std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map(|mut f| f.read_to_string(&mut buf))
        .map_err(Error::Io)??;

    Ok(buf)
}

fn display_accounts() -> Result<()> {
    let json = read_file_to_string(format!("{}/cosmos.json", DEFAULT_ACCOUNTS_PATH.to_owned()))?;
    let accounts: Vec<CosmosAccount> = serde_json::from_str(&json)?;

    for account in accounts {
        println!("{account:?}");
    }
    Ok(())
}
