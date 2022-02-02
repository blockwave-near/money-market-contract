use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
  pub oracle_contrract: AccountId,
  pub market_contract: AccountId,
  pub liquidation_contract: AccountId,
  pub collector_contract: AccountId,
  // pub epoch_period: BlockHeight,
  // pub threshold_deposit_rate: D128,
  // pub target_deposit_rate: D128,
  // pub buffer_distribution_factor: D128,
  // pub anc_purchase_factor: D128,
  // pub price_timeframe: BlockHeight,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct State {
  // pub deposit_rate: D128,
// pub prev_stable_coin_total_supply: Balance,
// pub prev_exchange_rate: D128,
// pub prev_interest_buffer: u128,
// pub last_executed_height: BlockHeight,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Collection {
  pub white_list_elem_map: LookupMap<AccountId, WhitelistElem>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct WhitelistElem {
  pub name: String,
  pub symbol: String,
  pub max_ltv: D128,
  pub custody_contract: D128,
}

#[near_bindgen]
impl Contract {
  pub fn add_white_list_elem_map(&mut self, key: &String, value: &WhitelistElem) {
    self.collection.white_list_elem_map.insert(&key, value);
  }

  pub fn get_white_list_elem_map(&self, key: &String) -> WhitelistElem {
    match self.collection.white_list_elem_map.get(&key) {
      Some(value) => {
        let log_message = format!("Value from LookupMap is {:?}", value.clone());
        env::log(log_message.as_bytes());
        value
      }
      None => env::panic("".as_bytes()),
    }
  }
}
