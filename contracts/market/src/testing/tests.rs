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
    AccountId::from("collector"),
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
fn proper_initialization() {
  let (_, contract) = setup_contract();

  assert_eq!(AccountId::from("owner"), contract.config.owner_id);
  assert_eq!(D128::zero(), contract.config.max_borrow_factor);
  assert_eq!(
    AccountId::from("stable_coin"),
    contract.config.stable_coin_contract
  );
  assert_eq!(
    AccountId::from("overseer"),
    contract.config.overseer_contract
  );
  assert_eq!(D128::zero(), contract.state.anc_emission_rate);
  assert_eq!(
    D128::new_exp(1, -1),
    contract.interest_model_config.base_rate
  );
  assert_eq!(
    D128::new_exp(1, -1),
    contract.interest_model_config.interest_multiplier
  );
  assert_eq!(
    D128::new_exp(100, 0),
    contract.distribution_model_config.emission_cap
  );
  assert_eq!(
    D128::new_exp(10, 0),
    contract.distribution_model_config.emission_floor
  );
  assert_eq!(
    D128::new_exp(11, -1),
    contract.distribution_model_config.increment_multiplier
  );
  assert_eq!(
    D128::new_exp(9, -1),
    contract.distribution_model_config.decrement_multiplier
  );
}

#[test]
fn update_config() {
  let (_, mut contract) = setup_contract();

  contract.update_config(
    None,
    Some(AccountId::from("stable_coin1")),
    Some(D128::one()),
    Some(AccountId::from("overseer1")),
    Some(AccountId::from("collector1")),
  );

  assert_eq!(D128::one(), contract.config.max_borrow_factor);
  assert_eq!(
    AccountId::from("stable_coin1"),
    contract.config.stable_coin_contract
  );
  assert_eq!(
    AccountId::from("overseer1"),
    contract.config.overseer_contract
  );
  assert_eq!(
    AccountId::from("collector1"),
    contract.config.collector_contract
  );

  contract.update_config(Some(AccountId::from("owner1")), None, None, None, None);

  assert_eq!(AccountId::from("owner1"), contract.config.owner_id);
}

#[test]
#[should_panic(expected = "Can only be called by the owner")]
fn assert_owner() {
  let (_, mut contract) = setup_contract();

  contract.update_config(Some(AccountId::from("owner1")), None, None, None, None);
  contract.update_config(Some(AccountId::from("owner2")), None, None, None, None);
}

#[test]
fn update_interest_model_config() {}

#[test]
fn update_distribution_model_config() {}

#[test]
fn proper_borrow_rate() {
  let (_, contract) = setup_contract();

  let rate = contract.get_borrow_rate(
    1000000u128,
    D128::new_exp(500000, 0),
    D128::new_exp(100000, 0),
  );
  assert_eq!(D128::ratio(19, 140), rate);

  let rate = contract.get_borrow_rate(0u128, D128::zero(), D128::zero());
  assert_eq!(D128::new_exp(1, -1), rate);
}

#[test]
fn proper_emission_rate() {
  let (_, contract) = setup_contract();

  // high = 8.75
  // low = 6.75
  // no changes
  let rate = contract.get_emission_rate(
    D128::new_exp(7, -2),
    D128::new_exp(1, -1),
    D128::new_exp(5, -2),
    D128::new_exp(99, 0),
  );
  assert_eq!(D128::new_exp(99, 0), rate);

  // increment
  let rate = contract.get_emission_rate(
    D128::new_exp(5, -2),
    D128::new_exp(1, -1),
    D128::new_exp(5, -2),
    D128::new_exp(80, 0),
  );
  assert_eq!(D128::new_exp(88, 0), rate);

  // cap
  let rate = contract.get_emission_rate(
    D128::new_exp(5, -2),
    D128::new_exp(1, -1),
    D128::new_exp(5, -2),
    D128::new_exp(99, 0),
  );
  assert_eq!(D128::new_exp(100, 0), rate);

  // decrement
  let rate = contract.get_emission_rate(
    D128::new_exp(9, -2),
    D128::new_exp(1, -1),
    D128::new_exp(5, -2),
    D128::new_exp(99, 0),
  );
  assert_eq!(D128::new_exp(891, -1), rate);

  // floor
  let rate = contract.get_emission_rate(
    D128::new_exp(9, -2),
    D128::new_exp(1, -1),
    D128::new_exp(5, -2),
    D128::new_exp(11, 0),
  );
  assert_eq!(D128::new_exp(10, 0), rate);
}
