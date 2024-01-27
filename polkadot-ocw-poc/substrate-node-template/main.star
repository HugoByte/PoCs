def run(plan, node_type = "conduit"):
    plan.add_service(
        name = "polkadot-ocw-poc",
        config = ServiceConfig(
            image = "hugobyte/polkadot-ocw-poc",
            ports = {
                "http": PortSpec(number = 9944),
            },
        ),
    )
