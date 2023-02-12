use clap::Parser;

use crate::{account::CosmosAccount, read_file_to_string, Result, DEFAULT_ACCOUNTS_PATH};

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

fn display_accounts() -> Result<()> {
    let json = read_file_to_string(format!("{}/cosmos.json", DEFAULT_ACCOUNTS_PATH.to_owned()))?;
    let accounts: Vec<CosmosAccount> = serde_json::from_str(&json)?;

    for account in accounts {
        println!("{account:#?}");
    }
    Ok(())
}
