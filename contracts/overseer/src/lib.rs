use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight,
    BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
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
        oracle_contrract: AccountId,
        market_contract: AccountId,
        liquidation_contract: AccountId,
        collector_contract: AccountId,
        target_deposit_rate: D128,
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
            oracle_contrract,
            market_contract,
            liquidation_contract,
            collector_contract,
            target_deposit_rate,
            oracle_payment_token,
            requester_contract,
        };

        let state = State {
            last_price_response: PriceResponse {
                price: D128::one(),
                last_updated_at: env::block_timestamp(),
            },
        };

        let collection = Collection {
            white_list_elem_map: LookupMap::new(StorageKey::WhitelistElem),
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
    pub fn execute_epoch_operations(
        &mut self,
        deposit_rate: D128,
        target_deposit_rate: D128,
        threshold_deposit_rate: D128,
        distributed_intereset: U128,
    ) {
        assert_one_yocto();
        self.internal_update_price_response();
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
