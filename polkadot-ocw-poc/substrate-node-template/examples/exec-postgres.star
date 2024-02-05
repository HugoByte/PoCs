postgres = import_module("github.com/hugobyte/postgres-package/main.star")

def run(plan):
    service_hostname = plan.run_sh(
        run = "jq -r '.service_hostname' /data/service.json | tr -d '\n'",
        files = {
            "/data/": "service-info-artifact",
        },
    )

    user = plan.run_sh(
        run = "jq -r '.user' /data/service.json | tr -d '\n'",
        files = {
            "/data/": "service-info-artifact",
        },
    )

    password = plan.run_sh(
        run = "jq -r '.password' /data/service.json | tr -d '\n'",
        files = {
            "/data/": "service-info-artifact",
        },
    )

    database = plan.run_sh(
        run = "jq -r '.database' /data/service.json | tr -d '\n'",
        files = {
            "/data/": "service-info-artifact",
        },
    )

    service = struct(
        name = "postgres",
        hostname = service_hostname.output,
    )

    query = postgres.run_query(
        plan,
        service,
        user.output,
        password.output,
        database.output,
        """
            SELECT json_agg(t) FROM (SELECT * FROM service_names) t;
        """,
    )

    return query["output"]
