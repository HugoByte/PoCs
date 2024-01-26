def run(plan):
    plan.add_service(
        name = "decentralwiz",
        config = ServiceConfig(
            image = ImageBuildSpec(
                image_name = "hugobyte/decentralwiz",
                build_context_dir = ".",
            ),
        ),
    )
