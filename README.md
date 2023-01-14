# cosmwasm-ibc

IBC implementation in CW contracts.

This is a simple IBC enabled CosmWasm smart contract. It expects to be deployed on two chains and will send messages to its counterpart. It then counts the number of times messages have been received on both sides.

### At a high level, to use this contract

1. Store and instantiate the contract on two IBC enabled chains. We will call these chains chain A and chain B.

2. Configure and run a relayer to connect the two contracts.

3. Execute the `Increment {}` method on one contract to increment the send a message and increment the count on the other one.

4. Use the `GetCount { connection }` query to determine the message count for a given connection.

## Background

To connect two CosmWasm contracts over IBC you must establish an IBC channel between them. The IBC channel establishment process uses a four way handshake. Here is a summary of the steps:

1. `OpenInit` Hello chain B, here is information that you can use to verify I am chain A. Do you have information I can use?

2. `OpenTry` Hello chain A, I have verified that you are who you say you are. Here is my verification information.

3. `OpenAck` Hello chain B. Thank you for that information I have verified you are who you say you are. I am now ready to talk.

4. `OpenConfirm` Hello chain A. I am also now ready to talk.

Once the handshake has been completed a channel will be established that the ibc messages may be sent over. In order to do a handshake and receive IBC messages your contract must implement the following entry points (see `src/ibc.rs`):

1. `ibc_channel_open` - Handles the `OpenInit` and `OpenTry` handshake steps.

2. `ibc_channel_connect` - Handles the `OpenAck` and `OpenConfirm` handshake steps.

3. `ibc_channel_close` - Handles the closing of an IBC channel by the counterparty.

4. `ibc_packet_receive` - Handles receiving IBC packets from the counterparty.

5. `ibc_packet_ack` - Handles ACK messages from the countarparty. This is effectively identical to the ACK message type in [TCP](https://developer.mozilla.org/en-US/docs/Glossary/TCP_handshake).

6. `ibc_packet_timeout` - Handles packet timeouts.

Having implemented these methods, once you instantiate an instance of the contract it will be assigned a port. Ports identify a receiver on a blockchain in much the same way as ports identify applications on a computer.

You can find the port that has been assigned to your contract by running
`wam query wasm contract <ADDRESS>` and inspecting the `ibc_port_id` field. For example:

```
archwayd query wasm contract archway1wt4l7uh25x009vgky052xxss95hjnw5z9vy8m396mz8tyd384jgsugz0dh --node <https://rpc.constantine-1.archway.tech:443>

address: archway1wt4l7uh25x009vgky052xxss95hjnw5z9vy8m396mz8tyd384jgsugz0dh

contract_info:

admin: ""

code_id: "301"

created: null

creator: archway12yzhvucd7f7008ua7saep09kx6zl2wdax5lwfc

extension: null

ibc_port_id: wasm.archway1wt4l7uh25x009vgky052xxss95hjnw5z9vy8m396mz8tyd384jgsugz0dh

label: ics20

```

## RELAYER SETUP

To establish a connecton between two contracts you will need to setup a relayer.

Using [Ignite][https://docs.ignite.com/] relayer :

1. Import the account which are having funds that can be used in relayer.

   `ignite account import [account_name]`

2. Configure the relayer by providing all the flags.

```
ignite relayer configure -a \
--source-rpc "https://rpc.constantine-1.archway.tech:443" \
--source-faucet "https://faucet.constantine-1.archway.tech" \
--source-port "wasm.archway1wt4l7uh25x009vgky052xxss95hjnw5z9vy8m396mz8tyd384jgsugz0dh" \
--source-version "ics20-1" \
--source-gasprice "0.25uconst" \
--source-prefix "archway" \
--source-gaslimit 300000 \
--target-rpc "https://rpc.malaga-420.cosmwasm.com:443" \
--target-faucet "https://faucet.malaga-420.cosmwasm.com" \
--target-port "wasm.wasm1vxu6js9d3vvmyltmy9lc9jr96ul5a3vk2u2f9uzp3r4dvx4mu4mq33ltxd" \
--target-version "ics20-1" \
--target-gasprice "0.25umlg" \
--target-prefix "wasm" \
--target-gaslimit 300000
```

3. Provide the source and target account imported.

```
 ------
 Setting up chains
 ------

 ? Source Account hermisarchway
 ? Target Account hermiscosmos

 🔐  Account on "source" is hermisarchway(archway1vm62p5n37j5ffrwr5mgnltdmgxcax0frcpav8q)

  |· faucet is not operational: Internal Server Error
  |· (balance: -)

 🔐  Account on "target" is hermiscosmos(wasm134pggaja9zu52mdfdv4pykn5g6j5m47lfn72yd)

  |· faucet is not operational: invalid character 'W' looking for beginning of value
  |· (balance: -)

 ⛓  Configured chains: constantine-1-malaga-420
```

4. Start the relayer by using command:
   `ignite relayer connect`

```
 ------
Paths
------

constantine-1-malaga-420:
    constantine-1 > (port: wasm.archway1wt4l7uh25x009vgky052xxss95hjnw5z9vy8m396mz8tyd384jgsugz0dh) (channel: channel-8)
    malaga-420    > (port: wasm.wasm1vxu6js9d3vvmyltmy9lc9jr96ul5a3vk2u2f9uzp3r4dvx4mu4mq33ltxd)    (channel: channel-13)

------
Listening and relaying packets between chains...
------

Relay 1 packets from malaga-420 => constantine-1
Relay 1 acks from constantine-1 => malaga-420
Relay 1 packets from constantine-1 => malaga-420
Relay 1 acks from malaga-420 => constantine-1
Relay 1 packets from malaga-420 => constantine-1
Relay 1 acks from constantine-1 => malaga-420
```
