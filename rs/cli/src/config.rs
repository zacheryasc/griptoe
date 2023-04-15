// use serde::Deserialize;

// use crate::{read_file_to_string, Error, Result};

// #[derive(Deserialize)]
// pub struct Config {
//     pub accounts_path: String,
// }

// impl Config {
//     pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
//         toml::from_str(&read_file_to_string(path)?).map_err(Error::TomlDeserializer)
//     }
// }
