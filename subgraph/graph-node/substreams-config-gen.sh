#!/bin/bash

export NETWORK=solana-mainnet-beta
export SUBSTREAMS_ENDPOINT=mainnet.sol.streamingfast.io:443
export SUBSTREAMS_API_TOKEN=${SUBSTREAMS_API_TOKEN}

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

envsubst < ${ROOT}/config/substreams-generic.template.toml > ${ROOT}/generated-config.toml

echo "A new config.toml has been generated with the following values: 
NETWORK=${NETWORK}
SUBSTREAMS_ENDPOINT=${SUBSTREAMS_ENDPOINT}
SUBSTREAMS_API_TOKEN=${SUBSTREAMS_API_TOKEN}"

