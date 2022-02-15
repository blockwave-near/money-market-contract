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
        AccountId::from("stable_coin"),
        AccountId::from("oracle"),
        AccountId::from("market"),
        AccountId::from("liquidation"),
        AccountId::from("collector"),
        86400u64,
        D128::new_exp(3, -3),
        D128::new_exp(5, -3),
        D128::new_exp(20, -2),
        D128::new_exp(20, -2),
        AccountId::from("oralce_payment_token"),
        AccountId::from("requester"),
    );
    (context, contract)
}
