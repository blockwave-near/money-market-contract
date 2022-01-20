use near_decimal::d128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, Balance, PanicOnDefault};
use uint::construct_uint;

mod internal;
mod owner;
#[cfg(test)]
mod tests;
mod view;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    emission_cap: d128,
    emission_floor: d128,
    increment_multiplier: d128,
    decrement_multiplier: d128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        emission_cap: d128,
        emission_floor: d128,
        increment_multiplier: d128,
        decrement_multiplier: d128,
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        Self {
            owner_id,
            emission_cap: emission_cap,
            emission_floor: emission_floor,
            increment_multiplier: increment_multiplier,
            decrement_multiplier: decrement_multiplier,
        }
    }
}
