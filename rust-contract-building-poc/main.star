def run(plan):

    plan.upload_files(
        src="nameservice",
        name = "contracts"
    )

    service_config = ServiceConfig(
        image="cosmwasm/optimizer:0.15.0",
        files= {
            "/code": "contracts"
        },
        entrypoint= ["/bin/sh"]
    )

    plan.add_service(name="rustcontractbuilder", config=service_config)

    plan.exec(service_name = "rustcontractbuilder", recipe= ExecRecipe(["optimize.sh", "."]))

    plan.store_service_files(service_name="rustcontractbuilder", src="/code/artifacts", name="artifacts")

    
