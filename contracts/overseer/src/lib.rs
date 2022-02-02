use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight,
    BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};

use uint::construct_uint;

use crate::math::D128;
use crate::state::{Collection, Config, State};
use crate::utils::{ext_self, ext_stable_coin};

mod internal;
mod math;
mod owner;
mod state;
#[cfg(test)]
mod tests;
mod utils;
mod view;

const NO_DEPOSIT: Balance = 0;

const SINGLE_CALL_GAS: Gas = 100_000_000_000_000;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    WhitelistElem,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    config: Config,
    state: State,
    collection: Collection,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );

        let config = Config {
            oracle_contrract: AccountId::from(""),
            market_contract: AccountId::from(""),
            liquidation_contract: AccountId::from(""),
            collector_contract: AccountId::from(""),
        };

        let state = State {};

        let collection = Collection {
            white_list_elem_map: LookupMap::new(StorageKey::WhitelistElem),
        };

        Self {
            owner_id,
            config,
            state,
            collection,
        }
    }

    pub fn execute_epoch_operations(
        &mut self,
        deposit_rate: D128,
        target_deposit_rate: D128,
        threshold_deposit_rate: D128,
        distributed_intereset: U128,
    ) {
        // ext_stable_coin::ft_total_supply(
        //     &self.config.stable_coin_contract,
        //     NO_DEPOSIT,
        //     SINGLE_CALL_GAS,
        // )
        // .then(ext_self::callback_execute_epoch_operations(
        //     deposit_rate,
        //     target_deposit_rate,
        //     threshold_deposit_rate,
        //     distributed_intereset,
        //     &env::current_account_id(),
        //     NO_DEPOSIT,
        //     SINGLE_CALL_GAS,
        // ));
    }
}
