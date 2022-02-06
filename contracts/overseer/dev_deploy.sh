#!/bin/bash
./build.sh
near dev-deploy \
    --wasmFile res/overseer.wasm \
    --initFunction new \
    --initArgs '{
        "owner_id": "blockwave.testnet",
        "oracle_contrract": "oracle.synchro.testnet",
        "market_contract": "market.synchro.testnet",
        "liquidation_contract": "liquidation.synchro.testnet",
        "collector_contract": "collector.synchro.testnet",
        "decrement_multiplier": {
            "num": 100000000,
            "decimal": 100000000,
        },
    }'