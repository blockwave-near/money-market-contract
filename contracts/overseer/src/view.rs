use crate::*;

#[near_bindgen]
impl Contract {
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }

  pub fn get_state(&self) -> State {
    self.state
  }

  pub fn get_target_deposit_rate(&self) -> D128 {
    self.config.target_deposit_rate
  }
}
