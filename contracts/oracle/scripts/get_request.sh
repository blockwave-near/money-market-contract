#!/bin/bash

requester=${requester:-requester.blockwave.testnet}

[[ -z "$1" ]] && echo "Expected syntax: $0 YOUR_REQUEST_ID" >&2 && exit 1

near call $requester get_data_request "{\"request_id\": \"$1\"}" --accountId $requester
