use crate::*;

impl Contract {
  pub fn get_config(&self) -> (AccountId, d128, d128) {
    (
      self.owner_id.clone(),
      self.base_rate,
      self.interest_multiplier,
    )
  }

  pub fn get_borrow_rate(
    &self,
    market_balance: Balance,
    total_liabilities: U128,
    total_reserves: U128,
  ) -> d128 {
    let total_value_in_market =
      U256::from(market_balance) + U256::from(total_liabilities.0) - U256::from(total_reserves.0);
    let utilization_ratio = if total_value_in_market == U256::zero() {
      d128::zero()
    } else {
      d128::from(total_liabilities.0 as u64) / d128::from(total_value_in_market.as_u128() as u64)
    };

    let rate = utilization_ratio * self.interest_multiplier + self.base_rate;
    rate
  }
}
