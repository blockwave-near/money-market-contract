use crate::*;

#[near_bindgen]
impl Contract {
    pub fn update_config(
        &mut self,
        owner: Option<AccountId>,
        bnear_contract: Option<AccountId>,
        stable_coin_contract: Option<AccountId>,
        requester_contract: Option<AccountId>,
        oracle_payment_token: Option<AccountId>,
        safe_ratio: Option<D128>,
        bid_fee: Option<D128>,
        max_premium_rate: Option<D128>,
        liquidation_threshold: Option<Balance>,
    ) {
        self.assert_owner();

        if let Some(owner) = owner {
            self.owner = owner;
        }

        if let Some(bnear_contract) = bnear_contract {
            self.bnear_contract = bnear_contract;
        }

        if let Some(stable_coin_contract) = stable_coin_contract {
            self.stable_coin_contract = stable_coin_contract;
        }

        if let Some(requester_contract) = requester_contract {
            self.requester_contract = requester_contract;
        }

        if let Some(oracle_payment_token) = oracle_payment_token {
            self.oracle_payment_token = oracle_payment_token;
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