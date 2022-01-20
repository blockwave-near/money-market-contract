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
  let contract = Contract::new(accounts(0), d128!(100), d128!(10), d128!(1.1), d128!(0.9));
  (context, contract)
}

// // TESTS HERE
#[test]
fn proper_initialization() {
  let (_, contract) = setup_contract();
  let (owner_id, emission_cap, emission_floor, increment_multiplier, decrement_multiplier) =
    contract.get_config();
  assert_eq!(accounts(0), owner_id);
  assert_eq!(d128!(100), emission_cap);
  assert_eq!(d128!(10), emission_floor);
  assert_eq!(d128!(1.1), increment_multiplier);
  assert_eq!(d128!(0.9), decrement_multiplier);
}

#[test]
fn update_config() {
  let (_, mut contract) = setup_contract();

  contract.update_config(Some(accounts(1)), None, None, None, None);

  let (owner_id, emission_cap, emission_floor, increment_multiplier, decrement_multiplier) =
    contract.get_config();
  assert_eq!(accounts(1), owner_id);
  assert_eq!(d128!(100), emission_cap);
  assert_eq!(d128!(10), emission_floor);
  assert_eq!(d128!(1.1), increment_multiplier);
  assert_eq!(d128!(0.9), decrement_multiplier);
}

#[test]
#[should_panic(expected = "Can only be called by the owner")]
fn update_config_panic() {
  let (_, mut contract) = setup_contract();

  contract.update_config(Some(accounts(1)), None, None, None, None);
  contract.update_config(Some(accounts(0)), None, None, None, None);
}

#[test]
fn proper_emission_rate() {
  let (_, contract) = setup_contract();

  // high = 8.75
  // low = 6.75
  // no changes
  let rate = contract.get_emission_rate(d128!(0.07), d128!(0.1), d128!(0.05), d128!(99));
  assert_eq!("99", rate.to_string());

  // increment
  let rate = contract.get_emission_rate(d128!(0.05), d128!(0.1), d128!(0.05), d128!(80));
  assert_eq!("88.0", rate.to_string()); // TODO why "88.0" instead of "88"?

  // cap
  let rate = contract.get_emission_rate(d128!(0.05), d128!(0.1), d128!(0.05), d128!(99));
  assert_eq!("100", rate.to_string());

  // decrement
  let rate = contract.get_emission_rate(d128!(0.09), d128!(0.1), d128!(0.05), d128!(99));
  assert_eq!("89.1", rate.to_string());

  // floor
  let rate = contract.get_emission_rate(d128!(0.09), d128!(0.1), d128!(0.05), d128!(11));
  assert_eq!("10", rate.to_string());
}
