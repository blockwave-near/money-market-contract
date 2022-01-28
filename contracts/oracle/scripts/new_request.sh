#!/bin/bash

network=${network:-testnet}
accountId=${accountId:-requester.blockwave.testnet}
senderId=${senderId:-blockwave.testnet}
paymentToken=${paymentToken:-v2.wnear.flux-dev}

while [ $# -gt 0 ]; do

   if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="$2"
        echo $1 $2
   fi

  shift
done

JSON='{\"sources\": [ { \"end_point\": \"https://api.coingecko.com/api/v3/simple/price?ids=tether%2Cnear&vs_currencies=usd&include_last_updated_at=true\", \"source_path\":\"\"}], \"tags\":[\"pricing\",\"near\",\"tether\"],  \"challenge_period\":\"120000000000\", \"settlement_time\":\"1\", \"data_type\":\"String\", \"creator\":\"blockwave.testnet\"}'
env NEAR_ENV=testnet near call $paymentToken ft_transfer_call "{\"amount\": \"1000000000000000000000000\", \"msg\": \"$JSON\", \"receiver_id\": \"$accountId\"}" --accountId $senderId --amount 0.000000000000000000000001 --gas=300000000000000

# env NEAR_ENV=$network near call $paymentToken ft_transfer_call $ARGS --accountId $accountId --amount 0.000000000000000000000001 --gas=300000000000000