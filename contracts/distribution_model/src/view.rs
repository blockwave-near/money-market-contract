use crate::*;

#[near_bindgen]
impl Contract {
  pub fn get_config(&self) -> (AccountId, d128, d128, d128, d128) {
    (
      self.owner_id.clone(),
      self.emission_cap,
      self.emission_floor,
      self.increment_multiplier,
      self.decrement_multiplier,
    )
  }

  pub fn get_emission_rate(
    &self,
    deposit_rate: d128,
    target_deposit_rate: d128,
    threshold_deposit_rate: d128,
    current_emission_rate: d128,
  ) -> d128 {
    let half_dec: d128 = d128!(1) + d128!(1);
    let mid_rate = (threshold_deposit_rate + target_deposit_rate) / half_dec;
    let high_trigger = (mid_rate + target_deposit_rate) / half_dec;
    let low_trigger = (mid_rate + threshold_deposit_rate) / half_dec;

    let emission_rate = if deposit_rate < low_trigger {
      current_emission_rate * self.increment_multiplier
    } else if deposit_rate > high_trigger {
      current_emission_rate * self.decrement_multiplier
    } else {
      current_emission_rate
    };

    let emission_rate = if emission_rate > self.emission_cap {
      self.emission_cap
    } else if emission_rate < self.emission_floor {
      self.emission_floor
    } else {
      emission_rate
    };
    emission_rate
  }
}
