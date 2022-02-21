use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight,
    BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult, Timestamp,
};

use uint::construct_uint;

use crate::math::{D128, DECIMAL};
use crate::state::{Collection, Config, State, WhitelistElem};
use crate::tokens::{Token, Tokens, TokensMath};
use crate::utils::{
    ext_custody_bnear, ext_market, ext_self, fungible_token, fungible_token_transfer_call,
    requester,
};

mod collateral;
mod internal;
mod math;
mod owner;
mod state;
#[cfg(test)]
mod testing;
mod tokens;
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
    Collateral,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceResponse {
    pub price: D128,
    pub last_updated_at: u64,
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
        stable_coin_contract: AccountId,
        oracle_contrract: AccountId,
        market_contract: AccountId,
        liquidation_contract: AccountId,
        collector_contract: AccountId,
        epoch_period: BlockHeight,
        threshold_deposit_rate: D128,
        target_deposit_rate: D128,
        buffer_distribution_factor: D128,
        anc_purchase_factor: D128,
        oracle_payment_token: AccountId,
        requester_contract: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );

        let config = Config {
            owner_id,
            stable_coin_contract,
            oracle_contrract,
            market_contract,
            liquidation_contract,
            collector_contract,
            epoch_period,
            threshold_deposit_rate,
            target_deposit_rate,
            buffer_distribution_factor,
            anc_purchase_factor,
            oracle_payment_token,
            requester_contract,
        };

        let state = State {
            deposit_rate: D128::zero(),
            prev_stable_coin_total_supply: 0,
            last_executed_height: 0u64,
            prev_exchange_rate: D128::one(),
            prev_interest_buffer: 0,
            last_price_response: PriceResponse {
                price: D128::one(),
                last_updated_at: env::block_timestamp(),
            },
        };

        let collection = Collection {
            white_list_elem_map: UnorderedMap::new(StorageKey::WhitelistElem),
            collateral_map: LookupMap::new(StorageKey::Collateral),
        };

        let mut instance = Self {
            config,
            state,
            collection,
        };

        instance.internal_update_price_response();

        instance
    }

    #[payable]
    pub fn register_whitelist(
        &mut self,
        name: String,
        symbol: String,
        collateral_token: AccountId,
        custody_contract: AccountId,
        max_ltv: D128,
    ) {
        assert_one_yocto();
        self.assert_owner();
        self.internal_update_price_response();

        self.add_white_list_elem_map(
            &collateral_token,
            &WhitelistElem {
                name: name.to_string(),
                symbol: symbol.to_string(),
                custody_contract,
                max_ltv,
            },
        );
    }

    #[payable]
    pub fn update_whitelist(
        &mut self,
        collateral_token: AccountId,
        custody_contract: Option<AccountId>,
        max_ltv: Option<D128>,
    ) {
        assert_one_yocto();
        self.assert_owner();
        self.internal_update_price_response();
        let mut white_list_elem: WhitelistElem = self.get_white_list_elem_map(&collateral_token);

        if let Some(custody_contract) = custody_contract {
            white_list_elem.custody_contract = custody_contract;
        }

        if let Some(max_ltv) = max_ltv {
            white_list_elem.max_ltv = max_ltv;
        }

        self.add_white_list_elem_map(&collateral_token, &white_list_elem);
    }

    #[payable]
    pub fn execute_epoch_operations(&mut self) {
        assert_one_yocto();
        self.internal_update_price_response();

        if env::block_index() < self.state.last_executed_height + self.config.epoch_period {
            env::panic("Epoch Not Passed".as_bytes());
        }

        let block_height = env::block_index();
        let blocks = block_height - self.state.last_executed_height;

        let interest_buffer = env::account_balance();

        ext_market::get_epoch_state(
            Some(block_height),
            None,
            &self.config.market_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        )
        .then(ext_self::callback_execute_epoch_operations(
            blocks,
            interest_buffer,
            &env::current_account_id(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ));
    }

    #[payable]
    pub fn update_epoch_state(&mut self, intereset_buffer: U128, distributed_intereset: U128) {
        assert_one_yocto();
        self.internal_update_price_response();
        self.assert_owner();

        let block_height = env::block_index();
        let blocks = block_height - self.state.last_executed_height;

        ext_market::get_epoch_state(
            Some(block_height),
            Some(distributed_intereset),
            &self.config.market_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        )
        .then(ext_self::callback_update_epoch_state(
            intereset_buffer,
            distributed_intereset,
            block_height,
            blocks,
            &env::current_account_id(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ));
    }
}
