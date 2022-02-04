use crate::*;

#[near_bindgen]
impl Contract {
  pub(crate) fn assert_owner(&self) {
    assert_eq!(
      env::predecessor_account_id(),
      self.config.owner_id,
      "Can only be called by the owner"
    );
  }

  pub(crate) fn assert_overseer(&self) {
    assert_eq!(
      env::predecessor_account_id(),
      self.config.overseer_contract,
      "Can only be called by the overseer"
    );
  }

  pub fn ft_info(&self) -> Promise {
    ext_stable_coin::ft_total_supply(
      &self.config.stable_coin_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    )
    .and(ext_stable_coin::ft_balance_of(
      env::current_account_id(),
      &self.config.stable_coin_contract,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    ))
  }
}
