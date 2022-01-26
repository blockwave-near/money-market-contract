use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::U128;
use near_sdk::collections::{LookupMap};
use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, Balance, PanicOnDefault};
use uint::construct_uint;

mod internal;
mod owner;
mod token_receiver;
mod views;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub amount: U128,
    pub premium_rate: Fraction,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceResponse {
    pub rate: Fraction,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeConstraints {
    pub block_time: u64,
    pub valid_timeframe: u64,
}

impl Fraction {
    pub fn mul(&self, num: u128) -> u128 {
        (U256::from(num) * U256::from(self.numerator) 
            / U256::from(self.denominator)).as_u128()
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    oracle_contract: AccountId,
    safe_ratio: Fraction,
    bid_fee: Fraction,
    max_premium_rate: Fraction,
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
        safe_ratio: Fraction,
        bid_fee: Fraction,
        max_premium_rate: Fraction,
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
        let bid: Bid = self.get_bid(&liquidator);

        let price: PriceResponse = self.query_price(
            Some(TimeConstraints {
                block_time: env::block_timestamp(),
                valid_timeframe: self.price_timeframe,
            }),
        );

        let collateral_value: u128 = price.rate.mul(amount.0);
        let premium_rate: Fraction = std::cmp::min(bid.premium_rate, self.max_premium_rate);
        
    }
}
