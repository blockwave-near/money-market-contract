#!/bin/bash
./build.sh
near dev-deploy \
    --wasmFile res/bnear_token.wasm \
    --initFunction new \
    --initArgs '{"staking_pool": "dev-1642678330804-87084875037482"}'