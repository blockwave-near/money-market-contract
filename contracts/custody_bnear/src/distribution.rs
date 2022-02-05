use crate::*;

// REWARD_THRESHOLD
// This value is used as the minimum reward claim amount
// thus if a user's reward is less than 1 ust do not send the ClaimRewards msg
const REWARDS_THRESHOLD: Balance = 1_000_000u128; // TODO check decimal

#[near_bindgen]
impl Contract {
  // Executor: overseer
  pub fn distribute_rewards(&self) {
    self.assert_overseer();

    ext_reward::get_accrued_rewards(
      env::current_account_id(),
      &self.config.reward_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    )
    .then(ext_self::callback_distribute_rewards(
      REWARDS_THRESHOLD,
      &env::current_account_id(),
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    ));
  }

  // Executor: itself
  pub fn distribute_hook(&self) {
    fungible_token::ft_balance_of(
      env::current_account_id(),
      &self.config.stable_coin_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    )
    .then(ext_self::callback_distribute_hook(
      &env::current_account_id(),
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    ));
  }

  // Executor: itself
  // pub fn swap_to_stable_denom(&self) {

  // }
}
