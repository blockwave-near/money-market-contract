use near_decimal::d128;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;

use super::*;

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests

fn setup_contract() -> (VMContextBuilder, Contract) {
  let mut context = VMContextBuilder::new();
  testing_env!(context
    .predecessor_account_id(accounts(0))
    .attached_deposit(1)
    .build());
  let contract = Contract::new(accounts(0), d128!(0.1), d128!(0.1));
  (context, contract)
}

// TESTS HERE
#[test]
fn proper_initialization() {
  let (_, contract) = setup_contract();
  let (owner_id, base_rate, interest_multiplier) = contract.get_config();
  assert_eq!(accounts(0), owner_id);
  assert_eq!(d128!(0.1), base_rate);
  assert_eq!(d128!(0.1), interest_multiplier);
}

#[test]
fn update_config() {
  let (_, mut contract) = setup_contract();

  contract.update_config(Some(accounts(1)), None, None);

  let (owner_id, base_rate, interest_multiplier) = contract.get_config();
  assert_eq!(accounts(1), owner_id);
  assert_eq!(d128!(0.1), base_rate);
  assert_eq!(d128!(0.1), interest_multiplier);
}

#[test]
#[should_panic(expected = "Can only be called by the owner")]
fn update_config_panic() {
  let (_, mut contract) = setup_contract();

  contract.update_config(Some(accounts(1)), None, None);
  contract.update_config(Some(accounts(0)), None, None);
}

#[test]
fn proper_borrow_rate() {
  let (_, contract) = setup_contract();

  let rate = contract.get_borrow_rate(1000000u128, U128::from(500000u128), U128::from(100000u128));
  assert_eq!("0.1357142857142857142857142857142857", rate.to_string());

  let rate = contract.get_borrow_rate(0u128, U128::from(0u128), U128::from(0u128));
  assert_eq!("0.1", rate.to_string());
}
