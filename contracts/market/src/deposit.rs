use crate::*;

impl Contract {
  pub fn deposit_stable(&mut self, deposit_amount: Balance) {
    if deposit_amount == 0 {
      env::panic("".as_bytes()); // TODO
    }

    let block_height = env::block_index();

    self.compute_interest(block_height, Some(deposit_amount));
    self.compute_reward(block_height);

    self
      .compute_exchange_rate(Some(deposit_amount))
      .then(ext_self::callback_deposit_stable(
        deposit_amount,
        &env::current_account_id(),
        NO_DEPOSIT,
        SINGLE_CALL_GAS,
      ));
  }

  pub fn redeem_stable(&mut self, burn_amount: Balance) {
    let block_height = env::block_index();

    self.compute_interest(block_height, None);
    self.compute_reward(block_height);

    self
      .compute_exchange_rate(None)
      .then(ext_self::callback_redeem_stable(
        burn_amount,
        &env::current_account_id(),
        NO_DEPOSIT,
        SINGLE_CALL_GAS,
      ));
  }

  pub fn assert_redeem_amount(&self, current_balance: Balance, redeem_amount: D128) {
    if redeem_amount + self.state.total_reserves > D128::new(current_balance * 100_000_000) {
      env::panic("".as_bytes());
    }
  }

  pub fn compute_exchange_rate(&self, deposit_amount: Option<Balance>) -> Promise {
    fungible_token::ft_total_supply(
      &self.config.stable_coin_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    )
    .then(ext_self::callback_compute_exchange_rate(
      env::account_balance(),
      deposit_amount,
      &self.config.stable_coin_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    ))
  }

  pub fn compute_exchange_rate_raw(
    &self,
    stable_coin_total_supply: u128,
    balance: Balance,
  ) -> D128 {
    if stable_coin_total_supply == 0 {
      return D128::one();
    }

    // TODO check overflow

    (balance * self.state.total_liabilities - self.state.total_reserves) / stable_coin_total_supply
  }
}
