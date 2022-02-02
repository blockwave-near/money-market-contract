use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};

use super::*;

fn setup_contract() -> (VMContextBuilder, Contract) {
  let mut context = VMContextBuilder::new();
  testing_env!(context
    .predecessor_account_id(accounts(0))
    .attached_deposit(1)
    .build());
  let contract = Contract::new(
    accounts(0).into(),
    D128::zero(),
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
