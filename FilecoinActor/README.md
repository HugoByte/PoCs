# FileCoin HelloWorld Actor

PoC for create an actor and deploy the actor in Filecoin Network.

Step by steps to run the node and deploy the actor and onvoking. In this node running inside the docker container.

## Steps

### Run the Node
``` 
docker run -p 1234:1234 -i -t --rm --name lotus-fvm-localnet ghcr.io/jimpick/lotus-fvm-localnet-lite:latest lotus daemon --lotus-make-genesis=devgen.car --genesis-template=localnet.json --bootstrap=false  
```

### Run the Minor
In another Terminal run the monor node 
```
docker exec -i -t lotus-fvm-localnet lotus-miner run --nosync
```

You can watch the chain progress 
```
docker exec -it lotus-fvm-localnet watch lotus chain list --count=3
```

## Build the WASM 

1. Clone this Reposotory 
2. Build the it
    ```
    Cargo build --release
    ```
3. The wasm will be in 
   ``` 
    ./target/release/wbuild/filecoin_hello_world_actor/filecoin_hello_world_actor.compact.wasm ```

## Deploy to the Filecoin 
1. Connect to the local net
    - Before that need to expose the ip in docker container 
        - Open the config.toml file present inside the ./lotus folder in container 
        - Uncomment the ListenAddress to 0.0.0.0
            eg: ListenAddress = "/ip4/0.0.0.0/tcp/1234/http"
        - Restart the Node
    - Copy the token , token present in the toke fin inside the .lotus folder.
    - Export the API
      eg: export FULLNODE_API_INFO="<token>:/ip4/127.0.0.1/tcp/8088/http".

2. Install the Actor code 
    ```
    lotus chain install-actor <wasm-file> 
    ```
    You will get a code-CID, Note this code-cid
3. Creat Actor Instance
    ```
    lotus chain create-actor <code-cid>
    ```
    In this you will get ID Address, note this actor address id for invoking.
4. Invoke the Actor
    ```
    lotus chain invoke <actor-id-address> <method-number>
    ```
    Eg : 
    lotus chain invoke t1001 2


