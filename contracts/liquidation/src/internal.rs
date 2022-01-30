use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "This method can only be called by {}",
            self.owner
        );
    }

    pub(crate) fn internal_get_bid(&self, bidder: &AccountId) -> Bid {
        self.bids.get(bidder).expect("No bids with the specified information exist")
    }

    pub(crate) fn internal_remove_bid(&mut self, bidder: &AccountId) {
        self.bids.remove(bidder);
    }

    pub(crate) fn internal_store_bid(&mut self, bidder: &AccountId, bid: Bid) {
        self.bids.insert(bidder, &bid);
    }

    /// updates price response at every function call
    pub(crate) fn internal_ping(&mut self) {
        self.internal_update_price_response(self.requester_contract.clone());
    }

    pub(crate) fn internal_execute_bid(
        &mut self,
        liquidator: AccountId,
        repay_address: AccountId,
        fee_address: AccountId,
        amount: U128,   // amount of bNEAR (decimal: 24)
    ) {
        self.internal_ping();
        let bid: Bid = self.internal_get_bid(&liquidator);

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
        let bid_fee: Balance = self.bid_fee.mul_int(required_stable);
        // decimal: 6
        let repay_amount: Balance = required_stable - bid_fee;

        fungible_token_transfer(self.bnear_contract.clone(), liquidator, amount.0)
            .and(fungible_token_transfer(self.stable_coin_contract.clone(), repay_address, repay_amount));
        
        if bid_fee != 0 {
            fungible_token_transfer(self.stable_coin_contract.clone(), fee_address, bid_fee);
        }
    }
}