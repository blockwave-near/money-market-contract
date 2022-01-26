use crate::*;

#[near_bindgen]
impl Contract {
    pub fn update_config(
        &mut self,
        owner: Option<AccountId>,
        oracle_contract: Option<AccountId>,
        safe_ratio: Option<Fraction>,
        bid_fee: Option<Fraction>,
        max_premium_rate: Option<Fraction>,
        liquidation_threshold: Option<Balance>,
        price_timeframe: Option<u64>,
    ) {
        self.assert_owner();

        if let Some(owner) = owner {
            self.owner = owner;
        }

        if let Some(oracle_contract) = oracle_contract {
            self.oracle_contract = oracle_contract;
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

        if let Some(price_timeframe) = price_timeframe {
            self.price_timeframe = price_timeframe;
        }
    }
}