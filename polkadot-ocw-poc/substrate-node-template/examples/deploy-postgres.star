postgres = import_module("github.com/hugobyte/postgres-package/main.star")

def run(plan):
    plan.render_templates(
        config = {
            "seed.sql": struct(
                template = """
                CREATE TABLE service_names (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL
                );

                INSERT INTO service_names (name) VALUES ('Service A');
                INSERT INTO service_names (name) VALUES ('Service B');
                INSERT INTO service_names (name) VALUES ('Service C');
                """,
                data = "",
            ),
        },
        name = "seed-file-artifact",
    )

    postgres_output = postgres.run(plan, image = "postgres:15.2-alpine", user = "postgres", service_name = "postgres", seed_file_artifact_name = "seed-file-artifact")

    plan.render_templates(
        config = {
            "service.json": struct(
                template = """
                    {
                        "service_hostname": "{{.service_hostname}}",
                        "user": "{{.user}}",
                        "password": "{{.password}}",
                        "database": "{{.database}}"
                    }
                """,
                data = {
                    "service_hostname": "{0}".format(postgres_output.service.hostname),
                    "user": "{0}".format(postgres_output.user),
                    "password": "{0}".format(postgres_output.password),
                    "database": "{0}".format(postgres_output.database),
                },
            ),
        },
        name = "service-info-artifact",
    )

    return postgres_output
