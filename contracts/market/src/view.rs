use crate::*;

#[near_bindgen]
impl Contract {
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }

  pub fn get_state(&self) -> State {
    self.state
  }

  pub fn get_balance(&self) -> Balance {
    env::account_balance()
  }

  pub fn get_borrower_info(
    &mut self,
    borrower: AccountId,
    block_height: Option<BlockHeight>,
  ) -> BorrowerInfo {
    let mut borrwer_info: BorrowerInfo = self.get_borrower_info_map(&borrower);

    let block_height = if let Some(block_height) = block_height {
      block_height
    } else {
      env::block_index()
    };

    self.compute_interest(block_height, None);
    self.compute_borrower_interest(&mut borrwer_info);

    self.compute_reward(block_height);
    self.compute_borrower_reward(&mut borrwer_info);

    borrwer_info
  }

  // pub fn get_borrower_infos(
  //   &mut self,
  //   start_after: Option<AccountId>,
  //   limit: Option<u32>,
  // ) -> Vec<BorrowerInfo> {
  //   let start_after = if let Some(start_after) = start_after {
  //     Some(start_after)
  //   } else {
  //     None
  //   };

  //   let borrower_infos: Vec<BorrowerInfo> =
  // }
}
