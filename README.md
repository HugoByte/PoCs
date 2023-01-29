<!--
 Copyright 2023 soul

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# CROSS CONTRACT CALL

A simple demonstration of cross contract call in cosmwasm

# Overview

Simple demonstration of Cross Contract Call using Counter example.

When an end user executes message in `Counter1` it increases it counter and pass the message to Contract `Counter2` .
Contract `Counter2` increases the count after sucessfully exectuing the message received form `Counter1`.

This is a trivial example to demonstrate cross-contract calls.

We can query the count from `Counter1` to see how many times the message has been success fully executed in `Conuter2`

## HOW TO DEPLOY

- Crate `WASM` binary of the contracts by running

  `cargo wasm` or `RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --lib`

- Initialize the terminal and set the ENV by running

  `source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/malaga-420/defaults.env)`

- Deploy generated `WASM` binary

  `RES=$(wasmd tx wasm store <WASM_BINARY_PATH>  --from <WALLET_ADDRESS> $TXFLAG -y --output json -b block)`

- Extract the Contract CODE_ID from Result of previous execution

  `CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')`

- Instantiate contract by provinding Instantiate message

  `wasmd tx wasm instantiate $CODE_ID "$INIT" --from <WALLET_ADDRESS> --label "<LABEL>" $TXFLAG -y --no-admin`

- Get the contract address

  `CONTRACT=$(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')`

Now you can execute messages , query results from the contract using contract address.
