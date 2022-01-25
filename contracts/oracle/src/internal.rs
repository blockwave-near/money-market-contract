use crate::*;

// Private methods
impl Contract {
    pub(crate) fn assert_caller(&self, expected_caller: &AccountId) {
        assert_eq!(
            &env::predecessor_account_id(),
            expected_caller,
            "This method can only be called by {}",
            expected_caller
        );
    }
    // if whitelist is populated, make sure caller's account is included in it
    pub(crate) fn assert_whitelisted(&self, expected_account: &AccountId) {
        if self.whitelist.len() > 0 {
            assert!(
                self.whitelist.contains(expected_account),
                "ERR_NOT_WHITELISTED"
            )
        }
    }
}