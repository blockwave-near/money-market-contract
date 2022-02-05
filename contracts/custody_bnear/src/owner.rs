use crate::*;

#[near_bindgen]
impl Contract {
  #[payable]
  pub fn update_config(
    &mut self,
    overseer_contract: Option<AccountId>,
    collateral_token: Option<AccountId>,
    market_contract: Option<AccountId>,
    reward_contract: Option<AccountId>,
    liquidation_contract: Option<AccountId>,
    stable_coin_contract: Option<AccountId>,
    basset_info: Option<BAssetInfo>,
  ) {
    self.assert_owner();
    assert_one_yocto();
    if let Some(overseer_contract) = overseer_contract {
      self.config.overseer_contract = overseer_contract;
    }
    if let Some(collateral_token) = collateral_token {
      self.config.collateral_token = collateral_token;
    }
    if let Some(market_contract) = market_contract {
      self.config.market_contract = market_contract;
    }
    if let Some(reward_contract) = reward_contract {
      self.config.reward_contract = reward_contract;
    }
    if let Some(liquidation_contract) = liquidation_contract {
      self.config.liquidation_contract = liquidation_contract;
    }
    if let Some(stable_coin_contract) = stable_coin_contract {
      self.config.stable_coin_contract = stable_coin_contract;
    }
    if let Some(basset_info) = basset_info {
      self.config.basset_info = basset_info;
    }
  }
}
