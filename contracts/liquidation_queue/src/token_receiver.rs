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
        if env::predecessor_account_id() == self.bnear_contract {
            let payload: BnearReceiverPayload =
                serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");

            let repay_address: AccountId = payload.repay_address.unwrap_or(sender_id.clone());
            let fee_address: AccountId = payload.fee_address.unwrap_or(sender_id.clone());
            
            self.internal_execute_bid(payload.liquidator, repay_address, fee_address, amount);

            return PromiseOrValue::Value(U128(0));
        } else if env::predecessor_account_id() == self.stable_coin_contract {
            let payload: StableReceiverPayload =
                serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");

            self.internal_submit_bid(sender_id, payload.premium_rate, amount);

            return PromiseOrValue::Value(U128(0));
        } else {
            env::log(b"Only whitelisted tokens can transfer_call to this");

            return PromiseOrValue::Value(amount);
        }
    }
}