use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight,
    BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};

use uint::construct_uint;

use crate::distribution_model::DistributionModelConfig;
// use crate::fraction::Fraction;
use crate::interest_model::InterestModelConfig;
use crate::math::D128;
use crate::state::{BorrowerInfo, Collection, Config, State};
use crate::utils::{ext_overseer, ext_self, fungible_token};

mod borrow;
mod deposit;
mod distribution_model;
mod fraction;
mod fungible_token_handler;
mod interest_model;
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
    interest_model_config: InterestModelConfig,
    distribution_model_config: DistributionModelConfig,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        max_borrow_factor: D128,
        stable_coin_contract: AccountId,
        overseer_contract: AccountId,
        collector_contract: AccountId,

        base_rate: D128,
        interest_multiplier: D128,

        emission_cap: D128,
        emission_floor: D128,
        increment_multiplier: D128,
        decrement_multiplier: D128,
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );

        let config = Config {
            owner_id,
            max_borrow_factor,
            stable_coin_contract,
            overseer_contract,
            collector_contract,
        };

        let state = State {
            anc_emission_rate: D128::one(),
            total_liabilities: D128::zero(),
            total_reserves: D128::zero(),
            last_interest_updated: 0,
            global_interest_index: D128::zero(),
            prev_exchange_rate: D128::one(),
            prev_stable_coin_total_supply: 0,
            last_reward_updated: 0,
            global_reward_index: D128::zero(),
        };

        let collection = Collection {
            borrower_info_map: LookupMap::new(StorageKey::BorrowerInfo),
        };

        let interest_model_config = InterestModelConfig {
            base_rate,
            interest_multiplier,
        };

        let distribution_model_config = DistributionModelConfig {
            emission_cap,
            emission_floor,
            increment_multiplier,
            decrement_multiplier,
        };

        Self {
            config,
            state,
            collection,
            interest_model_config,
            distribution_model_config,
        }
    }

    pub fn execute_epoch_operations(
        &mut self,
        deposit_rate: D128,
        target_deposit_rate: D128,
        threshold_deposit_rate: D128,
        distributed_intereset: U128,
    ) {
        self.assert_overseer();
        fungible_token::ft_total_supply(
            &self.config.stable_coin_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        )
        .then(ext_self::callback_execute_epoch_operations(
            deposit_rate,
            target_deposit_rate,
            threshold_deposit_rate,
            distributed_intereset,
            &env::current_account_id(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ));
    }

    pub fn get_epoch_state(
        &mut self,
        block_height: Option<BlockHeight>,
        distributed_intereset: Option<U128>,
    ) -> Promise {
        let distributed_intereset = distributed_intereset.unwrap_or(U128::from(0));
        let balance: Balance = env::account_balance() - distributed_intereset.0;

        ext_overseer::get_target_deposit_rate(
            &self.config.overseer_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        )
        .and(fungible_token::ft_total_supply(
            &self.config.stable_coin_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ))
        .then(ext_self::callback_get_epoch_state(
            block_height,
            balance,
            distributed_intereset,
            &env::current_account_id(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ))
    }
}
