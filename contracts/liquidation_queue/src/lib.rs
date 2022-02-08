use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::{ValidAccountId, U64, U128};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, serde_json, assert_one_yocto, BorshStorageKey, AccountId, Balance, PanicOnDefault, PromiseOrValue, Promise};
use math::{D128, DECIMAL};
use utils::{fungible_token_transfer, fungible_token_transfer_call, requester, ext_self};
use assert::*;
use std::convert::TryInto;

mod api;
mod assert;
mod internal;
mod math;
mod owner;
mod state;
mod token_receiver;
mod utils;
mod views;

const SECOND_TO_NANO: u64 = 1_000_000_000;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Indexer,
    Bids,
    EpochScaleSum,
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
    pub premium_slot: u8,
    pub bidder: AccountId,
    // amount of USDT (decimal: 6)
    pub amount: U128,
    pub product_snapshot: D128,
    pub sum_snapshot: D128,
    pub pending_liquidated_collateral: U128,
    // unit: seconds
    pub wait_end: Option<U64>,
    pub epoch_snapshot: U128,
    pub scale_snapshot: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct CollateralInfo {
    pub bnear_contract: AccountId,
    pub bid_threshold: U128,
    pub max_slot: u8,
    pub premium_rate_per_slot: D128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub owner: AccountId,
    pub stable_coin_contract: AccountId,
    pub requester_contract: AccountId,
    pub oracle_payment_token: AccountId,
    pub overseer_contract: AccountId,
    pub custody_contract: AccountId,
    pub safe_ratio: D128,
    pub bid_fee: D128,
    pub liquidator_fee: D128,
    pub liquidation_threshold: Balance,
    // unit: seconds
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
    // premium_slot, epoch, scale => sum
    epoch_scale_sum: LookupMap<(u8, U128, U128), D128>,
    bid_idx: U128,
    total_bids: U128,
    last_price_response: PriceResponse,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner: ValidAccountId,
        stable_coin_contract: ValidAccountId,
        requester_contract: ValidAccountId,
        oracle_payment_token: ValidAccountId,
        overseer_contract: ValidAccountId,
        custody_contract: ValidAccountId,
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
                stable_coin_contract: stable_coin_contract.into(),
                requester_contract: requester_contract.into(),
                oracle_payment_token: oracle_payment_token.into(),
                overseer_contract: overseer_contract.into(),
                custody_contract: custody_contract.into(),
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
            epoch_scale_sum: LookupMap::new(StorageKeys::EpochScaleSum),
            bid_idx: U128(1),
            total_bids: U128(0),
            last_price_response: PriceResponse{price: D128::one(), last_updated_at: env::block_timestamp()},
        };
        // Updates initial price
        instance.internal_update_price_response();

        instance
    }

    /// After bids are submitted, need to execute the activation after wait_period expires
    /// Bids are not used for liquidations until activated
    pub fn activate_bids(&mut self, bids_idx: Option<Vec<U128>>) {
        let bidder: AccountId = env::predecessor_account_id();
        let mut available_bids: U128 = self.total_bids;

        let bids: Vec<Bid> = if let Some(bids_idx) = &bids_idx {
            bids_idx
                .iter()
                .map(|idx| self.internal_read_bid(*idx))
                .collect::<Vec<Bid>>()
        } else {
            self.internal_read_bids_by_user(&bidder, None, None)
                .into_iter()
                .filter(|bid| bid.wait_end.is_some())
                .collect::<Vec<Bid>>()
        };

        let mut total_activated_amount: U128 = U128(0);
        for mut bid in bids.into_iter() {
            if bid.bidder != bidder {
                panic!("unauthorized");
            }
            let mut bid_pool: BidPool = self.interanl_read_bid_pool(bid.premium_slot)
                .expect("No bids with the specified information exist");
            
            let amount_to_activate: U128 = bid.amount;

            // assert that the bid is inactive and wait period has expired
            if let Err(err_msg) = 
                assert_activate_status(&bid, available_bids, self.config.collateral_info.bid_threshold)
            {
                if bids_idx.is_some() {
                    // if the user provided the idx to activate, we should return error to notify the user
                    panic!("{}", err_msg);
                } else {
                    // otherwise just skip this bid
                    continue;
                }
            }

            // update bid and bid pool, add new share and pool indexes to bid
            process_bid_activation(&mut bid, &mut bid_pool, amount_to_activate);

            // save to storage
            self.internal_store_bid(bid.idx, &bid);
            self.internal_store_bid_pool(bid.premium_slot, &bid_pool);

            total_activated_amount = (total_activated_amount.0 + amount_to_activate.0).into();
            available_bids = (available_bids.0 + amount_to_activate.0).into();
        }

        self.total_bids = available_bids;
    }

    /// Bid owners can withdraw the ramaning bid amount at any time
    #[payable]
    pub fn retract_bid(&mut self, bid_idx: U128, amount: Option<U128>) {
        assert_one_yocto();
        self.internal_update_price_response();
        
        let bidder: AccountId = env::predecessor_account_id();
        let mut bid: Bid = self.internal_read_bid(bid_idx);

        assert_eq!(bid.bidder, bidder, "unauthorized");

        // check if bid is active or waiting
        let withdraw_amount: U128 = if bid.wait_end.is_some() {
            // waiting bid amount can be withdrawn without restriction
            let waiting_withdraw_amount: U128 = assert_withdraw_amount(amount, bid.amount);
            if waiting_withdraw_amount.0 == bid.amount.0 {
                self.internal_remove_bid(bid.idx);
            } else {
                bid.amount = (bid.amount.0 - waiting_withdraw_amount.0).into();
                self.internal_store_bid(bid.idx, &bid);
            }

            waiting_withdraw_amount
        } else {
            let available_bids: U128 = self.total_bids;
            let mut bid_pool: BidPool =
                self.interanl_read_bid_pool(bid.premium_slot)
                    .expect("No bid pool with the specified information exist");
            
            // calculate spent and reward until this moment
            let (withdrawable_amount, residue_bid): (U128, D128) = 
                self.internal_calculate_remaining_bid(&bid, &bid_pool);
            let (liquidated_callateral, residue_collateral): (U128, D128) = 
                self.internal_calculate_liquidated_collateral(&bid);
            
            // accumulate pending reward to be claimed later
            bid.pending_liquidated_collateral = (bid.pending_liquidated_collateral.0 + liquidated_callateral.0).into();

            // stack residues, will give it to next claimer if it becomes bigger than 1.0
            bid_pool.residue_collateral = bid_pool.residue_collateral + residue_collateral;
            bid_pool.residue_bid = bid_pool.residue_bid + residue_bid;

            // check requested amount
            let withdraw_amount: U128 = assert_withdraw_amount(amount, withdrawable_amount);

            // remove or update bid
            if withdraw_amount.0 == withdrawable_amount.0 && bid.pending_liquidated_collateral.0 == 0 {
                self.internal_remove_bid(bid.idx);
            } else {
                self.internal_store_bid(
                    bid.idx,
                    &Bid {
                        amount: (withdrawable_amount.0 - withdraw_amount.0).into(),
                        product_snapshot: bid_pool.product_snapshot,
                        sum_snapshot: bid_pool.sum_snapshot,
                        scale_snapshot: bid_pool.current_scale,
                        ..bid
                    },
                );
            }

            // update available bid amount
            bid_pool.total_bid_amount = (bid_pool.total_bid_amount.0 - withdraw_amount.0).into();

            // claim residue bids if it is bigger than 1.0
            let refund_amount: u128 = withdraw_amount.0 + self.internal_claim_bid_residue(&mut bid_pool);

            self.internal_store_bid_pool(
                bid.premium_slot,
                &bid_pool,
            );
            self.total_bids = (available_bids.0 - withdraw_amount.0).into();

            refund_amount.into()
        };

        fungible_token_transfer(self.config.stable_coin_contract.clone(), bidder, withdraw_amount.0);
    }

    /// Bid owner can claim their share of the liquidated collateral until the
    /// bid is consumed     
    pub fn claim_liquidations(&mut self, bids_idx: Option<Vec<U128>>) {
        let bidder: AccountId = env::predecessor_account_id();

        let bids: Vec<Bid> = if let Some(bids_idx) = bids_idx {
            bids_idx
                .iter()
                .map(|idx| self.internal_read_bid(*idx))
                .collect::<Vec<Bid>>()
        } else {
            self.internal_read_bids_by_user(&bidder, None, None)
        };

        let mut claim_amount: u128 = 0;
        for bid in bids.into_iter() {
            assert_eq!(bid.bidder, bidder, "unauthorized");

            if bid.wait_end.is_some() {
                // bid not activated
                continue;
            }

            let mut bid_pool: BidPool =
                self.interanl_read_bid_pool(bid.premium_slot)
                    .expect("No bid pool with the specified information exist");
            
            // calculate remaining bid amount
            let (remaining_bid, residue_bid) : (U128, D128) =
                self.internal_calculate_remaining_bid(&bid, &bid_pool);
            
            // calculate liquidated collateral
            let (liquidated_collateral, residue_collateral) : (U128, D128) =
                self.internal_calculate_liquidated_collateral(&bid);
            
            // keep residues
            bid_pool.residue_collateral = bid_pool.residue_collateral + residue_collateral;
            bid_pool.residue_bid = bid_pool.residue_bid + residue_bid;

            // get claimable amount
            claim_amount += bid.pending_liquidated_collateral.0
                + liquidated_collateral.0
                + self.internal_claim_col_residue(&mut bid_pool);
            
            // store bid_pool to update residue
            self.internal_store_bid_pool(bid.premium_slot, &bid_pool);

            // check if bid has been consumed, include 1 for rounding
            if remaining_bid.0 <= 1 {
                self.internal_remove_bid(bid.idx);
            } else {
                self.internal_store_bid(
                    bid.idx,
                    &Bid {
                        amount: remaining_bid,
                        product_snapshot: bid_pool.product_snapshot,
                        sum_snapshot: bid_pool.sum_snapshot,
                        scale_snapshot: bid_pool.current_scale,
                        pending_liquidated_collateral: U128(0),
                        ..bid
                    }
                );
            }
        }
        if claim_amount != 0 {
            fungible_token_transfer(
                self.config.collateral_info.bnear_contract.clone(), 
                bidder, 
                claim_amount
            );
        }
    }
}

fn process_bid_activation(bid: &mut Bid, bid_pool: &mut BidPool, amount: U128) {
    bid.product_snapshot = bid_pool.product_snapshot;
    bid.sum_snapshot = bid_pool.sum_snapshot;
    bid.wait_end = None;
    bid.scale_snapshot = bid_pool.current_scale;
    bid.epoch_snapshot = bid_pool.current_epoch;

    bid_pool.total_bid_amount = (bid_pool.total_bid_amount.0 + amount.0).into();
}