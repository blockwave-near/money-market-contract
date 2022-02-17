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
        if env::predecessor_account_id() == self.config.collateral_token {
            self.deposit_collateral(sender_id, amount.0);
            return PromiseOrValue::Value(U128(0));
        } else {
            env::log(b"Only whitelisted tokens can transfer_call to this");

            return PromiseOrValue::Value(amount);
        }
    }
}
