# erc 20 
 
In this project we have created a 
Erc20 token using ink! tokens. It is the most commonly used token standard.

The Erc20 token standard defines the interface for most of the smart contracts that run on the etherum block chain.

These standars allows indiviual to deploy their cryptocurrency on top of existing platform.
Here we perform different operations like transfer of token from one account to another account and then what are the balances left in both the account after the transfer we can check we have implemented such functions where we can check these all,user balances are mapped with there account address , we have used maps for storing the values.

Intial storage consists of total_supply of tokens and balances which are present in each indiviual account. After that we can transfer tokens from one account to different account we have created a transfer function which helps in doing so , then we  can also check for the balances left in accounts after transfer.


# flipper 

Flipper is a project which helps in when we create a smart contract , flipper generates a smart contract by default it creates a function  flip() when we run this function it will change boolean value of variable from true to false , and there is second function get() , which we get the current value of boolean variable. The lib.rs file contains two function for testing that the contract work as expected.

To explore Project files :
first we have to create a default create a new smart contract that generates a function flip() that will change the boolean vaiable.

1.Open a terminal shell on your computer, if needed.

2.Change to project folder for the flipper smart contract, if needed:

3.Open the Cargo.toml file in a text editor and review the dependencies for the contract.

4.In the [dependencies] section, modify the scale and scale-info settings, if necessary.

scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }

scale-info = { version = "2", default-features = false, features = ["derive"] , optional = true }

5.Save any changes to the Cargo.toml file, then close the file.

6.Open the lib.rs file in a text editor and review the macros, constructors, and functions defined for the contract.

- The storage macro defines a structure to stores a single boolean value for the contract.
The new and default functions initialize the boolean value to false.

- There's a message macro with a flip function to change the state of the data stored for the contract.

- There's a message macro with a get function to get the current state of the data stored for the contract.

# Test the default contract
At the bottom of the lib.rs source code file, there are simple test cases to verify the functionality of the contract. You can test whether this code is functioning as expected using the offchain test environment.

To test the contract:

1.Open a terminal shell on your computer, if needed.

2.Verify that you are in the flipper project folder, if needed.

3.Use the test subcommand to execute the default tests for the flipper contract by running the following command:

**cargo test**

The command should compile the program and display output similar to the following to indicate successful test completion:

running 2 tests
test flipper::tests::it_works ... ok
test flipper::tests::default_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.


# Build the contract

After testing the default contract, you are ready to compile this project to WebAssembly.

To build the WebAssembly for this smart contract:

1.Open a terminal shell on your computer, if needed.

2.Verify that you are in the flipper project folder.

3.Compile the flipper smart contract using the nightly toolchain by running the following command:

cargo +nightly contract build

This command builds a WebAssembly binary for the flipper project, a metadata file that contains the contract Application Binary Interface (ABI), and a .contract file that you use to deploy the contract. For example, you should see output similar to the following:

Original wasm size: 47.8K, Optimized: 22.4K

The contract was built in DEBUG mode.

Your contract artifacts are ready. You can find them in:
/Users/dev-doc/flipper/target/ink

