Steps

1. Setup Validator authourity
```
kurtosis run github.com/hugobyte/pocs/polkadot-ocw-poc/substrate-node-template '{ "node_type": "validator", "node_args": { "seed": "[seed phrase]" } }'
```

2. Get Cluster IP of Node 1
```
kubectl describe svc polkadot-ocw-poc -n kt-[Enclave of Node 1]
```

3. Get Peer Id of first node
```
kubectl logs polkadot-ocw-poc -n kt-[Enclave of Node 1]
```

4. Deploy 2nd Validator Node with bootnode info
```
kurtosis run github.com/hugobyte/pocs/polkadot-ocw-poc/substrate-node-template '{ "node_type": "validator", "node_args": { "seed": "[seed phrase]" }, "bootnodes": "/ip4/[cluster ip of node 1]/tcp/30333/p2p/[peer id of node 1]" }'
```

5. Get Namespace of kurtosis engine
```
kubectl get ns
```
6. Get Cluster IP of engine
```
kubectl describe svc kurtosis-engine-[uuid of engine] -n kurtosis-engine-[uuid of engine]
```

6. Deploy Provider

```
kurtosis run github.com/hugobyte/pocs/polkadot-ocw-poc/substrate-node-template '{ "node_type": "provider", "node_args": { "seed":"[seed phrase]", "engine_host": "https://[cluster ip of engine]:9710"}, "bootnodes": "/ip4/[cluster ip of node 1]/tcp/30333/p2p/[peer id of node 1]" }'
```

7. Get Cluster Ip of Provider
```
kubectl describe svc polkadot-ocw-poc -n kt-[Enclave of provider]
```
8. Set Providers Endpoint using RPC
```
{
  "jsonrpc": "2.0",
  "method": "template_setPublicEndpoint",
  "params": ["http://[cluster ip of provider]:9944"],
  "id": 1
}
```
9. Add bootnodes using RPC
```
{
  "jsonrpc": "2.0",
  "method": "template_setBootnodes",
  "params": ["/ip4/[cluster ip of node 1]/tcp/30333/p2p/[peer id of node 1]"],
  "id": 1
}
```