use crate::*;

#[ext_contract(ext_stable_coin)]
pub trait ExtStableCoinContract {
  fn ft_total_supply(&self) -> PromiseOrValue<U128>;

  fn ft_balance_of(&self, account_id: AccountId) -> PromiseOrValue<U128>;

  fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) -> Promise;
}

#[ext_contract(ext_market)]
pub trait MarketContract {
  fn get_borrower_info(
    &mut self,
    borrower: AccountId,
    block_height: Option<BlockHeight>,
  ) -> BorrowerInfo;
}

#[ext_contract(ext_self)]
pub trait SelfContract {
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BorrowerInfo {
  pub interest_index: D128,
  pub reward_index: D128,
  pub loan_amount: Balance,
  pub pending_rewards: D128,
}

// TODO: need to move to each files(ex. borrow.ts, deposit.ts, etc )?
#[near_bindgen]
impl Contract {
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
          let white_list_elem: WhitelistElem = self.get_white_list_elem_map(&collateral.0);
          // TODO handle result with {borrwer, amount} from custody
        }
      }
    }
  }

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
