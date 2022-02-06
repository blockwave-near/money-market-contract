use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::{ValidAccountId, U64, U128};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::{env, near_bindgen, serde_json, assert_one_yocto, BorshStorageKey, AccountId, Balance, PanicOnDefault, PromiseOrValue, Promise};
use math::{D128, DECIMAL};
use utils::{fungible_token_transfer, fungible_token_transfer_call, requester, ext_self};
use std::convert::TryInto;

mod internal;
mod math;
mod owner;
mod state;
mod token_receiver;
mod utils;
mod views;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Indexer,
    Bids,
    Account { account_hash: Vec<u8> },
    BidPools,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceResponse {
    pub price: D128,
    pub last_updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BidPool {
    pub sum_snapshot: D128,
    pub product_snapshot: D128,
    pub total_bid_amount: U128,
    pub premium_rate: D128,
    pub current_epoch: U128,
    pub current_scale: U128,
    pub residue_collateral: D128,
    pub residue_bid: D128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub idx: U128,
    pub collateral_token: AccountId,
    pub premium_slot: u8,
    pub bidder: AccountId,
    // amount of USDT (decimal: 6)
    pub amount: U128,
    pub product_snapshot: D128,
    pub sum_snapshot: D128,
    pub pending_liquidated_collateral: U128,
    pub wait_end: Option<U64>,
    pub epoch_snapshot: U128,
    pub scale_snapshot: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct CollateralInfo {
    pub bid_threshold: U128,
    pub max_slot: u8,
    pub premium_rate_per_slot: D128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub owner: AccountId,
    pub bnear_contract: AccountId,
    pub stable_coin_contract: AccountId,
    pub requester_contract: AccountId,
    pub oracle_payment_token: AccountId,
    pub overseer_contract: AccountId,
    pub safe_ratio: D128,
    pub bid_fee: D128,
    pub liquidator_fee: D128,
    pub liquidation_threshold: Balance,
    pub waiting_period: u64,
    pub collateral_info: CollateralInfo,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    config: Config,
    bids_indexer_by_user: LookupMap<AccountId, UnorderedSet<U128>>,
    bids: LookupMap<U128, Bid>,
    bid_pools: UnorderedMap<u8, BidPool>,
    last_price_response: PriceResponse,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner: ValidAccountId,
        bnear_contract: ValidAccountId,
        stable_coin_contract: ValidAccountId,
        requester_contract: ValidAccountId,
        oracle_payment_token: ValidAccountId,
        overseer_contract: ValidAccountId,
        safe_ratio: D128,
        bid_fee: D128,
        liquidator_fee: D128,
        liquidation_threshold: Balance,
        waiting_period: U64,
        collateral_info: CollateralInfo,
    ) -> Self {
        let mut instance = Self{
            config: Config {
                owner: owner.into(),
                bnear_contract: bnear_contract.into(),
                stable_coin_contract: stable_coin_contract.into(),
                requester_contract: requester_contract.into(),
                oracle_payment_token: oracle_payment_token.into(),
                overseer_contract: overseer_contract.into(),
                safe_ratio,
                bid_fee,
                liquidator_fee,
                liquidation_threshold,
                waiting_period: waiting_period.into(),
                collateral_info
            },
            bids_indexer_by_user: LookupMap::new(StorageKeys::Indexer),
            bids: LookupMap::new(StorageKeys::Bids),
            bid_pools: UnorderedMap::new(StorageKeys::BidPools),
            last_price_response: PriceResponse{price: D128::one(), last_updated_at: env::block_timestamp()},
        };
        // Updates initial price
        instance.internal_update_price_response();

        instance
    }

    #[payable]
    pub fn retract_bid(&mut self, amount: Option<U128>) {
        assert_one_yocto();
        
        self.internal_update_price_response();
        
        let bidder: AccountId = env::predecessor_account_id();

        let bid: Bid = self.internal_get_bid(&bidder).expect("No bids with the specified information exist");

        let amount: U128 = amount.unwrap_or(bid.amount);

        if amount.0 > bid.amount.0 {
            panic!("Retract amount cannot exceed bid balance: {}", bid.amount.0);
        }

        if amount.0 == bid.amount.0 {
            self.internal_remove_bid(&bidder);
        } else {
            self.internal_store_bid(
                &bidder,
                Bid {
                    amount: (bid.amount.0 - amount.0).into(),
                    ..bid
                }
            );
        }

        fungible_token_transfer(self.config.stable_coin_contract.clone(), bidder, amount.0);
    }
}
