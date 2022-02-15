use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn update_config(
        &mut self,
        oracle_contrract: Option<AccountId>,
        market_contract: Option<AccountId>,
        liquidation_contract: Option<AccountId>,
        collector_contract: Option<AccountId>,
        epoch_period: Option<BlockHeight>,
        target_deposit_rate: Option<D128>,
        oracle_payment_token: Option<AccountId>,
        requester_contract: Option<AccountId>,
    ) {
        self.assert_owner();
        assert_one_yocto();
        if let Some(oracle_contrract) = oracle_contrract {
            self.config.oracle_contrract = oracle_contrract;
        }
        if let Some(market_contract) = market_contract {
            self.config.market_contract = market_contract;
        }
        if let Some(liquidation_contract) = liquidation_contract {
            self.config.liquidation_contract = liquidation_contract;
        }
        if let Some(collector_contract) = collector_contract {
            self.config.collector_contract = collector_contract;
        }
        if let Some(epoch_period) = epoch_period {
            self.config.epoch_period = epoch_period;
        }
        if let Some(target_deposit_rate) = target_deposit_rate {
            self.config.target_deposit_rate = target_deposit_rate;
        }
        if let Some(oracle_payment_token) = oracle_payment_token {
            self.config.oracle_payment_token = oracle_payment_token;
        }
        if let Some(requester_contract) = requester_contract {
            self.config.requester_contract = requester_contract;
        }
    }
}
