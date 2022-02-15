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

    fn callback_liquidate_collateral2(
        &mut self,
        sender: AccountId,
        borrower: AccountId,
        cur_collaterals: Tokens,
        prev_balance: Balance,
    );

    fn callback_execute_epoch_operations(
        &mut self,
        blocks: BlockHeight,
        mut interest_buffer: Balance,
    );

    fn callback_update_epoch_state(
        &mut self,
        intereset_buffer: U128,
        distributed_intereset: U128,
        block_height: BlockHeight,
        blocks: BlockHeight,
    );
}

#[ext_contract(ext_market)]
pub trait MarketContract {
    fn get_borrower_info(
        &mut self,
        borrower: AccountId,
        block_height: Option<BlockHeight>,
    ) -> BorrowerInfo;

    fn get_balance(&self) -> Balance;

    fn repay_stable_from_liquidation(&mut self, borrower: AccountId, prev_balance: Balance);

    fn get_epoch_state(
        &mut self,
        block_height: Option<BlockHeight>,
        distributed_intereset: Option<U128>,
    ) -> Promise;

    fn execute_epoch_operations(
        &mut self,
        deposit_rate: D128,
        target_deposit_rate: D128,
        threshold_deposit_rate: D128,
        distributed_intereset: U128,
    );
}

#[ext_contract(ext_custody_bnear)]
pub trait CustodyBnearContract {
    fn lock_collateral(&mut self, borrower: AccountId, amount: Balance);

    fn unlock_collateral(&mut self, borrower: AccountId, amount: Balance);

    fn liquidate_collateral(&mut self, liquidator: AccountId, borrower: AccountId, amount: Balance);

    fn distribute_rewards(&self);
}

