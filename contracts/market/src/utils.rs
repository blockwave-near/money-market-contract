use crate::*;

// #[ext_contract(ext_interest_model)]
// pub trait ExtInterestModel {
//   fn get_borrow_rate(
//     &self,
//     market_balance: Balance,
//     total_liabilities: U128,
//     total_reserves: U128,
//   ) -> d128;
// }

// #[ext_contract(ext_distribution_model)]
// pub trait ExtDistributionModel {
//   fn get_emission_rate(
//     &self,
//     deposit_rate: d128,
//     target_deposit_rate: d128,
//     threshold_deposit_rate: d128,
//     current_emission_rate: d128,
//   ) -> d128;
// }

#[ext_contract(fungible_token)]
pub trait FungibleToken {
  fn ft_total_supply(&self) -> PromiseOrValue<U128>;

  fn ft_balance_of(&self, account_id: AccountId) -> PromiseOrValue<U128>;

  fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) -> Promise;
}

#[ext_contract(ext_overseer)]
pub trait OverseerContract {
  fn get_borrow_limit(&self, borrower: AccountId) -> PromiseOrValue<(AccountId, U128)>;

  fn get_target_deposit_rate(&self) -> PromiseOrValue<D128>;
}

#[ext_contract(ext_self)]
pub trait SelfContract {
  fn callback_compute_interset(
    &mut self,
    block_height: BlockHeight,
    deposit_amount: Option<Balance>,
  );

  fn callback_borrow_stable(&self, borrow_amount: Balance, liability: BorrowerInfo);

  fn callback_compute_exchange_rate(
    &self,
    amount: Balance,
    deposit_amount: Option<Balance>,
  ) -> D128;

  fn callback_deposit_stable(&self, deposit_amount: Balance);

  fn callback_redeem_stable(&self, burn_amount: Balance);

  fn callback_execute_epoch_operations(
    &self,
    deposit_rate: D128,
    target_deposit_rate: D128,
    threshold_deposit_rate: D128,
    distributed_intereset: U128,
  );
}

// TODO: need to move to each files(ex. borrow.ts, deposit.ts, etc )?
#[near_bindgen]
impl Contract {
  #[private]
  pub fn callback_compute_interset(
    &mut self,
    block_height: BlockHeight,
    deposit_amount: Option<Balance>,
  ) {
    assert_eq!(env::promise_results_count(), 2, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        // TODO
      }
      PromiseResult::Successful(result) => {
        let stable_coin_total_supply = near_sdk::serde_json::from_slice::<U128>(&result).unwrap().0;
        match env::promise_result(1) {
          PromiseResult::NotReady => unreachable!(),
          PromiseResult::Failed => {
            // TODO
          }
          PromiseResult::Successful(result) => {
            let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap().0
              - deposit_amount.unwrap_or(0);

            let borrow_rate = self.get_borrow_rate(
              balance,
              self.state.total_liabilities,
              self.state.total_reserves,
            );

            let target_deposit_rate = D128::one(); // TODO after overseer contract

            self.compute_interest_raw(
              block_height,
              stable_coin_total_supply,
              borrow_rate,
              target_deposit_rate,
            )
          }
        }
      }
    }
  }

  #[private]
  pub fn callback_borrow_stable(
    &mut self,
    borrow_amount: Balance,
    liability: &mut BorrowerInfo,
  ) -> (AccountId, u128) {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let (borrower, borrow_limit_raw) =
          near_sdk::serde_json::from_slice::<(AccountId, U128)>(&result).unwrap();
        let borrow_limit = borrow_limit_raw.0;

        if borrow_limit < borrow_amount + liability.loan_amount {
          env::panic("borrow exceed limit".as_bytes()); // TODO
        }

        let current_balance = env::account_balance();

        self.assert_max_borrow_factor(current_balance, borrow_amount);

        liability.loan_amount += borrow_amount;
        self.state.total_liabilities = self.state.total_liabilities + borrow_amount;

        self.add_borrower_info_map(&borrower, liability);

        return (borrower, borrow_amount);
      }
    }
  }

  #[private]
  pub fn callback_compute_exchange_rate(
    &self,
    amount: Balance,
    deposit_amount: Option<Balance>,
  ) -> D128 {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let stable_coin_total_supply = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();

        let balance = amount - deposit_amount.unwrap_or(0);

        self.compute_exchange_rate_raw(stable_coin_total_supply.into(), balance)
      }
    }
  }

  #[private]
  fn callback_deposit_stable(&mut self, deposit_amount: Balance) {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let exchange_rate = near_sdk::serde_json::from_slice::<D128>(&result).unwrap();
        let mint_amount: Balance = (deposit_amount / exchange_rate).as_u128();

        self.state.prev_stable_coin_total_supply += mint_amount;

        // TODO response mint_amount
      }
    }
  }

  #[private]
  fn callback_redeem_stable(&mut self, burn_amount: Balance) {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let exchange_rate = near_sdk::serde_json::from_slice::<D128>(&result).unwrap();
        let redeem_amount = burn_amount * exchange_rate;

        let current_balance = env::account_balance();

        self.assert_redeem_amount(current_balance, redeem_amount);

        self.state.prev_stable_coin_total_supply =
          self.state.prev_stable_coin_total_supply - burn_amount;

        // TODO response for redeem success
        let sender = env::predecessor_account_id();
      }
    }
  }
  #[private]
  pub fn callback_execute_epoch_operations(
    &mut self,
    deposit_rate: D128,
    target_deposit_rate: D128,
    threshold_deposit_rate: D128,
    distributed_intereset: U128,
  ) {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => {
        env::panic("fail".as_bytes());
      }
      PromiseResult::Successful(result) => {
        let stable_coin_total_supply = near_sdk::serde_json::from_slice::<U128>(&result).unwrap().0;
        let balance: Balance = env::account_balance() - distributed_intereset.0;

        let borrow_rate = self.get_borrow_rate(
          balance,
          self.state.total_liabilities,
          self.state.total_reserves,
        );
        let block_height = env::block_index();

        self.compute_interest_raw(
          block_height,
          stable_coin_total_supply,
          borrow_rate,
          target_deposit_rate,
        );

        self.state.prev_exchange_rate = self
          .compute_exchange_rate_raw(stable_coin_total_supply, balance + distributed_intereset.0);

        self.compute_reward(block_height);

        let total_reserves = self.state.total_reserves.as_u128();

        if total_reserves != 0 && balance > total_reserves {
          self.state.total_reserves = self.state.total_reserves - total_reserves;
          // TODO
        }

        self.state.anc_emission_rate = self.get_emission_rate(
          deposit_rate,
          target_deposit_rate,
          threshold_deposit_rate,
          self.state.anc_emission_rate,
        );
      }
    }
  }
}
