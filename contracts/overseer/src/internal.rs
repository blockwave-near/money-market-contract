use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.config.owner_id,
            "Can only be called by the owner"
        );
    }

    /// updates price response at every function call
    pub(crate) fn internal_update_price_response(&mut self) -> Promise {
        requester::get_data_request(
            env::current_account_id().try_into().unwrap(),
            // Near params
            &self.config.requester_contract,
            0,
            3_000_000_000_000,
        )
        .then(ext_self::callback_get_price_response(
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
}
