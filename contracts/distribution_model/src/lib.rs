use near_decimal::d128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    emission_cap: d128,
    emission_floor: d128,
    increment_multiplier: d128,
    decrement_multiplier: d128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        emission_cap: d128,
        emission_floor: d128,
        increment_multiplier: d128,
        decrement_multiplier: d128,
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        Self {
            emission_cap,
            emission_floor,
            increment_multiplier,
            decrement_multiplier,
        }
    }

    pub fn get_emission_rate(
        &self,
        deposit_rate: d128,
        target_deposit_rate: d128,
        threshold_deposit_rate: d128,
        current_emission_rate: d128,
    ) -> d128 {
        let half_dec = d128!(1) + d128!(1);
        let mid_rate = (threshold_deposit_rate + target_deposit_rate) / half_dec;
        let high_trigger = (mid_rate + target_deposit_rate) / half_dec;
        let low_trigger = (mid_rate + threshold_deposit_rate) / half_dec;

        let emission_rate = if deposit_rate < low_trigger {
            current_emission_rate * self.increment_multiplier
        } else if deposit_rate > high_trigger {
            current_emission_rate * self.decrement_multiplier
        } else {
            current_emission_rate
        };

        let emission_rate = if emission_rate > self.emission_cap {
            self.emission_cap
        } else if emission_rate < self.emission_floor {
            self.emission_floor
        } else {
            emission_rate
        };
        emission_rate
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
