use crate::*;

#[near_bindgen]
impl Contract {
  #[payable]
  pub fn update_config(
    &mut self,
    owner_id: Option<AccountId>,
    emission_cap: Option<d128>,
    emission_floor: Option<d128>,
    increment_multiplier: Option<d128>,
    decrement_multiplier: Option<d128>,
  ) {
    assert_one_yocto();
    self.assert_owner();

    if let Some(owner_id) = owner_id {
      self.owner_id = owner_id;
    }

    if let Some(emission_cap) = emission_cap {
      self.emission_cap = emission_cap;
    }

    if let Some(emission_floor) = emission_floor {
      self.emission_floor = emission_floor;
    }

    if let Some(increment_multiplier) = increment_multiplier {
      self.increment_multiplier = increment_multiplier;
    }

    if let Some(decrement_multiplier) = decrement_multiplier {
      self.decrement_multiplier = decrement_multiplier;
    }
  }
}