#[ext_contract(ext_liquidation)]
pub trait LiquidationContract {
    fn get_liquidation_amount();
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
        assert_eq!(env::promise_results_count(), 2, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::panic("fail".as_bytes());
            }
            PromiseResult::Successful(result) => {
                let borrowerInfo: BorrowerInfo =
                    near_sdk::serde_json::from_slice::<BorrowerInfo>(&result).unwrap();

                match env::promise_result(1) {
                    PromiseResult::NotReady => unreachable!(),
                    PromiseResult::Failed => {
                        env::panic("fail".as_bytes());
                    }
                    PromiseResult::Successful(result) => {
                        let prev_balance: Balance =
                            near_sdk::serde_json::from_slice::<Balance>(&result).unwrap();
                        let borrow_amount = borrowerInfo.loan_amount;
                        if borrow_limit >= borrow_amount {
                            env::panic("CannotLiquidationSafeLoan".as_bytes());
                        }

                        ext_liquidation::get_liquidation_amount(
                            &self.config.liquidation_contract,
                            NO_DEPOSIT,
                            SINGLE_CALL_GAS,
                        )
                        .then(ext_self::callback_liquidate_collateral2(
                            env::predecessor_account_id(),
                            borrower,
                            cur_collaterals,
                            prev_balance,
                            &env::current_account_id(),
                            NO_DEPOSIT,
                            SINGLE_CALL_GAS,
                        ));
                    }
                }
            }
        }
    }

    fn callback_liquidate_collateral2(
        &mut self,
        sender: AccountId,
        borrower: AccountId,
        cur_collaterals: Tokens,
        prev_balance: Balance,
    ) {
        assert_eq!(env::promise_results_count(), 2, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::panic("fail".as_bytes());
            }
            PromiseResult::Successful(result) => {
                // let liquidation_amount: Tokens =
                //     near_sdk::serde_json::from_slice::<Tokens>(&result).unwrap();
                let liquidation_amount: Tokens = vec![(String::from(""), 0)]; // TODO: need to cross-contract call to liquidation contract

                let mut latest_collarterals = cur_collaterals.clone();

                latest_collarterals.sub(liquidation_amount.clone());
                self.add_collateral_map(&borrower, &latest_collarterals);

                let liquidations = liquidation_amount.iter().map(|collateral| {
                    let white_list_elem: WhitelistElem =
                        self.get_white_list_elem_map(&collateral.0);

                    ext_custody_bnear::liquidate_collateral(
                        sender.clone(),
                        borrower.clone(),
                        collateral.1,
                        &white_list_elem.custody_contract,
                        NO_DEPOSIT,
                        SINGLE_CALL_GAS,
                    );
                });

                ext_market::repay_stable_from_liquidation(
                    borrower.clone(),
                    prev_balance,
                    &self.config.market_contract,
                    NO_DEPOSIT,
                    SINGLE_CALL_GAS,
                );
            }
        }
    }

    #[private]
    fn callback_execute_epoch_operations(
        &mut self,
        blocks: BlockHeight,
        mut interest_buffer: Balance,
    ) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::panic("fail".as_bytes());
            }
            PromiseResult::Successful(result) => {
                let (exchange_rate, mut stable_coin_total_supply): (D128, U128) =
                    near_sdk::serde_json::from_slice::<(D128, U128)>(&result).unwrap();
                let stable_coin_total_supply: Balance = stable_coin_total_supply.0;

                let effective_deposit_rate = exchange_rate / self.state.prev_exchange_rate;
                let deposit_rate = (effective_deposit_rate - D128::one()) / blocks as u128;

                let accrued_buffer: u128 = interest_buffer - self.state.prev_interest_buffer;
                let anc_purchase_amount: Balance =
                    (accrued_buffer * self.config.anc_purchase_factor).as_u128();

                if anc_purchase_amount != 0 {
                    fungible_token::ft_transfer(
                        self.config.collector_contract.clone(),
                        anc_purchase_amount.into(),
                        None,
                        &self.config.stable_coin_contract,
                        NO_DEPOSIT,
                        SINGLE_CALL_GAS,
                    );
                }

                interest_buffer = interest_buffer - anc_purchase_amount;

                let mut distributed_intereset: u128 = 0;

                if deposit_rate < self.config.threshold_deposit_rate {
                    let missing_deposit_rate = self.config.threshold_deposit_rate - deposit_rate;
                    let prev_deposits =
                        self.state.prev_stable_coin_total_supply * self.state.prev_exchange_rate;

                    let missing_deposits =
                        (prev_deposits * missing_deposit_rate).mul_int(blocks as u128);
                    let distribution_buffer = self
                        .config
                        .buffer_distribution_factor
                        .mul_int(interest_buffer);

                    distributed_intereset = std::cmp::min(missing_deposits, distribution_buffer);
                    interest_buffer = interest_buffer - distributed_intereset;

                    if distributed_intereset != 0 {
                        // TODO should be deduct_tax?
                        fungible_token::ft_transfer(
                            self.config.market_contract.clone(),
                            distributed_intereset.into(),
                            None,
                            &self.config.stable_coin_contract,
                            NO_DEPOSIT,
                            SINGLE_CALL_GAS,
                        );
                    }
                }

                for elem in self.collection.white_list_elem_map.iter() {
                    ext_custody_bnear::distribute_rewards(
                        &elem.1.custody_contract,
                        NO_DEPOSIT,
                        SINGLE_CALL_GAS,
                    );
                }

                self.update_epoch_state(interest_buffer.into(), distributed_intereset.into());
            }
        }
    }

    #[private]
    fn callback_update_epoch_state(
        &mut self,
        intereset_buffer: U128,
        distributed_intereset: U128,
        block_height: BlockHeight,
        blocks: BlockHeight,
    ) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::panic("fail".as_bytes());
            }
            PromiseResult::Successful(result) => {
                let (exchange_rate, mut stable_coin_total_supply): (D128, U128) =
                    near_sdk::serde_json::from_slice::<(D128, U128)>(&result).unwrap();

                let effective_deposit_rate = exchange_rate / self.state.prev_exchange_rate;
                let deposit_rate = (effective_deposit_rate - D128::one()) / blocks as u128;

                self.state.last_executed_height = block_height;
                self.state.prev_stable_coin_total_supply = stable_coin_total_supply.0;
                self.state.prev_exchange_rate = exchange_rate;
                self.state.prev_interest_buffer = intereset_buffer.0;
                self.state.deposit_rate = deposit_rate;

                ext_market::execute_epoch_operations(
                    deposit_rate,
                    self.config.target_deposit_rate,
                    self.config.threshold_deposit_rate,
                    distributed_intereset,
                    &self.config.market_contract,
                    NO_DEPOSIT,
                    SINGLE_CALL_GAS,
                );
            }
        }
    }
}
