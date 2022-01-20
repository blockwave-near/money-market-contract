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

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    base_rate: d128,
    interest_multiplier: d128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, base_rate: d128, interest_multiplier: d128) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );

        Self {
            owner_id,
            base_rate: base_rate,
            interest_multiplier: interest_multiplier,
        }
    }
}
