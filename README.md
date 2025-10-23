

# Local development

#### Before you can build the project, SQLX needs a local database set up

- Start DB only with ports
```shell 
podman-compose run --publish 5432:5432 db
```

- Apply migrations to initialize the DB using [SQLX Cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
```shell
sqlx migrate run
```

