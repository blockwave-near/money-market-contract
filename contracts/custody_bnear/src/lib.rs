use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, serde_json, AccountId, Balance, BlockHeight,
    BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};

use uint::construct_uint;

use crate::math::D128;
use crate::state::{BAssetInfo, BorrowerInfo, Collection, Config, State};
use crate::utils::{ext_reward, ext_self, fungible_token};

mod collateral;
mod distribution;
mod fungible_token_handler;
mod internal;
mod math;
mod owner;
mod state;
#[cfg(test)]
mod testing;
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
    BorrowerInfo,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    config: Config,
    state: State,
    collection: Collection,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        overseer_contract: AccountId,
        collateral_token: AccountId,
        market_contract: AccountId,
        reward_contract: AccountId,
        liquidation_contract: AccountId,
        stable_coin_contract: AccountId,
        basset_info: BAssetInfo,
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );

        let config = Config {
            owner_id,
            overseer_contract,
            collateral_token,
            market_contract,
            reward_contract,
            liquidation_contract,
            stable_coin_contract,
            basset_info,
        };

        let state = State {};

        let collection = Collection {
            borrower_info_map: LookupMap::new(StorageKey::BorrowerInfo),
        };

        Self {
            config,
            state,
            collection,
        }
    }
}
