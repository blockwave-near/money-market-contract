use crate::*;

#[near_bindgen]
impl Contract {
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }

  pub fn get_state(&self) -> State {
    self.state
  }

  pub fn get_borrower(&self, borrower: AccountId) -> BorrowerInfo {
    self.get_borrower_info_map(&borrower)
  }
}
