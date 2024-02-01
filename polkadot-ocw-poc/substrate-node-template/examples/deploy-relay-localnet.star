package = import_module("github.com/hugobyte/polkadot-kurtosis-package/main.star")

def run(plan):
    relaychain = {
        "name": "rococo",
        "nodes": [
            {
                "name": "alice",
                "node_type": "validator",
                "prometheus": False,
            },
            {
                "name": "bob",
                "node_type": "validator",
                "prometheus": True,
            },
        ],
    }

    package.run(plan, "localnet", relaychain, [], True)
