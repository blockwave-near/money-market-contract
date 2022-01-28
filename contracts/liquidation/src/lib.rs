use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::U128;
use near_sdk::collections::{LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};
use math::{D128};

mod internal;
mod math;
mod owner;
mod token_receiver;
mod utils;
mod views;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub amount: U128,
    pub premium_rate: D128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceResponse {
    pub rate: D128,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeConstraints {
    pub block_time: u64,
    pub valid_timeframe: u64,
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    oracle_contract: AccountId,
    safe_ratio: D128,
    bid_fee: D128,
    max_premium_rate: D128,
    liquidation_threshold: Balance,
    price_timeframe: u64,
    bids: LookupMap<AccountId, Bid>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner: AccountId,
        oracle_contract: AccountId,
        safe_ratio: D128,
        bid_fee: D128,
        max_premium_rate: D128,
        liquidation_threshold: Balance,
        price_timeframe: u64
    ) -> Self {
        Self{
            owner,
            oracle_contract,
            safe_ratio,
            bid_fee,
            max_premium_rate,
            liquidation_threshold,
            price_timeframe,
            bids: LookupMap::new(b"b".to_vec()),
        }
    }

    pub fn execute_bid(
        &self,
        liquidator: AccountId,
        repay_address: AccountId,
        fee_address: AccountId,
        amount: U128,
    ) {
        let bid: Bid = self.internal_get_bid(&liquidator);

        let price: PriceResponse = self.query_price(
            Some(TimeConstraints {
                block_time: env::block_timestamp(),
                valid_timeframe: self.price_timeframe,
            }),
        );

        let collateral_value: Balance = price.rate * amount.0;
        let required_stable: Balance = (D128::one() - std::cmp::min(bid.premium_rate, self.max_premium_rate))
            * collateral_value;
        
        if required_stable > bid.amount.0 {
            panic!("Insufficient Bid Balance. Required: {}", required_stable);
        }

        // Update bid
        if bid.amount.0 == required_stable {
            self.internal_remove_bid(&liquidator);
        } else {
            self.internal_store_bid(
                &liquidator,
                Bid {
                    amount: (bid.amount.0 - required_stable).into(),
                    ..bid
                }
            );
        }

        let bid_fee: Balance = self.bid_fee * required_stable;
        let repay_address: Balance = required_stable - bid_fee;

        // TODO: cross-contract calls
    }
}
