use crate::*;

#[near_bindgen]
impl Contract {
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }

  pub fn get_state(&self) -> State {
    self.state
  }
}
