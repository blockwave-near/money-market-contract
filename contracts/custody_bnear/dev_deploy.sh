#!/bin/bash
./build.sh
near dev-deploy \
    --wasmFile res/custody_bnear.wasm \
    --initFunction new \
    --initArgs '{
        "owner_id": "blockwave.testnet",
        "overseer_contract": "overseer.synchro.testnet",
        "collateral_token": "bnear.synchro.testnet",
        "market_contract": "market.synchro.testnet",
        "reward_contract": "reward.synchro.testnet",
        "liquidation_contract": "liquidation.synchro.testnet",
        "stable_coin_contract": "stable_coin.testnet",
        "basset_info": {
            "name": "bnear",
            "symbol": "bNear",
            "decimals": 8,
        },
    }'
