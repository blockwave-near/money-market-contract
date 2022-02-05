use crate::*;

#[near_bindgen]
impl Contract {
  pub fn borrow_stable(&mut self, borrow_amount: Balance) {
    let borrower = env::predecessor_account_id();

    let mut liability: BorrowerInfo = self.get_borrower_info_map(&borrower);

    self.compute_interest(env::block_index(), None);
    self.compute_borrower_interest(&mut liability);

    self.compute_reward(env::block_index());
    self.compute_borrower_reward(&mut liability);

    ext_overseer::get_borrow_limit(
      borrower,
      &self.config.overseer_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    )
    .then(ext_self::callback_borrow_stable(
      borrow_amount,
      liability,
      &env::current_account_id(),
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    ));
  }

  pub fn repay_stable_from_liquidation(&mut self, borrower: AccountId, prev_balance: Balance) {
    self.assert_overseer();

    let cur_balance: Balance = env::account_balance();
    let balance_diff: u128 = cur_balance - prev_balance;
    self.repay_stable(borrower, balance_diff);
  }

  pub fn repay_stable(&mut self, borrower: AccountId, amount: Balance) {
    if amount == 0 {
      env::panic("".as_bytes());
    }

    let mut liability: BorrowerInfo = self.get_borrower_info_map(&borrower);

    let block_height = env::block_index();

    self.compute_interest(block_height, Some(amount));
    self.compute_borrower_interest(&mut liability);

    self.compute_reward(block_height);
    self.compute_borrower_reward(&mut liability);

    let repay_amount: Balance;
    if liability.loan_amount < amount {
      repay_amount = liability.loan_amount;
      liability.loan_amount = 0;

      fungible_token::ft_transfer(
        borrower.clone(),
        U128::from(amount - repay_amount),
        None,
        &self.config.stable_coin_contract,
        1,
        SINGLE_CALL_GAS,
      );
    } else {
      repay_amount = amount;
      liability.loan_amount = liability.loan_amount - repay_amount;
    }

    self.state.total_liabilities = self.state.total_liabilities - repay_amount;

    self.add_borrower_info_map(&borrower, &liability);
  }

  pub fn claim_reward(&mut self) {
    let borrower = env::predecessor_account_id();
    let mut liability: BorrowerInfo = self.get_borrower_info_map(&borrower);

    let block_height = env::block_index();

    self.compute_interest(block_height, None);
    self.compute_borrower_interest(&mut liability);

    self.compute_reward(block_height);
    self.compute_borrower_reward(&mut liability);

    let claim_amount: Balance = liability.pending_rewards.as_u128();
    liability.pending_rewards = liability.pending_rewards - claim_amount; // TODO why not assign 0 ?

    self.add_borrower_info_map(&borrower, &liability);

    // TODO distributor contract should send reward to borrwer
  }

  pub(crate) fn compute_interest(
    &self,
    block_height: BlockHeight,
    deposit_amount: Option<Balance>,
  ) {
    if self.state.last_interest_updated >= block_height {
      return;
    }

    self.ft_info().then(ext_self::callback_compute_interset(
      block_height,
      deposit_amount,
      &env::current_account_id(),
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    ));
  }

  pub(crate) fn compute_interest_raw(
    &mut self,
    block_height: BlockHeight,
    stable_coin_total_supply: u128,
    borrow_rate: D128,
    target_deposit_rate: D128,
  ) {
    if self.state.last_interest_updated >= block_height {
      return;
    }

    let passed_blocks: BlockHeight = block_height - self.state.last_interest_updated;

    let interest_factor: D128 = borrow_rate * passed_blocks as u128;
    let interest_accrued: D128 = self.state.total_liabilities * interest_factor;

    self.state.global_interest_index =
      (D128::one() + interest_factor) * self.state.global_interest_index;
    self.state.total_liabilities = interest_accrued * self.state.total_liabilities;

    let balance: Balance = env::account_balance();

    let mut exchange_rate: D128 = self.compute_exchange_rate_raw(stable_coin_total_supply, balance);
    let effective_deposit_rate: D128 = exchange_rate / self.state.prev_exchange_rate;
    let deposit_rate: D128 = (effective_deposit_rate - D128::one()) / passed_blocks as u128;

    if deposit_rate > target_deposit_rate {
      let excess_deposit_rate: D128 = deposit_rate - target_deposit_rate;
      let prev_deposit: u128 =
        (self.state.prev_exchange_rate).mul_int(self.state.prev_stable_coin_total_supply);

      let excess_yield: u128 = excess_deposit_rate.mul_int(prev_deposit * passed_blocks as u128);

      self.state.total_reserves = excess_yield + self.state.total_reserves;
      exchange_rate = self.compute_exchange_rate_raw(stable_coin_total_supply, balance);
    }

    self.state.prev_stable_coin_total_supply = stable_coin_total_supply;
    self.state.prev_exchange_rate = exchange_rate;
    self.state.last_interest_updated = block_height;
  }

  pub fn compute_borrower_interest(&self, liability: &mut BorrowerInfo) {
    liability.loan_amount = (liability.loan_amount * self.state.global_interest_index
      / liability.interest_index)
      .as_u128();
    liability.interest_index = self.state.global_interest_index;
  }

  pub fn compute_reward(&mut self, block_height: BlockHeight) {
    if self.state.last_reward_updated >= block_height {
      return;
    }

    let passed_blocks: BlockHeight = block_height - self.state.last_reward_updated;
    let reward_accrued: u128 = self.state.anc_emission_rate.mul_int(passed_blocks.into());
    let borrow_amount = self.state.total_liabilities / self.state.global_interest_index;

    if reward_accrued != 0 && borrow_amount != D128::zero() {
      self.state.global_reward_index =
        self.state.global_reward_index + reward_accrued / borrow_amount;
    }
  }

  pub(crate) fn compute_borrower_reward(&self, liability: &mut BorrowerInfo) {
    liability.pending_rewards = liability.pending_rewards
      + liability.loan_amount / self.state.global_interest_index
        * (self.state.global_reward_index - liability.reward_index);
    liability.reward_index = self.state.global_reward_index;
  }

  pub fn assert_max_borrow_factor(&self, current_balance: Balance, borrow_amount: Balance) {
    if self.state.total_liabilities + borrow_amount
      > (current_balance + self.state.total_liabilities - self.state.total_reserves)
        * self.config.max_borrow_factor
    {
      env::panic("Max Borrow Factor Reached".as_bytes());
    }

    if borrow_amount + self.state.total_reserves - current_balance > D128::zero() {
      env::panic("No Stable Available".as_bytes());
    }
  }
}
