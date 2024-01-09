def run(plan):

    plan.upload_files(
        src = "./build.gradle",
        name = "build-gradle"
    )

    plan.upload_files(
        src = "./app/build.gradle",
        name = "app-build-gradle"
    )

    service_details = plan.add_service(
        name = "gradle",
        config = ServiceConfig(
            image = "gradle:latest", 
            files = {
                "/temp/build" : "build-gradle",
                "/temp/app/build" : "app-build-gradle"
            }, 
            entrypoint = ["/bin/sh"]
        ),
    )

    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","mkdir /java-contract"]
        ),
    )

    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","cd /java-contract && gradle init --type java-application --dsl groovy --package java.contract --project-name java-contract --test-framework junit-jupiter --no-split-project"]
        ),
    )

    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","rm /java-contract/app/build.gradle"]
        ),
    )

    plan.exec(
        service_name="gradle",
        recipe=ExecRecipe(
            command=[ "/bin/sh", "-c", "cp -r /temp/app/build/* /java-contract/app"],
        ),
    )

    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","cp -r /temp/build/* /java-contract"]
        ),
    )

    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","cd /java-contract && ./gradlew build"]
        ),
    )

    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","cd /java-contract && ./gradlew test"]
        )
    )
    plan.exec(
        service_name = "gradle",
        recipe = ExecRecipe(
            command=["/bin/sh","-c","cd /java-contract && ./gradlew optimizedJavaJar"]
        ),
    )
 
    plan.store_service_files(
        service_name="gradle", 
        src = "/java-contract/app/build", 
        name="contract_artifacts"
    )

    return service_details