#!/bin/bash

# default params
network=${network:-testnet}
accountId=${accountId:-requester.blockwave.testnet}
master=${master:-blockwave.testnet}
initialBalance=${initialBalance:-10}

while [ $# -gt 0 ]; do

   if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="$2"
        echo $1 $2
   fi

  shift
done

NEAR_ENV=$network near delete $accountId $master
NEAR_ENV=$network near create-account $accountId --masterAccount $master --initialBalance $initialBalance