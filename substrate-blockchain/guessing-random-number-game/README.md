# Substrate Custom Pallet Readme

## Overview
This custom Substrate blockchain allows users to play a number guessing game. Sudo user(admin) can set a target number and then let the players make guesses to try and match the target number. The pallet defines storage items, dispatchable functions, events, and errors to support the guessing game functionality.

## Features
- Set a target number
- Check a guess the target number
- Remove the target number
- Emit events for setting a target, making a guess, and removing the target

## Usage
Follow these steps for build and run this chain

1. Clone the repo and build the chain using the following command
    ```bash
    cargo build --release
    ```
2. Run the node using the following command
    ```bash
    ./target/release/node-template --dev
    ```
3. Use the [polkadot.org.js](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/chainstate) for test the functionalities