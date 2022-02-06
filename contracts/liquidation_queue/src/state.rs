use crate::*;

const DEFAULT_LIMIT: u8 = 10;
const MAX_LIMIT: u8 = 31;

#[near_bindgen]
impl Contract{
    pub(crate) fn internal_read_bid(&self, bid_idx: U128) -> Option<Bid> {
        self.bids.get(&bid_idx)
    }

    pub(crate) fn internal_store_bid(&mut self, bid_idx: U128, bid: &Bid) {
        self.bids.insert(&bid_idx, bid);

        let mut idx_set: UnorderedSet<U128> = self.bids_indexer_by_user.get(&bid.bidder)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKeys::Account { account_hash: env::sha256(bid.bidder.as_bytes()) }
                )
            }
        );
        idx_set.insert(&bid_idx);
        self.bids_indexer_by_user.insert(&bid.bidder, &idx_set);
    }

    pub(crate) fn internal_remove_bid(&mut self, bid_idx: U128) {
        let bid: Bid = self.internal_read_bid(bid_idx).expect("No bids with the specified information exist");
        self.bids.remove(&bid_idx);

        let mut idx_set: UnorderedSet<U128> = self.bids_indexer_by_user.get(&bid.bidder).unwrap();
        idx_set.remove(&bid_idx);
        self.bids_indexer_by_user.insert(&bid.bidder, &idx_set);
    }

    pub(crate) fn interanl_read_bid_pool(&self, premium_slot: u8) -> Option<BidPool> {
        self.bid_pools.get(&premium_slot)
    }

    pub(crate) fn internal_store_bid_pool(&mut self, premium_slot: u8, bid_pool: &BidPool) {
        self.bid_pools.insert(&premium_slot, bid_pool);
    }

    pub(crate) fn internal_read_or_create_bid_pool(&mut self, collateral_info: &CollateralInfo, premium_slot: u8) -> BidPool {
        match self.interanl_read_bid_pool(premium_slot) {
            Some(bid_pool) => bid_pool,
            None => {
                assert!((0..collateral_info.max_slot + 1).contains(&premium_slot), "Invalid premium slot");

                let bid_pool = BidPool {
                    product_snapshot: D128::one(),
                    sum_snapshot: D128::zero(),
                    total_bid_amount: U128(0),
                    premium_rate: collateral_info.premium_rate_per_slot * (premium_slot as u128),
                    current_epoch: U128(0),
                    current_scale: U128(0),
                    residue_collateral: D128::zero(),
                    residue_bid: D128::zero(),
                };

                self.internal_store_bid_pool(premium_slot, &bid_pool);
                bid_pool
            }
        }
    }

    // TODO: impl
    pub(crate) fn internal_read_bid_pools(&self, start_after: Option<u8>, limit: Option<u8>) -> Option<Vec<BidPool>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start: u8 = self.internal_calc_range_start(start_after);

        // let mut bid_pools: Vec<(u8, BidPool)> = self.bid_pools.to_vec();
        // let mut slice: &[(u8, BidPool)] = &bid_pools[(start as usize)..];
        // slice.sort_by(|a, b| (&a.0).cmp(&b.0));
        
        // slice.to_vec().map(|elem| {
        //     let (_, pool) = elem;
        //     pool
        // })
        // .collect()

        let mut bid_pools = self.bid_pools;
        let keys: &Vector<u8> = bid_pools.keys_as_vector();
    }

    // this will set the first key after the provided key, by appending a 1 byte
    pub(crate) fn internal_calc_range_start(&self, start_after: Option<u8>) -> u8 {
        start_after.unwrap_or(0) + 1
    }
}