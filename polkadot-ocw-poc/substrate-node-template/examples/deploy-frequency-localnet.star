package = import_module("github.com/hugobyte/polkadot-kurtosis-package/main.star@056279d021b492f65fba360d2d22bb2421339653")

def run(plan):
    relaychain = {
        "name": "rococo-local",
        "nodes": [
            {
                "name": "alice",
                "node_type": "validator",
                "prometheus": False,
            },
            {
                "name": "bob",
                "node_type": "validator",
                "prometheus": False,
            },
        ],
    }

    parachains = [
        {
            "name": "frequency",
            "nodes": [
                {
                    "name": "alice",
                    "node_type": "validator",
                    "prometheus": False,
                },
                {
                    "name": "bob",
                    "node_type": "full",
                    "prometheus": True,
                },
            ],
        },
    ]

    package.run(plan, "localnet", relaychain, parachains, True, True)
