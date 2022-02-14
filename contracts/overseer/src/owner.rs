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
    }
}
