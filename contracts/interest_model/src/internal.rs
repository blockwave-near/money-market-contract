use crate::*;

impl Contract {
  pub(crate) fn assert_owner(&self) {
    assert_eq!(
      env::predecessor_account_id(),
      self.owner_id,
      "Can only be called by the owner"
    );
  }
}
