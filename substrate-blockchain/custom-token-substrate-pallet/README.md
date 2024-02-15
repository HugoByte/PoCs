## Custom Token Pallet for Substrate

The custom token pallet offered here facilitates the implementation of a custom token functionality within a Substrate runtime. It includes functionalities such as exchanging native currency for custom tokens, transferring tokens between accounts, and checking token balances.

### Features
- **Exchange Native to Custom Token**: Allows users to exchange native currency for custom tokens, ensuring sufficient balance checks.
- **Token Transfer**: Facilitates the transfer of custom tokens between accounts, managing balances accordingly.
- **Get Balance**: Enables users to retrieve their current token balance.

### Usage
To integrate this custom token pallet into your Substrate runtime, follow these steps:

1. Add the custom token pallet to your Substrate runtime Cargo.toml file:
    ```toml
    pallet-custom-token = { version = "4.0.0-dev", default-features = false, path = "../pallets/template" }
    ```
2. In the Cargo.toml file, ensure that the custom token pallet is added as a dependency in the feature std by including the line:
    ```toml
   [features]
   default = ["std"]
   std = [
       # other dependencies
       "pallet-custom-token/std"
   ]
    ```
3. Configure the `pallet_custom_token` pallet in the Substrate node runtime as follows:
    ```rust
    impl pallet_custom_token::Config for Runtime {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
    }
    ```
4. Deploy and run your updated Substrate runtime with the custom token pallet included to enable custom token using the following command
    ```bash
    cargo build --release
    ```
5. Run the node using the following command
    ```bash
    ./target/release/node-template --dev
    ```
6. Use the [polkadot.org.js](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/chainstate) for test the functionalities