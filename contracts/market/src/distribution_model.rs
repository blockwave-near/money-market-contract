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
  #[payable]
  pub fn update_distribution_model_config(
    &mut self,
    emission_cap: Option<D128>,
    emission_floor: Option<D128>,
    increment_multiplier: Option<D128>,
    decrement_multiplier: Option<D128>,
  ) {
    assert_one_yocto();
    self.assert_owner();

    if let Some(emission_cap) = emission_cap {
      self.distribution_model_config.emission_cap = emission_cap;
    }

    if let Some(emission_floor) = emission_floor {
      self.distribution_model_config.emission_floor = emission_floor;
    }

    if let Some(increment_multiplier) = increment_multiplier {
      self.distribution_model_config.increment_multiplier = increment_multiplier;
    }

    if let Some(decrement_multiplier) = decrement_multiplier {
      self.distribution_model_config.decrement_multiplier = decrement_multiplier;
    }
  }

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
