use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};

use crate::*;

fn setup_contract() -> (VMContextBuilder, Contract) {
  let mut context = VMContextBuilder::new();
  testing_env!(context
    .predecessor_account_id(accounts(0))
    .attached_deposit(1)
    .build());
  let contract = Contract::new(
    AccountId::from("owner"),
    AccountId::from("overseer"),
    AccountId::from("collateral"),
    AccountId::from("market"),
    AccountId::from("reward"),
    AccountId::from("liquidation"),
    AccountId::from("stable_coin"),
    BAssetInfo {
      name: String::from("name"),
      symbol: String::from("symbol"),
      decimals: 8,
    },
  );
  (context, contract)
}
