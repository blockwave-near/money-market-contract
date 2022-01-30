use crate::*;

#[near_bindgen]
impl Contract {
    pub fn update_config(
        &mut self,
        owner: Option<AccountId>,
        requester_contract: Option<AccountId>,
        payment_token: Option<AccountId>,
        safe_ratio: Option<D128>,
        bid_fee: Option<D128>,
        max_premium_rate: Option<D128>,
        liquidation_threshold: Option<Balance>,
    ) {
        self.assert_owner();

        if let Some(owner) = owner {
            self.owner = owner;
        }

        if let Some(requester_contract) = requester_contract {
            self.requester_contract = requester_contract;
        }

        if let Some(payment_token) = payment_token {
            self.payment_token = payment_token;
        }

        if let Some(safe_ratio) = safe_ratio {
            self.safe_ratio = safe_ratio;
        }

        if let Some(bid_fee) = bid_fee {
            self.bid_fee = bid_fee;
        }

        if let Some(max_premium_rate) = max_premium_rate {
            self.max_premium_rate = max_premium_rate;
        }

        if let Some(liquidation_threshold) = liquidation_threshold {
            self.liquidation_threshold = liquidation_threshold;
        }
    }
}