- flipper.contract (code + metadata)
- flipper.wasm (the contract's code)
- metadata.json (the contract's metadata)

The .contract file that includes both the business logic and metadata is the file you use to deploy the contract on a chain.

The metadata.json file in the target/ink directory describes all the interfaces that you can use to interact with this contract. This file contains several important sections:

The spec section includes information about the functions—like constructors and messages—that can be called, the events that are emitted, and any documentation that can be displayed. This section also includes a selector field that contains a 4-byte hash of the function name and is used to route contract calls to the correct functions.

The storage section defines all the storage items managed by the contract and how to access them.

The types section provides the custom data types used throughout the rest of the JSON.

# Start the Substrate smart contracts node
If you have successfully installed substrate-contracts-node, you can start a local blockchain node for your smart contract.

To start the preconfigured contracts-node:

1.Open a new terminal shell on your computer, if needed.

2.Change to the root directory that contains the substrate-contracts-node binary.

For example, if you downloaded the precompiled binary on a macOS computer, you can run the following command:

cd artifacts/substrate-contracts-node-mac

3.Start the contracts node in local development mode by running the following command:

substrate-contracts-node --dev
You should see output in the terminal similar to the following:

2022-03-07 14:46:25 Substrate Contracts Node
2022-03-07 14:46:25 ✌️  version 0.8.0-382b446-x86_64-macos
2022-03-07 14:46:25 ❤️  by Parity Technologies <admin@parity.io>, 2021-2022
2022-03-07 14:46:25 📋 Chain specification: Development
2022-03-07 14:46:25 🏷  Node name: possible-plants-8517
2022-03-07 14:46:25 👤 Role: AUTHORITY
2022-03-07 14:46:25 💾 Database: RocksDb at /var/folders/2_/g86ns85j5l7fdnl621ptzn500000gn/T/substrateEdrJW9/chains/dev/db/full
2022-03-07 14:46:25 ⛓  Native runtime: substrate-contracts-node-100 (substrate-contracts-node-1.tx1.au1)
2022-03-07 14:46:25 🔨 Initializing Genesis block/state (state: 0xe9f1…4b89, header-hash: 0xa1b6…0194)
2022-03-07 14:46:25 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2022-03-07 14:46:26 🏷  Local node identity is: 12D3KooWQ3P8BH7Z1C1ZoNSXhdGPCiPR7irRSeQCQMFg5k3W9uVd
2022-03-07 14:46:26 📦 Highest known block at #0
After a few seconds, you should see blocks being finalized.

To interact with the blockchain, you need to connect to this node. You can connect to the node through a browser by opening the Contracts UI.

4.Navigate to the Contracts UI in a web browser, then click Yes allow this application access.

5.Select Local Node.

Connect to the local node

# Deploy the contract

At this point, you have completed the following steps:

Installed the packages for local development.
Generated the WebAssembly binary for the flipper smart contract.
Started the local node in development mode.
Connected to a local node through the Contracts UI front-end.
The next step is to deploy the flipper contract on your Substrate chain.

However, deploying a smart contract on Substrate is a little different than deploying on traditional smart contract platforms. For most smart contract platforms, you must deploy a completely new blob of the smart contract source code each time you make a change. For example, the standard ERC20 token has been deployed to Ethereum thousands of times. Even if a change is minimal or only affects some initial configuration setting, each change requires a full redeployment of the code. Each smart contract instance consumes blockchain resources equivalent to the full contract source code, even if no code was actually changed.

In Substrate, the contract deployment process is split into two steps:

Upload the contract code to the blockchain.

Create an instance of the contract.
With this pattern, you can store the code for a smart contract like the ERC20 standard on the blockchain once, then instantiate it any number of times. You don't need to reload the same source code repeatedly, so your smart contract doesn't consume unnecessary resources on the blockchain.

# Upload the contract code
For this tutorial, you use the Contracts UI front-end to deploy the flipper contract on the Substrate chain.

To upload the smart contract source code:

1.Open to the Contracts UI in a web browser.

2.Verify that you are connected to the Local Node.

3.Click Add New Contract.

4.Click Upload New Contract Code.

5.Select an Account to use to create a contract instance.

You can select any existing account, including a predefined account such as alice.

6.Type a descriptive Name for the smart contract, for example, Flipper Contract.

7.Browse and select or drag and drop the flipper.contract file that contains the bundled Wasm blob and metadata into the upload section.

Upload the contract

8.Click Next to continue.

# Create an instance on the blockchain
Smart contracts exist as an extension of the account system on the Substrate blockchain. When you create an instance of this smart contract, Substrate creates a new AccountId to store any balance managed by the smart contract and to allow you to interact with the contract.

After you upload the smart contract and click Next, the Contracts UI displays information about the content of the smart contract.

To create the instance:

1.Review and accept the default Deployment Constructor options for the initial version of the smart contract.

2.Review and accept the default Max Gas Allowed.

Create an instance of the smart contract

3.Click Next.

The transaction is now queued. If you needed to make changes, you could click Go Back to modify the input.

Complete instantiation

4.Click Upload and Instantiate.

Depending on the account you used, you might be prompted for the account password. If you used a predefined account, you won't need to provide a password.

Successfully deployed instance of the smart contract

# Call the smart contract
Now that your contract has been deployed on the blockchain, you can interact with it. The default flipper smart contract has two functions—flip() and get()—and you can use the Contracts UI to try them out.

get() function
You set the initial value of the flipper contract value to false when you instantiated the contract. You can use the get() function to verify the current value is false.

To test the get() function:

1.Select any account from the Account list.

This contract doesn't place restrictions on who is allowed to send the get() request.

2.Select get(): bool from the Message to Send list.

3.Click Read.

4.Verify that the value false is returned in the Call Results.

Calling the get() function returns false

# flip() function
The flip() function changes the value from false to true.

To test the flip() function:

1.Select any predefined account from the Account list.

The flip() function is a transaction that changes the chain state and requires an account with funds to be used to execute the call. Therefore, you should select an account that has a predefined account balance, such as the alice account.

2.Select flip() from the Message to Send list.

3.Click Call.

4.Verify that the transaction is successful in the Call Results.

Successful transaction

5.Select get(): bool from the Message to Send list.

6.Click Read.

7.Verify the new value is true in the Call Results.

The get() function displays the current value is true




#   incrementer 

In this project what we do is we created a function which will store the value of the variable which we give it to , first we will give it a default value to 0 , after then we can increase or decrease the value accordingly , and can store it.

we have created a function MyContract() where it will contain the value we have to give to the variable and then we have added a function that will modify the storage value accordingly. 



