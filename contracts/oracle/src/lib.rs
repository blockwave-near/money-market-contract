use flux_sdk::{
    DataRequestDetails, NewDataRequestArgs, Nonce, Outcome, RequestStatus, WrappedBalance,
};
use fungible_token_handler::{fungible_token_transfer_call};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, U64};
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

mod fungible_token_handler;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests;
mod internal;
mod views;


near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub oracle: AccountId,
    pub payment_token: AccountId,
    pub nonce: Nonce,
    pub data_requests: LookupMap<AccountId, DataRequestDetails>,
    pub whitelist: UnorderedSet<AccountId>, // accounts allowed to call create_data_request(). if len() == 0, no whitelist (any account can make data request)
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"Contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        oracle: AccountId,
        payment_token: AccountId,
        whitelist: Option<Vec<ValidAccountId>>,
    ) -> Self {
        let mut requester_instance = Self {
            oracle,
            payment_token,
            nonce: Nonce::new(),
            data_requests: LookupMap::new(b"drq".to_vec()),
            whitelist: UnorderedSet::new(b"w".to_vec()),
        };

        // populate whitelist
        if let Some(whitelist) = whitelist {
            for acct in whitelist {
                requester_instance.whitelist.insert(acct.as_ref());
            }
        }

        requester_instance
    }

    #[payable]
    pub fn create_data_request(
        &mut self,
        amount: WrappedBalance,
        creator: AccountId,
        payload: NewDataRequestArgs,
    ) -> Promise {
        self.assert_caller(&self.payment_token);
        let request_id = creator.clone();

        // insert request_id into tags
        let mut payload = payload;
        let mut tags = payload.tags;
        tags.push(request_id.clone());
        payload.tags = tags.to_vec();

        let dr = DataRequestDetails {
            amount,
            payload: payload.clone(),
            tags: tags,
            status: RequestStatus::Pending,
            creator,
            has_withdrawn_validity_bond: false,
        };
        self.data_requests.insert(&request_id, &dr);
        log!("storing data request under {}", request_id);
        fungible_token_transfer_call(
            self.payment_token.clone(),
            self.oracle.clone(),
            amount.into(),
            json!({ "NewDataRequest": payload }).to_string(),
        )
    }

    #[payable]
    pub fn set_outcome(&mut self, requester: AccountId, outcome: Outcome, tags: Vec<String>) {
        self.assert_caller(&self.oracle);
        assert_eq!(
            env::current_account_id(),
            requester,
            "can only set outcomes for requests that are initiated by this requester"
        );
        assert_eq!(env::attached_deposit(), 1);

        let request_id = tags.last().unwrap();
        let mut request = self.data_requests.get(&request_id).unwrap();
        request.status = RequestStatus::Finalized(outcome);
        self.data_requests.insert(&request_id, &request);
    }
}