use crate::*;

#[ext_contract(fungible_token)]
pub trait FungibleToken {
  fn ft_total_supply(&self) -> U128;

  fn ft_balance_of(&self, account_id: AccountId) -> U128;

  fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
  fn ft_transfer_call(
    &mut self,
    receiver_id: AccountId,
    amount: U128,
    memo: Option<String>,
    msg: String,
  ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_reward)]
pub trait RewardContract {
  fn get_account_stake_rewards(&self, account_id: AccountId) -> U128;
}

#[ext_contract(ext_self)]
pub trait Contract {
  fn callback_distribute_rewards(&mut self, REWARDS_THRESHOLD: Balance);
  fn callback_distribute_hook(&self);
}

#[near_bindgen]
impl Contract {
  #[private]
  fn callback_distribute_rewards(&mut self, REWARDS_THRESHOLD: Balance) {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let accrued_rewards: Balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap().0;

        if accrued_rewards < REWARDS_THRESHOLD {
          return;
        }

        self.swap_to_stable_denom();
      }
    }
  }

  fn callback_distribute_hook(&self) {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let reward_amount: Balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap().0;

        if reward_amount != 0 {
          fungible_token::ft_transfer(
            self.config.overseer_contract.clone(),
            U128::from(reward_amount),
            None,
            &self.config.stable_coin_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
          );
        }
      }
    }
  }
}
