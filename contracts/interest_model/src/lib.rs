use near_decimal::d128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    base_rate: d128,
    interest_multiplier: d128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(base_rate: d128, interest_multiplier: d128) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        Self {
            base_rate,
            interest_multiplier,
        }
    }

    pub fn get_borrow_rate(
        &self,
        market_balance: Balance,
        total_liabilities: d128,
        total_reserves: d128,
    ) -> d128 {
        let total_value_in_market = d128!(market_balance) + total_liabilities - total_reserves;
        let utilization_ratio = if total_value_in_market.is_zero() {
            d128!(0)
        } else {
            total_liabilities / total_value_in_market
        };

        let rate = utilization_ratio * self.interest_multiplier + self.base_rate;
        rate
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
