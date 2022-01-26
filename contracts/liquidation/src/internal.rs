use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            self.owner,
            "This method can only be called by {}",
            self.owner
        );
    }
}