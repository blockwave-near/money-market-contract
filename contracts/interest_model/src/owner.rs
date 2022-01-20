use crate::*;

#[near_bindgen]
impl Contract {
  #[payable]
  pub fn update_config(
    &mut self,
    owner_id: Option<AccountId>,
    base_rate: Option<d128>,
    interest_multiplier: Option<d128>,
  ) {
    assert_one_yocto();
    self.assert_owner();

    if let Some(owner_id) = owner_id {
      self.owner_id = owner_id;
    }

    if let Some(base_rate) = base_rate {
      self.base_rate = base_rate;
    }

    if let Some(interest_multiplier) = interest_multiplier {
      self.interest_multiplier = interest_multiplier;
    }
  }
}
