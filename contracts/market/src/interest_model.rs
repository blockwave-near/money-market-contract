use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InterestModelConfig {
  pub base_rate: D128,
  pub interest_multiplier: D128,
}

#[near_bindgen]
impl Contract {
  pub fn get_borrow_rate(
    &self,
    market_balance: Balance,
    total_liabilities: D128,
    total_reserves: D128,
  ) -> D128 {
    let total_value_in_market =
      D128::new_exp(market_balance, 0) + total_liabilities - total_reserves;
    let utilization_ratio: D128 = if total_value_in_market == D128::zero() {
      D128::new(0)
    } else {
      total_liabilities / total_value_in_market
    };

    let rate = utilization_ratio * self.interest_model_config.interest_multiplier
      + self.interest_model_config.base_rate;
    rate
  }
}
