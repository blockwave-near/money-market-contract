use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub owner_id: AccountId,
    pub stable_coin_contract: AccountId,
    pub oracle_contrract: AccountId,
    pub market_contract: AccountId,
    pub liquidation_contract: AccountId,
    pub collector_contract: AccountId,
    pub epoch_period: BlockHeight,
    pub threshold_deposit_rate: D128,
    pub target_deposit_rate: D128,
    pub buffer_distribution_factor: D128,
    pub anc_purchase_factor: D128,
    pub oracle_payment_token: AccountId,
    pub requester_contract: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct State {
    pub deposit_rate: D128,
    pub prev_stable_coin_total_supply: Balance,
    pub prev_exchange_rate: D128,
    pub prev_interest_buffer: u128,
    pub last_executed_height: BlockHeight,
    pub last_price_response: PriceResponse,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Collection {
    pub white_list_elem_map: UnorderedMap<AccountId, WhitelistElem>,
    pub collateral_map: LookupMap<AccountId, Tokens>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct WhitelistElem {
    pub name: String,
    pub symbol: String,
    pub max_ltv: D128,
    pub custody_contract: AccountId,
}

#[near_bindgen]
impl Contract {
    pub fn add_white_list_elem_map(&mut self, key: &String, value: &WhitelistElem) {
        self.collection.white_list_elem_map.insert(&key, value);
    }

    pub fn get_white_list_elem_map(&self, key: &String) -> WhitelistElem {
        match self.collection.white_list_elem_map.get(&key) {
            Some(value) => {
                let log_message = format!("Value from LookupMap is {:?}", value.clone());
                env::log(log_message.as_bytes());
                value
            }
            None => env::panic("".as_bytes()),
        }
    }

    pub fn add_collateral_map(&mut self, key: &String, value: &Tokens) {
        self.collection.collateral_map.insert(&key, value);
    }

    pub fn get_collateral_map(&self, key: &String) -> Tokens {
        match self.collection.collateral_map.get(&key) {
            Some(value) => {
                let log_message = format!("Value from LookupMap is {:?}", value.clone());
                env::log(log_message.as_bytes());
                value
            }
            None => env::panic("".as_bytes()),
        }
    }
}
