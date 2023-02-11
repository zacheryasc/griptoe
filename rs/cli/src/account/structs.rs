pub struct CosmosAccount {
  pub name: String,
  pub derivation_hint: String,
  pub pubkey: Pubkey,
  pub assets: Vec<CosmosAsset>,
}

pub enum CosmosAsset {
  Token(TokenAsset),
  Stake(StakeAsset),
}

pub struct TokenAsset {
  pub kind: CosmosTokenKind,
  pub amount: u64,
}

pub struct StakeAsset {
  pub validator_name: String,
  pub address: Pubkey,
}

pub enum CosmosTokenKind {
  Atom
}
