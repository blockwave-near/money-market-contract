use crate::*;

use flux_sdk::consts::{DR_NEW_GAS, GAS_BASE_TRANSFER};
use flux_sdk::{AnswerType, DataRequestDetails, Outcome, RequestStatus};

#[ext_contract(fungible_token)]
pub trait FungibleToken {
    fn ft_total_supply(&self) -> PromiseOrValue<U128>;

    fn ft_balance_of(&self, account_id: AccountId) -> PromiseOrValue<U128>;

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
}

#[ext_contract(requester)]
pub trait RequesterContract {
    fn get_data_request(&self, request_id: ValidAccountId) -> Option<DataRequestDetails>;
}

#[ext_contract(ext_self)]
pub trait Contract {
    fn callback_get_price_response(&mut self, #[callback] result: Option<DataRequestDetails>);
    fn callback_unlock_collateral(
        &self,
        borrower: AccountId,
        cur_collaterals: Tokens,
        borrow_limit: u128,
        block_height: BlockHeight,
    );

    fn callback_liquidate_collateral(
        &self,
        borrower: AccountId,
        cur_collaterals: Tokens,
        borrow_limit: u128,
        block_height: BlockHeight,
    );
}

#[ext_contract(ext_market)]
pub trait MarketContract {
    fn get_borrower_info(
        &mut self,
        borrower: AccountId,
        block_height: Option<BlockHeight>,
    ) -> BorrowerInfo;
}

#[ext_contract(ext_custody_bnear)]
pub trait CustodyBnearContract {
    fn lock_collateral(&mut self, borrower: AccountId, amount: Balance) -> Promise;

    fn unlock_collateral(&mut self, borrower: AccountId, amount: Balance) -> Promise;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BorrowerInfo {
    pub interest_index: D128,
    pub reward_index: D128,
    pub loan_amount: Balance,
    pub pending_rewards: D128,
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
    #[private]
    pub fn callback_get_price_response(&mut self, #[callback] result: Option<DataRequestDetails>) {
        let result: DataRequestDetails = result.expect("ERR: There is no response.");

        let status: RequestStatus = result.status;

        if let RequestStatus::Finalized(outcome) = status {
            if let Outcome::Answer(answer_type) = outcome {
                if let AnswerType::Number(number) = answer_type {
                    // store latest price response
                    self.state.last_price_response = PriceResponse {
                        price: D128::new(number.value.0),
                        last_updated_at: env::block_timestamp(),
                    };
                    // create new price request
                    self.internal_create_new_price_request();
                }
            }
        }
    }

    #[private]
    fn callback_unlock_collateral(
        &mut self,
        borrower: AccountId,
        cur_collaterals: Tokens,
        borrow_limit: u128,
        block_height: BlockHeight,
    ) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::panic("fail".as_bytes());
            }
            PromiseResult::Successful(result) => {
                let borrowerInfo: BorrowerInfo =
                    near_sdk::serde_json::from_slice::<BorrowerInfo>(&result).unwrap();
                if borrow_limit < borrowerInfo.loan_amount {
                    env::panic("UnlockTooLarge".as_bytes());
                }

                self.add_collateral_map(&borrower, &cur_collaterals);

                for collateral in cur_collaterals.clone() {
                    let white_list_elem: WhitelistElem =
                        self.get_white_list_elem_map(&collateral.0);
                    // TODO handle result with {borrwer, amount} from custody
                    ext_custody_bnear::unlock_collateral(
                        borrower.clone(),
                        collateral.1,
                        &white_list_elem.custody_contract,
                        NO_DEPOSIT,
                        SINGLE_CALL_GAS,
                    );
                }
            }
        }
    }

    #[private]
    fn callback_liquidate_collateral(
        &mut self,
        borrower: AccountId,
        cur_collaterals: Tokens,
        borrow_limit: u128,
        block_height: BlockHeight,
    ) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::panic("fail".as_bytes());
            }
            PromiseResult::Successful(result) => {
                let borrowerInfo: BorrowerInfo =
                    near_sdk::serde_json::from_slice::<BorrowerInfo>(&result).unwrap();
                let borrow_amount = borrowerInfo.loan_amount;
                if borrow_limit >= borrow_amount {
                    env::panic("CannotLiquidationSafeLoan".as_bytes());
                }

                let liquidation_amount: Tokens = vec![(String::from(""), 0)]; // TODO: need to cross-contract call to liquidation contract

                let mut latest_collarterals = cur_collaterals.clone();

                latest_collarterals.sub(liquidation_amount.clone());
                self.add_collateral_map(&borrower, &latest_collarterals);

                let prev_balance = 0; // TODO: need to cross-contract call to market contract

                // TODO: handle with Custody Contract
            }
        }
    }
}
