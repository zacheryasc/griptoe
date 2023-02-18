use clap::Parser;

use crate::{account::CosmosAccount, config::Config, read_file_to_string, Result};

#[derive(Debug, Parser)]
pub enum AccountCmd {
    /// Display all accounts
    Display,
}

impl AccountCmd {
    pub fn run(self, cfg: Config) -> Result<()> {
        match self {
            Self::Display => display_accounts(cfg.accounts_path),
        }
    }
}

fn display_accounts(path: String) -> Result<()> {
    let json = read_file_to_string(path)?;
    let accounts: Vec<CosmosAccount> = serde_json::from_str(&json)?;

    for account in accounts {
        println!("{account:#?}");
    }
    Ok(())
}
