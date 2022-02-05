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
fn proper_compute_borrower_interest() {
  let (context, mut contract) = setup_contract();
  let mock_state = State {
    total_liabilities: D128::new(1000000u128 * 100_000_000),
    total_reserves: D128::zero(),
    last_interest_updated: context.context.block_index,
    last_reward_updated: context.context.block_index,
    global_interest_index: D128::one(),
    global_reward_index: D128::zero(),
    anc_emission_rate: D128::one(),
    prev_stable_coin_total_supply: 0,
    prev_exchange_rate: D128::one(),
  };
  let mut liability1 = BorrowerInfo {
    interest_index: D128::one(),
    reward_index: D128::zero(),
    loan_amount: 0,
    pending_rewards: D128::zero(),
  };
  contract.state = mock_state;
  contract.compute_borrower_interest(&mut liability1);
  let liability2 = BorrowerInfo {
    interest_index: D128::one(),
    reward_index: D128::zero(),
    loan_amount: 0,
    pending_rewards: D128::zero(),
  };
  assert_eq!(liability1, liability2);

  let mock_state2 = State {
    total_liabilities: D128::new(300000 * 100_000_000),
    total_reserves: D128::new(1000 * 100_000_000),
    last_interest_updated: context.context.block_index,
    last_reward_updated: context.context.block_index,
    global_interest_index: D128::new(2 * 100_000_000),
    global_reward_index: D128::zero(),
    anc_emission_rate: D128::zero(),
    prev_stable_coin_total_supply: 0,
    prev_exchange_rate: D128::one(),
  };
  let mut liability3 = BorrowerInfo {
    interest_index: D128::new(4 * 100_000_000),
    reward_index: D128::zero(),
    loan_amount: 80,
    pending_rewards: D128::zero(),
  };
  contract.state = mock_state2;
  contract.compute_borrower_interest(&mut liability3);
  let liability4 = BorrowerInfo {
    interest_index: D128::new(2 * 100_000_000),
    reward_index: D128::zero(),
    loan_amount: 40,
    pending_rewards: D128::zero(),
  };
  assert_eq!(liability3, liability4);
}

#[test]
fn proper_compute_interest() {
  let (mut context, mut contract) = setup_contract();

  let mock_config = Config {
    owner_id: AccountId::from("owner"),
    stable_coin_contract: AccountId::from("stable_coin"),
    max_borrow_factor: D128::one(),
    overseer_contract: AccountId::from("overseer"),
  };

  let mut mock_state = State {
    total_liabilities: D128::new(1000000u128 * 100_000_000),
    total_reserves: D128::zero(),
    last_interest_updated: context.context.block_index,
    last_reward_updated: context.context.block_index,
    global_interest_index: D128::one(),
    global_reward_index: D128::zero(),
    anc_emission_rate: D128::one(),
    prev_stable_coin_total_supply: 0,
    prev_exchange_rate: D128::one(),
  };
  contract.state = mock_state;

  let mock_deposit_amount = Some(1000u128);

  contract.compute_interest(context.context.block_index, mock_deposit_amount);

  assert_eq!(
    mock_state,
    State {
      total_liabilities: D128::new(1000000u128 * 100_000_000),
      total_reserves: D128::zero(),
      last_interest_updated: context.context.block_index,
      last_reward_updated: context.context.block_index,
      global_interest_index: D128::one(),
      global_reward_index: D128::zero(),
      anc_emission_rate: D128::one(),
      prev_stable_coin_total_supply: 0,
      prev_exchange_rate: D128::one(),
    }
  );

  // TODO: how to resolve cross-contract call?
  // testing_env!(context
  //   .block_index(context.context.block_index + 100)
  //   .build());

  // contract.compute_interest(context.context.block_index, mock_deposit_amount);

  // assert_eq!(
  //   mock_state,
  //   State {
  //     total_liabilities: D128::new(2000000u128 * 100_000_000),
  //     total_reserves: D128::zero(),
  //     last_interest_updated: context.context.block_index,
  //     last_reward_updated: context.context.block_index - 100,
  //     global_interest_index: D128::new(2u128 * 100_000_000),
  //     global_reward_index: D128::zero(),
  //     anc_emission_rate: D128::one(),
  //     prev_stable_coin_total_supply: 2000000u128,
  //     prev_exchange_rate: D128::ratio(19995, 10000),
  //   }
  // );
}
