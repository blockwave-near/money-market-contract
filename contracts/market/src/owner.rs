use crate::*;

#[near_bindgen]
impl Contract {
  #[payable]
  pub fn update_interest_model_config(
    &mut self,
    // base_rate: Option<Fraction>,
    // interest_multiplier: Option<Fraction>,
    base_rate: Option<D128>,
    interest_multiplier: Option<D128>,
  ) {
    assert_one_yocto();
    self.assert_owner();

    if let Some(base_rate) = base_rate {
      self.interest_model_config.base_rate = base_rate;
    }

    if let Some(interest_multiplier) = interest_multiplier {
      self.interest_model_config.interest_multiplier = interest_multiplier;
    }
  }
}
