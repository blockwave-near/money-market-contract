use crate::*;

#[near_bindgen]
impl Contract {
  #[payable]
  pub fn update_config(
    &mut self,
    owner_id: Option<AccountId>,
    stable_coin_contract: Option<AccountId>,
    max_borrow_factor: Option<D128>,
    overseer_contract: Option<AccountId>,
    collector_contract: Option<AccountId>,
  ) {
    self.assert_owner();
    assert_one_yocto();

    if let Some(owner_id) = owner_id {
      self.config.owner_id = owner_id;
    }
    if let Some(stable_coin_contract) = stable_coin_contract {
      self.config.stable_coin_contract = stable_coin_contract;
    }
    if let Some(max_borrow_factor) = max_borrow_factor {
      self.config.max_borrow_factor = max_borrow_factor;
    }
    if let Some(overseer_contract) = overseer_contract {
      self.config.overseer_contract = overseer_contract;
    }
    if let Some(collector_contract) = collector_contract {
      self.config.collector_contract = collector_contract;
    }
  }

  #[payable]
  pub fn update_interest_model_config(
    &mut self,
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
}
