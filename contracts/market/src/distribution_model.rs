use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DistributionModelConfig {
  pub emission_cap: D128,
  pub emission_floor: D128,
  pub increment_multiplier: D128,
  pub decrement_multiplier: D128,
}

#[near_bindgen]
impl Contract {
  pub fn get_emission_rate(
    &self,
    deposit_rate: D128,
    target_deposit_rate: D128,
    threshold_deposit_rate: D128,
    current_emission_rate: D128,
  ) -> D128 {
    let half_dec: D128 = D128::one() + D128::one();
    let mid_rate = (threshold_deposit_rate + target_deposit_rate) / half_dec;
    let high_trigger = (mid_rate + target_deposit_rate) / half_dec;
    let low_trigger = (mid_rate + threshold_deposit_rate) / half_dec;

    let emission_rate = if deposit_rate < low_trigger {
      current_emission_rate * self.distribution_model_config.increment_multiplier
    } else if deposit_rate > high_trigger {
      current_emission_rate * self.distribution_model_config.decrement_multiplier
    } else {
      current_emission_rate
    };

    let emission_rate = if emission_rate > self.distribution_model_config.emission_cap {
      self.distribution_model_config.emission_cap
    } else if emission_rate < self.distribution_model_config.emission_floor {
      self.distribution_model_config.emission_floor
    } else {
      emission_rate
    };
    emission_rate
  }
}
