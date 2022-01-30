use crate::*;

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
    ) -> PromiseOrValue<U128> {
        assert_eq!(env::predecessor_account_id(), self.bnear_contract, "Only {} token can transfer_call to this", self.bnear_contract);

        let payload: ReceiverPayload =
            serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");

        let repay_address: AccountId = payload.repay_address.unwrap_or(env::predecessor_account_id());
        let fee_address: AccountId = payload.fee_address.unwrap_or(env::predecessor_account_id());
        
        self.internal_execute_bid(payload.liquidator, repay_address, fee_address, amount);

        PromiseOrValue::Value(U128(0))
    }
}