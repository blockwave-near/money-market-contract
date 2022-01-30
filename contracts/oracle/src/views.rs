use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_data_request(&self, request_id: ValidAccountId) -> Option<DataRequestDetails> {
        self.data_requests.get(&request_id.into())
    }
}