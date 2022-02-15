#!/bin/bash
./build.sh
near dev-deploy \
    --wasmFile res/overseer.wasm \
    --initFunction new \
    --initArgs '{
        "owner_id": "blockwave.testnet",
        "stable_coin_contract": "stable.coin.testnet",
        "oracle_contrract": "oracle.synchro.testnet",
        "market_contract": "market.synchro.testnet",
        "liquidation_contract": "liquidation.synchro.testnet",
        "collector_contract": "collector.synchro.testnet",
        "epoch_period": 86400,
        "threshold_deposit_rate: {
            "num": 300000,
            "decimal": 100000000,
        },
        "target_deposit_rate: {
            "num": 500000,
            "decimal": 100000000,
        },
        "buffer_distribution_factor: {
            "num": 20000000,
            "decimal": 100000000,
        },
        "anc_purchase_factor: {
            "num": 20000000,
            "decimal": 100000000,
        },
        "decrement_multiplier": {
            "num": 100000000,
            "decimal": 100000000,
        },
        "oracle_payment_token": "",
        "requester_contract": "",
    }'
