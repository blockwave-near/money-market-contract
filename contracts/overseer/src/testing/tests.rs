use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};

use crate::*;

fn setup_contract() -> (VMContextBuilder, Contract) {
  let mut context = VMContextBuilder::new();
  testing_env!(context
    .predecessor_account_id(accounts(0))
    .attached_deposit(1)
    .build());
  let contract = Contract::new(accounts(0).into());
  (context, contract)
}
