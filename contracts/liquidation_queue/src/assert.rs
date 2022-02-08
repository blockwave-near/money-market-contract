use crate::*;

#[near_bindgen]
impl Contract{
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.config.owner,
            "This method can only be called by {}",
            self.config.owner
        );
    }
}

pub fn assert_fees(fees: D128) {
    assert!(fees > D128::one(), "The sum of bid_fee and liquidator_fee can not be greater than one");
}

pub fn assert_activate_status(bid: &Bid, available_bids: U128, bid_threshold: U128) -> Result<(), String> {
    match bid.wait_end {
        Some(wait_end) => {
            if available_bids.0 < bid_threshold.0 {
                // skip waiting period
                return Ok(());
            } else if wait_end.0 > (env::block_timestamp() / SECOND_TO_NANO) {
                return Err(format!(
                    "Wait period expires at {}",
                    wait_end.0
                ));
            }
        }
        None => {
            return Err(String::from("Bid is already active"));
        },
    }
    Ok(())
}

pub fn assert_withdraw_amount(withdraw_amount: Option<U128>, withdrawable_amount: U128) -> U128 {
    let to_withdraw: U128 = if let Some(amount) = withdraw_amount {
        if amount.0 > withdrawable_amount.0 {
            panic!("{}", format!(
                "Requested amount is bigger than current withdrawable amount ({})",
                withdrawable_amount.0)
            );
        }
        amount
    } else {
        withdrawable_amount
    };

    to_withdraw
}