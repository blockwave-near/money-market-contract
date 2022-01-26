use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_bid(&self, bidder: &AccountId) -> Bid {
        self.bids.get(bidder).expect("No bids with the specified information exist");
    }
}