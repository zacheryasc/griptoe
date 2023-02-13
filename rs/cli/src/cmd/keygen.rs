use std::str::FromStr;

use bip39::Mnemonic;
use blake2::Digest;
use clap::Parser;
use sha2::Sha256;

use crate::{Error, Result};

#[derive(Debug, Parser)]
pub enum KeygenCmd {
    /// Hash provided stdin
    Hash(HashOpts),

    /// Convert stdin
    Convert(ConvertOpts),
}

#[derive(Debug, Parser)]
pub struct HashOpts {
    /// The byte representation of the input. Possible formats include:
    ///
    /// "raw" or "raw-string" -- A utf-8 encoded string
    ///     
    /// "hex" or "hexadecimal" -- A hexadecimal string
    ///     
    /// "bip39" -- A list of words representing a BIP39 encoded value
    #[clap(long, short = 'i', default_value = "raw")]
    pub input_fmt: Representation,

    /// The byte representation of the output. Potential formats include:
    ///
    /// "raw" or "raw-string" - A utf-8 encoded string
    ///     
    /// "hex" or "hexadecimal" - A hexadecimal string
    ///     
    /// "bip39" - A list of words representing a BIP39 encoded value
    #[clap(long, short = 'o', default_value = "hex")]
    pub output_fmt: Representation,

    /// Number of times to successively apply the hash function
    #[clap(long, short = 'r')]
    pub repetitions: Option<u64>,

    /// Prompt the user to supply an additional string to append when hashing
    #[clap(long, short = 'a', default_value_t = false)]
    pub add_string: bool,

    /// The hash function to apply to the input. Potential functions include:
    ///
    /// "sha" or "sha2" or "sha256" - the SHA2 256-bit hash
    ///
    /// "blake" or "blake2" or "blake2s" or "blake2s256" - the Blake2s 256-bit hash
    #[clap(long, short = 'f', default_value = "sha256")]
    pub function: HashFunction,

    /// Data to hash
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct ConvertOpts {
    /// The byte representation of the input. Possible formats include:
    ///
    /// "raw" or "raw-string" -- A utf-8 encoded string
    ///     
    /// "hex" or "hexadecimal" -- A hexadecimal string
    ///     
    /// "bip39" -- A list of words representing a BIP39 encoded value
    #[clap(long, short = 'i', default_value = "raw")]
    pub input_fmt: Representation,

    /// The byte representation of the output. Potential formats include:
    ///
    /// "raw" or "raw-string" - A utf-8 encoded string
    ///     
    /// "hex" or "hexadecimal" - A hexadecimal string
    ///     
    /// "bip39" - A list of words representing a BIP39 encoded value
    #[clap(long, short = 'o', default_value = "hex")]
    pub output_fmt: Representation,

    /// Data to hash
    pub input: String,
}

/// Possible byte representations
#[derive(Debug, Clone, Parser)]
pub enum Representation {
    /// A utf-8 encoded string
    RawString,
    /// A hexadecimal number
    Hexadecimal,
    /// a BIP39 set of words
    Bip39,
}

impl FromStr for Representation {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Representation::*;
        if s == "raw" || s == "raw-string" {
            return Ok(RawString);
        }
        if s == "hex" || s == "hexadecimal" {
            return Ok(Hexadecimal);
        }
        if s == "bip39" {
            return Ok(Bip39);
        }
        Err(Error::Parse("could not parse Representation".into()))
    }
}

#[derive(Debug, Clone, Parser)]
pub enum HashFunction {
    Sha256,
    Blake2s256,
}

impl FromStr for HashFunction {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use HashFunction::*;
        let s = s.to_ascii_lowercase();
        if s == "sha" || s == "sha2" || s == "sha256" {
            return Ok(Sha256);
        }
        if s == "blake" || s == "blake2" || s == "blake2s" || s == "blake2s256" {
            return Ok(Blake2s256);
        }
        Err(Error::Parse("could not parse hash function choice".into()))
    }
}

fn apply_sha(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize()[..].to_vec()
}

fn apply_blake2s(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = blake2::Blake2s256::new();
    hasher.update(bytes);
    hasher.finalize()[..].to_vec()
}

fn process_hash(opts: HashOpts) -> Result<()> {
    use Representation::*;

    let mut input = match opts.input_fmt {
        RawString => opts.input.as_bytes().to_vec(),
        Hexadecimal => hex::decode(opts.input.as_bytes())?,
        Bip39 => Mnemonic::from_phrase(&opts.input, bip39::Language::English)
            .map_err(|e| Error::Bip39(e.to_string()))?
            .entropy()
            .to_vec(),
    };

    if opts.add_string {
        println!("Please enter additional string: ");
        let buf = &mut String::new();
        std::io::stdin().read_line(buf)?;
        input.extend(buf.as_bytes());
    }

    let hash_fn = |bytes: &[u8]| match opts.function {
        HashFunction::Sha256 => apply_sha(bytes),
        HashFunction::Blake2s256 => apply_blake2s(bytes),
    };
    let mut hash_bytes = input;
    for _ in 0..opts.repetitions.unwrap_or(1) {
        hash_bytes = hash_fn(&hash_bytes);
    }

    let output = match opts.output_fmt {
        RawString => String::from_utf8_lossy(&hash_bytes).into_owned(),
        Hexadecimal => hex::encode(hash_bytes),
        Bip39 => {
            let mnemonic = Mnemonic::from_entropy(&hash_bytes, bip39::Language::English)
                .map_err(|e| Error::Bip39(e.to_string()))?;
            mnemonic.phrase().to_string()
        }
    };
    println!("{output}");
    Ok(())
}

fn process_convert(opts: ConvertOpts) -> Result<()> {
    use Representation::*;

    let input = match opts.input_fmt {
        RawString => opts.input.as_bytes().to_vec(),
        Hexadecimal => hex::decode(opts.input.as_bytes())?,
        Bip39 => Mnemonic::from_phrase(&opts.input, bip39::Language::English)
            .map_err(|e| Error::Bip39(e.to_string()))?
            .entropy()
            .to_vec(),
    };

    let output = match opts.output_fmt {
        RawString => String::from_utf8_lossy(&input).into_owned(),
        Hexadecimal => hex::encode(input),
        Bip39 => {
            let mnemonic = Mnemonic::from_entropy(&input, bip39::Language::English)
                .map_err(|e| Error::Bip39(e.to_string()))?;
            mnemonic.phrase().to_string()
        }
    };

    println!("{output}");

    Ok(())
}

impl KeygenCmd {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Convert(opts) => process_convert(opts),
            Self::Hash(opts) => process_hash(opts),
        }
    }
}
