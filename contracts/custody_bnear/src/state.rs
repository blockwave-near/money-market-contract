use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
  pub owner_id: AccountId,
  pub overseer_contract: AccountId,
  pub collateral_token: AccountId,
  pub market_contract: AccountId,
  pub reward_contract: AccountId,
  pub liquidation_contract: AccountId,
  pub stable_coin_contract: AccountId,
  pub basset_info: BAssetInfo,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct State {}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Collection {
  pub borrower_info_map: LookupMap<AccountId, BorrowerInfo>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BAssetInfo {
  pub name: String,
  pub symbol: String,
  pub decimals: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BorrowerInfo {
  pub balance: Balance,
  pub spendable: Balance,
}

#[near_bindgen]
impl Contract {
  pub fn add_borrower_info_map(&mut self, key: &String, value: &BorrowerInfo) {
    self.collection.borrower_info_map.insert(&key, value);
  }

  pub fn get_borrower_info_map(&self, key: &String) -> BorrowerInfo {
    match self.collection.borrower_info_map.get(&key) {
      Some(value) => {
        let log_message = format!("Value from LookupMap is {:?}", value.clone());
        env::log(log_message.as_bytes());
        value
      }
      None => env::panic("".as_bytes()),
    }
  }
}
