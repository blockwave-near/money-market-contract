use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};

use crate::*;

pub fn setup_contract() -> (VMContextBuilder, Contract) {
  let mut context = VMContextBuilder::new();
  testing_env!(context
    .predecessor_account_id(ValidAccountId::try_from("owner").unwrap())
    .attached_deposit(1)
    .build());
  let contract = Contract::new(
    AccountId::from("owner"),
    D128::zero(),
    AccountId::from("stable_coin"),
    AccountId::from("overseer"),
    D128::zero(),
    D128::new_exp(1, -1),
    D128::new_exp(1, -1),
    D128::new_exp(100, 0),
    D128::new_exp(10, 0),
    D128::new_exp(11, -1),
    D128::new_exp(9, -1),
  );
  (context, contract)
}

#[test]
fn proper_compute_exchange_rate() {
  let (context, mut contract) = setup_contract();
  let mock_config = Config {
    owner_id: AccountId::from("owner"),
    stable_coin_contract: AccountId::from("stable_coin"),
    max_borrow_factor: D128::one(),
    overseer_contract: AccountId::from("overseer"),
  };
  let mock_state = State {
    total_liabilities: D128::new(50000u128 * 100_000_000),
    total_reserves: D128::new(550000u128 * 100_000_000),
    last_interest_updated: context.context.block_index,
    last_reward_updated: context.context.block_index,
    global_interest_index: D128::one(),
    global_reward_index: D128::zero(),
    anc_emission_rate: D128::one(),
    prev_stable_coin_total_supply: 0,
    prev_exchange_rate: D128::one(),
  };
  let mock_deposit_amount = Some(1000000u128);

  // TODO: how to resolve cross-contract call?
  // let exchange_rate = contract.compute_exchange_rate(mock_deposit_amount);

  // assert_eq!(exchange_rate, D128::new_exp(5, -1));
}
