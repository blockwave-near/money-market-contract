#!/bin/bash
./build.sh
near deploy \
    --accountId market.synchro.testnet \
    --wasmFile res/market.wasm \
    --initFunction new \
     --initArgs '{
        "owner_id": "blockwave.testnet",
        "max_borrow_factor": {
            "num": "100000000",
            "decimal": 100000000
        },
        "stable_coin_contract": "stable_coin.testnet",
        "overseer_contract": "overseer.synchro.testnet",
        "collector_contract": "token.synchro.testnet",
        "distributor_contract": "token.synchro.testnet",
        "base_rate": {
            "num": "10000000",
            "decimal": 100000000
        },
        "interest_multiplier": {
            "num": "10000000",
            "decimal": 100000000
        },
        "emission_cap": {
            "num": "10000000000",
            "decimal": 100000000
        },
        "emission_floor": {
            "num": "1000000000",
            "decimal": 100000000
        },
        "increment_multiplier": {
            "num": "110000000",
            "decimal": 100000000
        },
        "decrement_multiplier": {
            "num": "90000000",
            "decimal": 100000000
        }
    }'