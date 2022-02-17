use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn get_target_deposit_rate(&self) -> D128 {
        self.config.target_deposit_rate
    }

    pub fn get_borrow_limit(
        &self,
        borrower: AccountId,
        block_time: Option<Timestamp>,
    ) -> (AccountId, U128) {
        let collaterals = self.get_collateral_map(&borrower);

        let (borrow_limit, _) = self.compute_borrow_limit(&collaterals, block_time);

        (borrower, U128::from(borrow_limit))
    }
}
