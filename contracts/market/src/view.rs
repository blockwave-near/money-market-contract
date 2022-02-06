use crate::*;

#[near_bindgen]
impl Contract {
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }

  pub fn get_state(&self) -> State {
    self.state
  }

  // TODO Is is right?
  pub fn get_balance(&self) -> Balance {
    env::account_balance()
  }

  // TODO: Is it view method? Because it should use ext_contract method...
  pub fn get_epoch_state(
    &mut self,
    block_height: Option<BlockHeight>,
    distributed_intereset: Option<D128>,
  ) {
    let distributed_intereset = distributed_intereset.unwrap_or(D128::zero());
    // let stable_coin_total_supply =
    let balance = env::account_balance();

    if let Some(block_height) = block_height {
      if block_height < self.state.last_interest_updated {
        env::panic("block_height must bigger than last_interest_updated".as_bytes());
      }

      let borrow_rate = self.get_borrow_rate(
        balance,
        self.state.total_liabilities,
        self.state.total_reserves,
      );

      // ext_overseer::get_target_deposit_rate(
      //   &self.config.overseer_contract,
      //   NO_DEPOSIT,
      //   SINGLE_CALL_GAS,
      // ).then(
      //   // ext_self::callback_get_epoch_state
      // )
    }
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
