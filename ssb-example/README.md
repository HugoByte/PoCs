# Kuska - ssb
secure scuttlebutt 

kuska is a implementation of secure scuttlebutt  network in rust.

## Building 
cargo build 

1. Install the ssb-server 
    ``` 
    npm install -g ssb-server
    ```

2. Start the server
    ``` 
    ssb-server start
    ```
    After starting the server  you will get a public key which will be further used while running the ssb for connection.
3. Clone this Repository 
4. Run 
   ```
   cargo run -- --connect Ip Address:Port No : Public key
   ```
   Example
   ``` 
   cargo run -- --connect 127.0.0.1:8008:<public key without .ed25519>
   ```