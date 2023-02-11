use serde::{Deserialize, Serialize};

use super::types::Pubkey;

#[derive(Debug, Deserialize, Serialize)]
pub struct CosmosAccount {
    pub name: String,
    pub derivation_hint: String,
    pub pubkey: Pubkey,
    pub assets: Vec<CosmosAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CosmosAsset {
    Token(TokenAsset),
    Stake(StakeAsset),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenAsset {
    pub kind: CosmosTokenKind,
    pub amount: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StakeAsset {
    pub validator_name: String,
    pub address: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CosmosTokenKind {
    Atom,
}

#[test]
fn can_deserialize() {
    let json = r#"{
      "name": "default",
      "derivation_hint": "hint",
      "pubkey": "111111111111111111111",
      "assets": [
          {
            "type": "Token",
            "kind": "Atom",
            "amount": 1234
          },
          {
            "type": "Stake",
            "validator_name": "ValidatorOne",
            "address": "11111111111111111",
            "amount": 4321
          }
      ]
  }"#;

    let _: CosmosAccount = serde_json::from_str(json).unwrap();
}

#[test]
fn can_serialize() {
    let account = CosmosAccount {
        name: "default".into(),
        derivation_hint: "hint".into(),
        pubkey: "111111111".into(),
        assets: vec![
            CosmosAsset::Token(TokenAsset {
                kind: CosmosTokenKind::Atom,
                amount: 1234,
            }),
            CosmosAsset::Stake(StakeAsset {
                validator_name: "Validator".into(),
                address: "11111111111".into(),
                amount: 4321,
            }),
        ],
    };

    let _ = serde_json::to_string(&account).unwrap();
}
