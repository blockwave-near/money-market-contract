use crate::*;

#[near_bindgen]
impl Contract {
    /// Overseer executes the liquidation providing a whitelisted collateral(bNEAR).
    /// This operation returns a repay_amount based on the available bids on each
    /// premium slot, consuming bids from lowest to higher premium slots
    pub(crate) fn on_receive_execute_liquidation(
        &mut self,
        sender: AccountId,
        liquidator: AccountId,
        repay_address: AccountId,
        fee_address: AccountId,
        amount: U128,
    ) {
        self.internal_update_price_response();

        let config: Config = self.config.clone();
        let collateral_info: CollateralInfo = config.collateral_info;
        let available_bids: u128 = self.total_bids.0;

        // only collateral token custody can execute liquidations
        assert_eq!(config.custody_contract, sender, "Unauthorized: only custody contract can execute liquidations");

        let mut remaining_collateral_to_liquidate: u128 = amount.0;
        let mut repay_amount: u128 = 0;
        let mut filled: bool = false;
        for slot in 0..collateral_info.max_slot + 1 {
            let mut bid_pool: BidPool = match self.interanl_read_bid_pool(slot) {
                Some(bid_pool) => bid_pool,
                None => continue,
            };
            if bid_pool.total_bid_amount.0 == 0 {
                continue;
            };

            let (pool_repay_amount, pool_liquidated_collateral) = self.internal_execute_pool_liquidation(
                &mut bid_pool,
                slot,
                remaining_collateral_to_liquidate,
                self.last_price_response.price,
                &mut filled,
            );

            self.internal_store_bid_pool(slot, &bid_pool);

            repay_amount += pool_repay_amount;

            if filled {
                remaining_collateral_to_liquidate = 0;
                break;
            } else {
                remaining_collateral_to_liquidate -= pool_liquidated_collateral;
            }
        }

        assert_eq!(remaining_collateral_to_liquidate, 0, "Not enough bids to execute this liquidation");

        self.total_bids = (available_bids - repay_amount).into();

        let bid_fee: D128 = repay_amount * config.bid_fee;
        let liquidator_fee: D128 = repay_amount * config.liquidator_fee;
        let repay_amount: D128 = repay_amount - bid_fee - liquidator_fee;
        
        fungible_token_transfer(
            config.stable_coin_contract.clone(), 
            repay_address, 
            repay_amount.as_u128()
        );

        if bid_fee != D128::zero() {
            fungible_token_transfer(
                config.stable_coin_contract.clone(), 
                fee_address, 
                bid_fee.as_u128()
            );
        }

        if liquidator_fee != D128::zero() {
            fungible_token_transfer(
                config.stable_coin_contract.clone(), 
                liquidator, 
                liquidator_fee.as_u128()
            );
        }
    }

    /// callback on transfer stable coin.
    /// Stable asset is submitted to create a bid record. If available bids for the collateral is under
    /// the threshold, the bid is activated. Bids are not used for liquidations until activated
    pub(crate) fn on_receive_submit_bid(&mut self, bidder: AccountId, premium_slot: u8, amount: U128) {
        self.internal_update_price_response();
        
        let config = self.config.clone();
        let collateral_info = config.collateral_info;

        // read or create bid_pool, make sure slot is valid
        let mut bid_pool: BidPool =
            self.internal_read_or_create_bid_pool(&collateral_info, premium_slot);

        // create bid object
        let bid_idx: u128 = self.internal_pop_bid_idx();
        let mut bid = Bid {
            idx: bid_idx.into(),
            premium_slot,
            bidder,
            amount,
            product_snapshot: D128::one(),
            sum_snapshot: D128::zero(),
            pending_liquidated_collateral: U128(0),
            wait_end: None,
            epoch_snapshot: U128(0),
            scale_snapshot: U128(0),
        };

        // if available bids is lower than bid_threshold, directly activate bid
        let available_bids: U128 = self.total_bids;
        if available_bids.0 < collateral_info.bid_threshold.0 {
            // update bid and bid pool, add new share and pool indexes to bid
            process_bid_activation(&mut bid, &mut bid_pool, amount);

            // store bid_pool
            self.internal_store_bid_pool(premium_slot, &bid_pool);

            // increase total bid amount
            self.total_bids = (available_bids.0 + amount.0).into();
        } else {
            // calculate wait_end from current time
            bid.wait_end = Some(((env::block_timestamp() + (config.waiting_period * SECOND_TO_NANO)) / SECOND_TO_NANO).into());
        };

        self.internal_store_bid(bid_idx.into(), &bid);
    }
}