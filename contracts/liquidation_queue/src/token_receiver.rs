use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BnearReceiverPayload {
    pub liquidator: AccountId,
    pub repay_address: Option<AccountId>,
    pub fee_address: Option<AccountId>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct StableReceiverPayload {
    pub premium_slot: u8,
}

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
        if env::predecessor_account_id() == self.config.collateral_info.bnear_contract {
            let payload: BnearReceiverPayload =
                serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");

            let repay_address: AccountId = payload.repay_address.unwrap_or(sender_id.clone());
            let fee_address: AccountId = payload.fee_address.unwrap_or(sender_id.clone());
            
            self.on_receive_execute_liquidation(sender_id, payload.liquidator, repay_address, fee_address, amount);

            return PromiseOrValue::Value(U128(0));
        } else if env::predecessor_account_id() == self.config.stable_coin_contract {
            let payload: StableReceiverPayload =
                serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");

            self.on_receive_submit_bid(sender_id, payload.premium_slot, amount);

            return PromiseOrValue::Value(U128(0));
        } else {
            panic!("Only whitelisted tokens can transfer_call to this");
        }
    }
}