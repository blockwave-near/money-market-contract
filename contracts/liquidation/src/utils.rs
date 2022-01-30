use crate::*;

use flux_sdk::consts::{DR_NEW_GAS, GAS_BASE_TRANSFER};
use flux_sdk::{DataRequestDetails, RequestStatus, Outcome, AnswerType};
use near_sdk::{ext_contract, AccountId, Promise};
use std::convert::TryInto;
use crate::math::{DECIMAL};

#[ext_contract(fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> Promise;
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> Promise;
    fn ft_balance_of(&self, account_id: AccountId) -> Promise;
}

#[ext_contract(requester)]
pub trait RequesterContract {
    fn get_data_request(&self, request_id: ValidAccountId) -> Option<DataRequestDetails>;
}

#[ext_contract(ext_self)]
pub trait Contract {
    fn callback_get_price_response(&mut self, #[callback] result: Option<DataRequestDetails>);
}

pub fn fungible_token_transfer(
    token_account_id: AccountId,
    receiver_id: AccountId,
    value: u128,
) -> Promise {
    fungible_token::ft_transfer(
        receiver_id,
        U128(value),
        None,
        // Near params
        &token_account_id,
        1,
        GAS_BASE_TRANSFER,
    )
}

pub fn fungible_token_transfer_call(
    token_account_id: AccountId,
    receiver_id: AccountId,
    value: u128,
    msg: String,
) -> Promise {
    fungible_token::ft_transfer_call(
        receiver_id,
        U128(value),
        None,
        msg,
        // Near params
        &token_account_id,
        1,
        DR_NEW_GAS,
    )
}

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_update_price_response(
        &mut self,
        requester_account_id: AccountId,
    ) -> Promise {
        requester::get_data_request(
            env::current_account_id().try_into().unwrap(),
            // Near params
            &requester_account_id,
            0,
            3_000_000_000_000,
        ).then(ext_self::callback_get_price_response(
            // Near params
            &env::current_account_id(),
            0,
            30_000_000_000_000,
        ))
    }

    pub(crate) fn internal_create_new_price_request(&self) {
        fungible_token_transfer_call(
            self.oracle_payment_token.clone(), 
            self.requester_contract.clone(), 
            1_000_000_000_000_000_000_000_000, 
            // query NEAR price
            format!("{{\"sources\": [{{ \"end_point\": \"https://api.coingecko.com/api/v3/simple/price?ids=tether%2Cnear&vs_currencies=usd\", \"source_path\":\"near.usd\"}}], \"tags\":[\"pricing\",\"near\"],  \"challenge_period\":\"120000000000\", \"settlement_time\":\"1\", \"data_type\":{{\"Number\":\"{}\"}}, \"creator\":\"{}\"}}", DECIMAL, env::current_account_id())
        );
    }

    #[private]
    pub fn callback_get_price_response(&mut self, #[callback] result: Option<DataRequestDetails>) {
        let result: DataRequestDetails = result.expect("ERR: There is no response.");
        
        let status: RequestStatus = result.status;

        if let RequestStatus::Finalized(outcome) = status {
            if let Outcome::Answer(answer_type) = outcome {
                if let AnswerType::Number(number) = answer_type {
                    // store latest price response
                    self.last_price_response = PriceResponse{
                        price: D128::new(number.value.0),
                        last_updated_at: env::block_timestamp(),
                    };
                    // create new price request
                    self.internal_create_new_price_request();
                }
            }
        }
    }
}
