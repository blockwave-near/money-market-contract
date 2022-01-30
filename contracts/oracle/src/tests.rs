use super::*;
use crate::fungible_token_handler::FungibleTokenReceiver;
use flux_sdk::DataRequestDataType;
use near_sdk::json_types::U128;
use near_sdk::serde_json;
use near_sdk::MockedBlockchain;
use near_sdk::{testing_env, VMContext};

fn alice() -> AccountId {
    "alice.near".to_string()
}

fn bob() -> AccountId {
    "bob.near".to_string()
}

fn oracle() -> AccountId {
    "oracle.near".to_string()
}

fn token() -> AccountId {
    "token.near".to_string()
}

fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
        current_account_id: alice(),
        signer_account_id: alice(),
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id: token(),
        input,
        block_index: 0,
        block_timestamp: 0,
        account_balance: 10000 * 10u128.pow(24),
        account_locked_balance: 0,
        storage_usage: 0,
        attached_deposit: 0,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view,
        output_data_receivers: vec![],
        epoch_height: 0,
    }
}

#[test]
#[should_panic(expected = "This method can only be called by oracle.near")]
fn ri_not_oracle() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let contract = Contract::new(oracle(), token(), None);
    contract.request_ft_transfer(token(), 100, alice());
}

#[test]
fn ri_create_dr_success() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new(oracle(), token(), None);

    contract.ft_on_transfer(
        alice(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["a".to_string()].to_vec(),
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );
}

#[test]
fn ri_whitelisted_success() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new(
        oracle(),
        token(),
        Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
    );

    contract.ft_on_transfer(
        alice(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["a".to_string()].to_vec(),
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );
}

#[test]
#[should_panic(expected = "ERR_NOT_WHITELISTED")]
fn ri_unwhitelisted_fail() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new(
        oracle(),
        token(),
        Some(vec![serde_json::from_str("\"bob.near\"").unwrap()]),
    );

    contract.ft_on_transfer(
        alice(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["a".to_string()].to_vec(),
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );
}

#[test]
fn ri_empty_tags_nonce_works() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new(
        oracle(),
        token(),
        Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
    );

    contract.ft_on_transfer(
        alice(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["a".to_string()].to_vec(),
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );

    assert!(contract.data_requests.get(&alice()).is_some());
}

#[test]
fn ri_some_tags_nonce_works() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new(
        oracle(),
        token(),
        Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
    );

    contract.ft_on_transfer(
        alice(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["butt".to_owned(), "on".to_owned()],
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );

    assert!(contract.data_requests.get(&alice()).is_some());
}

#[test]
fn ri_nonce_iterates_properly() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new(
        oracle(),
        token(),
        Some(vec![serde_json::from_str("\"alice.near\"").unwrap(), serde_json::from_str("\"bob.near\"").unwrap()]),
    );

    contract.ft_on_transfer(
        alice(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["a".to_string()].to_vec(),
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );

    contract.ft_on_transfer(
        bob(),
        U128(100),
        serde_json::json!({
            "sources": Some(Vec::<String>::new()),
            "outcomes": Some(vec!["a".to_string()].to_vec()),
            "challenge_period": U64(1500),
            "description": Some("a".to_string()),
            "tags": vec!["a".to_string()].to_vec(),
            "data_type": DataRequestDataType::String,
        })
        .to_string(),
    );

    assert!(contract.data_requests.get(&bob()).is_some());
}