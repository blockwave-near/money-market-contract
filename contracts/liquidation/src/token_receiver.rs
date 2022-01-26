use crate::*;
use near_sdk::{PromiseOrValue};

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<WrappedBalance> {
        let payload: NewDataRequestArgs =
            serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");
        self.assert_whitelisted(&sender_id.clone().into()); // if whitelist is set, make sure sender can call create_data_request()
        PromiseOrValue::Promise(self.create_data_request(amount.into(), sender_id.into(), payload))
    }
}