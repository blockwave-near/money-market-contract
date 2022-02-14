use crate::*;

#[near_bindgen]
impl Contract {
    pub fn lock_collateral(&mut self, collaterals: Tokens) {
        let borrower = env::predecessor_account_id();
        let mut cur_collaterals: Tokens = self.get_collateral_map(&borrower);

        cur_collaterals.add(collaterals);
        self.add_collateral_map(&borrower, &cur_collaterals);

        for collateral in cur_collaterals {
            let white_list_elem: WhitelistElem = self.get_white_list_elem_map(&collateral.0);
            ext_custody_bnear::lock_collateral(
                borrower.clone(),
                collateral.1,
                &white_list_elem.custody_contract,
                NO_DEPOSIT,
                SINGLE_CALL_GAS,
            );
        }
    }

    pub fn unlock_collateral(&mut self, collaterals: Tokens) {
        let borrower = env::predecessor_account_id();
        let mut cur_collaterals: Tokens = self.get_collateral_map(&borrower);

        cur_collaterals.sub(collaterals.clone());

        let block_height = env::block_index();

        let (borrow_limit, _) =
            self.compute_borrow_limit(&collaterals, Some(env::block_timestamp()));

        ext_market::get_borrower_info(
            borrower.clone(),
            Some(block_height),
            &self.config.market_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        )
        .then(ext_self::callback_unlock_collateral(
            borrower,
            cur_collaterals,
            borrow_limit,
            block_height,
            &env::current_account_id(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ));
    }

    pub fn liquidate_collateral(&self, borrower: AccountId) {
        let mut cur_collaterals: Tokens = self.get_collateral_map(&borrower);

        let (borrow_limit, collateral_prices) =
            self.compute_borrow_limit(&cur_collaterals, Some(env::block_timestamp()));

        let block_height = env::block_index();

        ext_market::get_borrower_info(
            borrower.clone(),
            Some(block_height),
            &self.config.market_contract,
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        )
        .then(ext_self::callback_liquidate_collateral(
            borrower,
            cur_collaterals,
            borrow_limit,
            block_height,
            &env::current_account_id(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        ));
    }

    pub(crate) fn compute_borrow_limit(
        &self,
        collaterals: &Tokens,
        block_time: Option<BlockHeight>, // TODO: What is it?
    ) -> (u128, Vec<D128>) {
        let mut borrow_limit: u128 = 0;
        let mut collateral_prices: Vec<D128> = vec![];

        for collateral in collaterals.iter() {
            let collateral_token = collateral.0.clone();
            let collateral_amount = collateral.1;

            let price = self.state.last_price_response.price;

            let elem: WhitelistElem = self.get_white_list_elem_map(&collateral.0);
            let collateral_value = collateral_amount * price;
            borrow_limit += (collateral_value * elem.max_ltv).as_u128();
            collateral_prices.push(price);
        }

        (borrow_limit, collateral_prices)
    }
}
