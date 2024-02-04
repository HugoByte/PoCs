def run(plan, node_type = "conduit", node_args = None, bootnodes = None):
    commands = ["--rpc-cors", "all", "--chain", "/data/customSpec.json", "--unsafe-rpc-external", "--" + node_type]

    if bootnodes != None:
        commands.extend([
            "--bootnodes",
            bootnodes,
        ])

    artifact_name = plan.upload_files(
        src = "./customSpec.json",
        name = "chain-spec",
    )

    if node_type == "conduit":
        if node_args == None or "request_id" not in node_args:
            plan.print("Error: For conduit node_type, node_args must include 'request_id'.")
            return

        commands.extend([
            "--request-id",
            str(node_args["request_id"]),
            "--provider-url",
            str(node_args["provider_url"]),
            "--api-container-host",
            str(node_args["api_container_host"]),
            "--offchain-worker",
            "always",
        ])

    if node_type == "provider":
        commands.extend([
            "--engine-host",
            str(node_args["engine_host"]),
            "--offchain-worker",
            "always",
        ])

    service = plan.add_service(
        name = "polkadot-ocw-poc",
        config = ServiceConfig(
            image = "hugobyte/polkadot-ocw-poc:0.2.4",
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

    if node_type == "conduit":
        plan.exec(
            service_name = "polkadot-ocw-poc",
            recipe = ExecRecipe(
                command = ["/bin/sh", "-c", "/usr/local/bin/node-template key generate --scheme Sr25519 --output-type json > /data/key.json"],
            ),
        )

        plan.store_service_files(
            service_name = "polkadot-ocw-poc",
            src = "/data/key.json",
            name = "keys-artifact",
        )

        result = plan.run_sh(
            run = "jq -r '.secretPhrase' /data/key.json | tr -d '\n'",
            files = {
                "/data": "keys-artifact",
            },
        )

        plan.exec(
            service_name = "polkadot-ocw-poc",
            recipe = ExecRecipe(
                command = ["/usr/local/bin/node-template", "key", "insert", "--scheme", "Sr25519", "--suri", result.output, "--key-type", "demo", "--chain", "/data/customSpec.json"],
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
