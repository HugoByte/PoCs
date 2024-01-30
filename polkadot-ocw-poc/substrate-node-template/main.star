def run(plan, node_type = "conduit", node_args = None, bootnodes = None):
    commands = ["--rpc-cors", "all", "--chain", "/data/customSpec.json", "--unsafe-rpc-external", "--" + node_type]

    if bootnodes != None:
        commands.extend([
            "--bootnodes",
            bootnodes,
        ])

    artifact_name = plan.upload_files(
        src = "github.com/hugobyte/pocs/polkadot-ocw-poc/substrate-node-template/customSpec.json",
        name = "chain-spec",
    )

    if node_type == "conduit":
        if node_args == None or "request_id" not in node_args:
            plan.print("Error: For conduit node_type, node_args must include 'request_id'.")
            return

        enclave_port = node_args.get("enclave_port", 9774)
        commands.extend([
            "--request-id",
            str(node_args["request_id"]),
            "--enclave-port",
            str(enclave_port),
            "--provider-url",
            str(node_args["provider_url"]),
            "--offchain-worker",
            "always",
        ])

    if node_type == "provider":
        enclave_port = node_args.get("enclave_port", 9710)
        commands.extend([
            "--engine-host",
            str(node_args["engine_host"]),
            "--enclave-port",
            str(enclave_port),
            "--offchain-worker",
            "always",
        ])

    plan.add_service(
        name = "polkadot-ocw-poc",
        config = ServiceConfig(
            image = "hugobyte/polkadot-ocw-poc:0.1.0",
            ports = {
                "ws": PortSpec(9944, transport_protocol = "TCP"),
                "lib2lib": PortSpec(30333, transport_protocol = "TCP"),
            },
            cmd = commands,
            files = {
                "/data": "chain-spec",
            },
        ),
    )

    if node_type == "validator":
        plan.exec(
            service_name = "polkadot-ocw-poc",
            recipe = ExecRecipe(
                command = ["/usr/local/bin/node-template", "key", "insert", "--scheme", "Sr25519", "--suri", "{0}".format(node_args["seed"]), "--key-type", "aura", "--chain", "/data/customSpec.json"],
            ),
        )

        plan.exec(
            service_name = "polkadot-ocw-poc",
            recipe = ExecRecipe(
                command = ["/usr/local/bin/node-template", "key", "insert", "--scheme", "Ed25519", "--suri", "{0}".format(node_args["seed"]), "--key-type", "gran", "--chain", "/data/customSpec.json"],
            ),
        )

    if node_type == "provider":
        plan.exec(
            service_name = "polkadot-ocw-poc",
            recipe = ExecRecipe(
                command = ["/usr/local/bin/node-template", "key", "insert", "--scheme", "Sr25519", "--suri", "{0}".format(node_args["seed"]), "--key-type", "demo"],
            ),
        )
