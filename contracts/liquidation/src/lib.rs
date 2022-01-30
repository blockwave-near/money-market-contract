use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::collections::{LookupMap};
use near_sdk::{env, near_bindgen, serde_json, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use math::{D128};
use utils::{fungible_token_transfer};

mod internal;
mod math;
mod owner;
mod token_receiver;
mod utils;
mod views;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    // amount of USDT (decimal: 6)
    pub amount: U128,
    pub premium_rate: D128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceResponse {
    pub price: D128,
    pub last_updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BnearReceiverPayload {
    pub liquidator: AccountId,
    pub repay_address: Option<AccountId>,
    pub fee_address: Option<AccountId>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct StableReceiverPayload {
    pub premium_rate: D128,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    bnear_contract: AccountId,
    stable_coin_contract: AccountId,
    requester_contract: AccountId,
    oracle_payment_token: AccountId,
    safe_ratio: D128,
    bid_fee: D128,
    max_premium_rate: D128,
    liquidation_threshold: Balance,
    bids: LookupMap<AccountId, Bid>,
    last_price_response: PriceResponse,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner: AccountId,
        bnear_contract: AccountId,
        stable_coin_contract: AccountId,
        requester_contract: AccountId,
        oracle_payment_token: AccountId,
        safe_ratio: D128,
        bid_fee: D128,
        max_premium_rate: D128,
        liquidation_threshold: Balance,
    ) -> Self {
        let mut instance = Self{
            owner,
            bnear_contract,
            stable_coin_contract,
            requester_contract,
            oracle_payment_token,
            safe_ratio,
            bid_fee,
            max_premium_rate,
            liquidation_threshold,
            bids: LookupMap::new(b"b".to_vec()),
            last_price_response: PriceResponse{price: D128::one(), last_updated_at: env::block_timestamp()},
        };
        // Updates initial price
        instance.internal_ping();

        instance
    }

    pub fn retract_bid(&mut self, amount: Option<U128>) {
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

        fungible_token_transfer(self.stable_coin_contract.clone(), bidder, amount.0);
    }
}
