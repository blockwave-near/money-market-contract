use crate::*;

#[ext_contract(ext_stable_coin)]
pub trait ExtStableCoinContract {
  fn ft_total_supply(&self) -> PromiseOrValue<U128>;

  fn ft_balance_of(&self, account_id: AccountId) -> PromiseOrValue<U128>;

  fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) -> Promise;
}

#[ext_contract(ext_self)]
pub trait SelfContract {}

// TODO: need to move to each files(ex. borrow.ts, deposit.ts, etc )?
#[near_bindgen]
impl Contract {}
