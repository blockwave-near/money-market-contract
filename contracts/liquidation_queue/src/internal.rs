use crate::*;

#[near_bindgen]
impl Contract {
    /// updates price response at every function call
    pub(crate) fn internal_update_price_response(
        &mut self,
    ) -> Promise {
        requester::get_data_request(
            env::current_account_id().try_into().unwrap(),
            // Near params
            &self.config.requester_contract,
            0,
            3_000_000_000_000,
        ).then(ext_self::callback_get_price_response(
            // Near params
            &env::current_account_id(),
            0,
            30_000_000_000_000,
        ))
    }

    pub(crate) fn internal_create_new_price_request(&self) {
        fungible_token_transfer_call(
            self.config.oracle_payment_token.clone(), 
            self.config.requester_contract.clone(), 
            1_000_000_000_000_000_000_000_000, 
            // query NEAR price
            format!("{{\"sources\": [{{ \"end_point\": \"https://api.coingecko.com/api/v3/simple/price?ids=tether%2Cnear&vs_currencies=usd\", \"source_path\":\"near.usd\"}}], \"tags\":[\"pricing\",\"near\"],  \"challenge_period\":\"120000000000\", \"settlement_time\":\"1\", \"data_type\":{{\"Number\":\"{}\"}}, \"creator\":\"{}\"}}", DECIMAL, env::current_account_id())
        );
    }

    /// On each collateral execution the product_snapshot and sum_snapshot are updated
    /// to track the expense and reward distribution for biders in the pool
    pub(crate) fn internal_execute_pool_liquidation(
        &mut self, 
        bid_pool: &mut BidPool,
        premium_slot: u8,
        collateral_to_liquidate: u128,
        price: D128,
        filled: &mut bool,
    ) -> (u128, u128) {
        let premium_price: D128 = price * (D128::one() - bid_pool.premium_rate);
        let mut pool_collateral_to_liquidate: u128 = collateral_to_liquidate;
        let mut pool_required_stable: D128 = pool_collateral_to_liquidate * premium_price;

        if pool_required_stable > D128::new(bid_pool.total_bid_amount.0 * DECIMAL) {
            pool_required_stable = D128::new(bid_pool.total_bid_amount.0 * DECIMAL);
            pool_collateral_to_liquidate = (pool_required_stable / premium_price).as_u128();
        } else {
            *filled = true;
        }

        // E / D
        let col_per_bid: D128 = D128::new(pool_collateral_to_liquidate * DECIMAL)
            / bid_pool.total_bid_amount.0;
        
        // Q / D
        let expense_per_bid: D128 = pool_required_stable
            / bid_pool.total_bid_amount.0;
        
        ///////// Update sum /////////
        // E / D * P     
        let sum: D128 = bid_pool.product_snapshot * col_per_bid;   

        // S + E / D * P
        bid_pool.sum_snapshot = bid_pool.sum_snapshot + sum;
        bid_pool.total_bid_amount = (bid_pool.total_bid_amount.0 - pool_required_stable.as_u128()).into();

        // save reward sum for current epoch and scale
        self.internal_store_epoch_scale_sum(
            premium_slot,
            bid_pool.current_epoch,
            bid_pool.current_scale,
            bid_pool.sum_snapshot,
        );

        ///////// Update product /////////
        // Check if the pool is emptied, if it is, reset (P = 1, S = 0)
        if expense_per_bid == D128::one() {
            bid_pool.sum_snapshot = D128::zero();
            bid_pool.product_snapshot = D128::one();
            bid_pool.current_scale = U128(0);

            bid_pool.current_epoch = (bid_pool.current_epoch.0 + 1).into();
        } else {
            // 1 - Q / D
            let product: D128 = D128::one() - expense_per_bid;

            // check if scale needs to be increased (in case product truncates to zero)
            let new_product: D128 = bid_pool.product_snapshot * product;
            bid_pool.product_snapshot = if new_product < D128::new(1_000_000_000) {
                bid_pool.current_scale = (bid_pool.current_scale.0 + 1).into();

                D128::new(bid_pool.product_snapshot.num.0 * 1_000_000_000u128) * product
            } else {
                new_product
            };
        }

        env::log(
            format!(
                "product: {}", bid_pool.product_snapshot
            )
        );
        (pool_required_stable.as_u128(), pool_collateral_to_liquidate)
    }

    pub(crate) fn internal_calculate_remaining_bid(&self, bid: &Bid, bid_pool: &BidPool) -> (U128, D128) {
        let scale_diff: u128 = bid_pool.current_scale.0.checked_sub(bid.scale_snapshot.0).unwrap();
        let epoch_diff: u128 = bid_pool.current_epoch.0.checked_sub(bid.epoch_snapshot.0).unwrap();

        let remaining_bid_dec: D128 = if epoch_diff != 0 {
            // pool was emptied, return 0
            D128::zero()
        } else if scale_diff == 0 {
            bid.amount.0 * bid_pool.product_snapshot / bid.product_snapshot
        } else if scale_diff == 1 {
            // product has been scaled
            let scaled_remaining_bid: D128 =
                bid.amount.0 * bid_pool.product_snapshot / bid.product_snapshot;
            
            D128::new(scaled_remaining_bid.num.0 / 1_000_000_000u128)
        } else {
            D128::zero()
        };

        let remaining_bid: u128 = remaining_bid_dec.as_u128();
        // stacks the residue when converting to integer
        let bid_residue: D128 = remaining_bid_dec - remaining_bid;

        (remaining_bid.into(), bid_residue)
    }

    pub(crate) fn internal_calculate_liquidated_collateral(&self, bid: &Bid) -> (U128, D128) {
        let reference_sum_snapshot: D128 = self.internal_read_epoch_scale_sum(
            bid.premium_slot,
            bid.epoch_snapshot,
            bid.scale_snapshot,
        ).unwrap_or(D128::zero());

        // reward = reward from first scale + reward from second scale (if any)
        let first_portion = reference_sum_snapshot - bid.sum_snapshot;
        let second_portion: D128 = if let Some(second_scale_sum_snapshot) = self.internal_read_epoch_scale_sum(
            bid.premium_slot,
            bid.epoch_snapshot,
            (bid.scale_snapshot.0 + 1).into()
        ) {
            D128::new((second_scale_sum_snapshot.num.0 - reference_sum_snapshot.num.0) / 1_000_000_000u128)
        } else {
            D128::zero()
        };

        let liquidation_collateral_dec: D128 = bid.amount.0 * (first_portion + second_portion)
            / bid.product_snapshot;
        let liquidated_collateral: u128 = liquidation_collateral_dec.as_u128();
        // stacks the residue when converting to integer
        let residue_collateral: D128 =
            liquidation_collateral_dec - liquidated_collateral;

        (liquidated_collateral.into(), residue_collateral)
    }

    pub(crate) fn internal_claim_col_residue(&self, bid_pool: &mut BidPool) -> u128 {
        let claimable = bid_pool.residue_collateral.as_u128();
        if claimable != 0 {
            bid_pool.residue_collateral = bid_pool.residue_collateral - claimable;
        }
        claimable
    }

    pub(crate) fn internal_claim_bid_residue(&self, bid_pool: &mut BidPool) -> u128 {
        let claimable = bid_pool.residue_bid.as_u128();
        if claimable != 0 {
            bid_pool.residue_bid = bid_pool.residue_bid - claimable;
        }
        claimable
    }
}