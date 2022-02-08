use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn update_config(
        &mut self,
        owner: Option<ValidAccountId>,
        stable_coin_contract: Option<ValidAccountId>,
        requester_contract: Option<ValidAccountId>,
        oracle_payment_token: Option<ValidAccountId>,
        overseer_contract: Option<ValidAccountId>,
        custody_contract: Option<ValidAccountId>,
        safe_ratio: Option<D128>,
        bid_fee: Option<D128>,
        liquidator_fee: Option<D128>,
        liquidation_threshold: Option<Balance>,
        waiting_period: Option<U64>,
        collateral_info: Option<CollateralInfo>,
    ) {
        self.assert_owner();
        assert_one_yocto();
        self.internal_update_price_response();

        if let Some(owner) = owner {
            self.config.owner = owner.into();
        }

        if let Some(stable_coin_contract) = stable_coin_contract {
            self.config.stable_coin_contract = stable_coin_contract.into();
        }

        if let Some(requester_contract) = requester_contract {
            self.config.requester_contract = requester_contract.into();
        }

        if let Some(oracle_payment_token) = oracle_payment_token {
            self.config.oracle_payment_token = oracle_payment_token.into();
        }

        if let Some(overseer_contract) = overseer_contract {
            self.config.overseer_contract = overseer_contract.into();
        }

        if let Some(custody_contract) = custody_contract {
            self.config.custody_contract = custody_contract.into();
        }

        if let Some(safe_ratio) = safe_ratio {
            self.config.safe_ratio = safe_ratio;
        }

        if let Some(bid_fee) = bid_fee {
            assert_fees(bid_fee + self.config.liquidator_fee);
            self.config.bid_fee = bid_fee;
        }

        if let Some(liquidator_fee) = liquidator_fee {
            assert_fees(liquidator_fee + self.config.bid_fee);
            self.config.liquidator_fee = liquidator_fee;
        }

        if let Some(liquidation_threshold) = liquidation_threshold {
            self.config.liquidation_threshold = liquidation_threshold;
        }

        if let Some(waiting_period) = waiting_period {
            self.config.waiting_period = waiting_period.into();
        }

        if let Some(collateral_info) = collateral_info {
            self.config.collateral_info = collateral_info;
        }
    }
}