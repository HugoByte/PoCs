def run(plan, args):
    plan.upload_files(
        src="./foundry-template",
        name="foundry-template"
    )

    plan.add_service(
        name = "foundry",
        config = ServiceConfig(
            image = "ghcr.io/foundry-rs/foundry:latest", 
            files = {
                "/temp/foundry-template" : "foundry-template"
            }, 
            entrypoint = ["/bin/sh"]
        ),
    )

    plan.exec(
        service_name = "foundry",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","cd /temp/foundry-template && forge install"]
        ),
    )

    build_details = plan.exec(
        service_name = "foundry",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","forge build"]
        ),
    )
    return build_details