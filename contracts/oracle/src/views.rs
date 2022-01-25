use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_data_request(&self, request_id: U64) -> Option<DataRequestDetails> {
        self.data_requests.get(&u64::from(request_id))
    }
}