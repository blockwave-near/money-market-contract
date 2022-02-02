use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
  pub stable_coin_contract: AccountId,
  pub max_borrow_factor: D128,
  pub overseer_contract: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct State {
  pub anc_emission_rate: D128,
  pub total_liabilities: D128,
  pub total_reserves: D128,
  pub last_interest_updated: BlockHeight,
  pub global_interest_index: D128,
  pub prev_exchange_rate: D128,
  pub prev_stable_coin_total_supply: u128,
  pub last_reward_updated: BlockHeight,
  pub global_reward_index: D128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BorrowerInfo {
  pub interest_index: D128,
  pub reward_index: D128,
  pub loan_amount: Balance,
  pub pending_rewards: D128,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Collection {
  pub borrower_info_map: LookupMap<AccountId, BorrowerInfo>,
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
