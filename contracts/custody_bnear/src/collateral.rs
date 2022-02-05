use crate::*;

#[near_bindgen]
impl Contract {
  // Executor: bAsset token contract
  pub fn deposit_collateal(&mut self, borrower: AccountId, amount: Balance) {
    let mut borrower_info: BorrowerInfo = self.get_borrower_info_map(&borrower);

    borrower_info.balance += amount;
    borrower_info.spendable += amount;

    self.add_borrower_info_map(&borrower, &borrower_info);
  }

  // Executor: borrwer
  pub fn withdraw_collateral(&mut self, amount: Option<Balance>) {
    let borrower = env::predecessor_account_id();
    let mut borrower_info: BorrowerInfo = self.get_borrower_info_map(&borrower);

    let amount = amount.unwrap_or(borrower_info.spendable);
    if borrower_info.spendable < amount {
      env::panic(
        ("Withdraw Amount Exceeds Spendable: ".to_string() + &borrower_info.spendable.to_string())
          .as_bytes(),
      );
    }

    borrower_info.balance = borrower_info.balance - amount;
    borrower_info.spendable = borrower_info.spendable - amount;

    self.add_borrower_info_map(&borrower, &borrower_info);

    fungible_token::ft_transfer(
      borrower,
      U128::from(amount),
      None,
      &self.config.collateral_token,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    );
  }

  // Executor: overseer
  pub fn lock_collateral(&mut self, borrower: AccountId, amount: Balance) {
    self.assert_overseer();

    let mut borrower_info: BorrowerInfo = self.get_borrower_info_map(&borrower);

    if amount > borrower_info.spendable {
      env::panic(
        ("Lock Amount Exceeds Spendable: ".to_string() + &borrower_info.spendable.to_string())
          .as_bytes(),
      );
    }

    borrower_info.spendable = borrower_info.spendable - amount;

    self.add_borrower_info_map(&borrower, &borrower_info);
  }

  pub fn unlock_collateral(&mut self, borrower: AccountId, amount: Balance) {
    self.assert_overseer();

    let mut borrower_info: BorrowerInfo = self.get_borrower_info_map(&borrower);
    let borrrowed_amount = borrower_info.balance - borrower_info.spendable;

    if amount > borrrowed_amount {
      env::panic(
        ("Unlcok Amount Exceeds Locked: ".to_string() + &borrrowed_amount.to_string()).as_bytes(),
      );
    }

    borrower_info.spendable += amount;
    self.add_borrower_info_map(&borrower, &borrower_info);
  }

  // Executer: overseer
  pub fn liquidate_collateral(
    &mut self,
    liquidator: AccountId,
    borrower: AccountId,
    amount: Balance,
  ) {
    self.assert_overseer();

    let mut borrower_info: BorrowerInfo = self.get_borrower_info_map(&borrower);
    let borrrowed_amount = borrower_info.balance - borrower_info.spendable;

    if amount > borrrowed_amount {
      env::panic(
        ("Liquidation Amount Exceeds Locked: ".to_string() + &borrrowed_amount.to_string())
          .as_bytes(),
      );
    }

    borrower_info.balance = borrower_info.balance - amount;
    self.add_borrower_info_map(&borrower, &borrower_info);

    let msg = serde_json::to_string(&{
      // TODO: call liquidation. how?
    })
    .unwrap();

    fungible_token::ft_transfer_call(
      self.config.liquidation_contract.clone(),
      U128::from(amount),
      None,
      msg,
      &self.config.collateral_token,
      NO_DEPOSIT,
      SINGLE_CALL_GAS,
    );
  }
}
