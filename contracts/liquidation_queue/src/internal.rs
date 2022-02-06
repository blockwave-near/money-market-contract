use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.config.owner,
            "This method can only be called by {}",
            self.config.owner
        );
    }

    pub(crate) fn assert_fees(&self, fees: D128) {
        assert!(fees > D128::one(), "The sum of bid_fee and liquidator_fee can not be greater than one");
    }

    /// updates price response at every function call
    pub(crate) fn internal_update_price_response(
        &mut self,
    ) -> Promise {
        requester::get_data_request(
            env::current_account_id().try_into().unwrap(),
            // Near params
            &self.config.requester_contract,
            0,
            3_000_000_000_000,
        ).then(ext_self::callback_get_price_response(
            // Near params
            &env::current_account_id(),
            0,
            30_000_000_000_000,
        ))
    }

    pub(crate) fn internal_create_new_price_request(&self) {
        fungible_token_transfer_call(
            self.config.oracle_payment_token.clone(), 
            self.config.requester_contract.clone(), 
            1_000_000_000_000_000_000_000_000, 
            // query NEAR price
            format!("{{\"sources\": [{{ \"end_point\": \"https://api.coingecko.com/api/v3/simple/price?ids=tether%2Cnear&vs_currencies=usd\", \"source_path\":\"near.usd\"}}], \"tags\":[\"pricing\",\"near\"],  \"challenge_period\":\"120000000000\", \"settlement_time\":\"1\", \"data_type\":{{\"Number\":\"{}\"}}, \"creator\":\"{}\"}}", DECIMAL, env::current_account_id())
        );
    }

    /// callback on transfer stable coin
    pub(crate) fn internal_submit_bid(&mut self, bidder: AccountId, premium_slot: u8, amount: U128) {
        self.internal_update_price_response();
        assert!(self.internal_read_bid(&bidder).is_none(), "User already has bid");
        assert!(premium_rate < self.config.max_premium_rate, "Premium rate cannot exceed the max premium rate");

        // read or create bid_pool, make sure slot is valid
        let mut bid_pool: BidPool =
            self.internal_read_or_create_bid_pool(&collateral_info, premium_slot);

        self.internal_store_bid(
            &bidder,
            Bid {
                amount,
                premium_rate
            }
        );
    }

    /// callback on transfer bnear token
    pub(crate) fn internal_execute_bid(
        &mut self,
        liquidator: AccountId,
        repay_address: AccountId,
        fee_address: AccountId,
        amount: U128,   // amount of bNEAR (decimal: 24)
    ) {
        self.internal_update_price_response();
        let bid: Bid = self.internal_get_bid(&liquidator).expect("No bids with the specified information exist");

        // corresponding collateral bNEAR value in USD (decimal: 6, which is decimal of USDT)
        let collateral_value: Balance = self.last_price_response.price.mul_int(amount.0) / 1_000_000_000_000_000_000;
        // required amount of USDT (decimal: 6)
        let required_stable: Balance = (D128::one() - std::cmp::min(bid.premium_rate, self.max_premium_rate))
            .mul_int(collateral_value);
        
        if required_stable > bid.amount.0 {
            panic!("Insufficient bid balance; Required balance: {}", required_stable);
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

        // decimal: 6
        let bid_fee: Balance = self.config.bid_fee.mul_int(required_stable);
        // decimal: 6
        let repay_amount: Balance = required_stable - bid_fee;

        fungible_token_transfer(self.config.bnear_contract.clone(), liquidator, amount.0)
            .and(fungible_token_transfer(self.config.stable_coin_contract.clone(), repay_address, repay_amount));
        
        if bid_fee != 0 {
            fungible_token_transfer(self.config.stable_coin_contract.clone(), fee_address, bid_fee);
        }
    }
}