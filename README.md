# Accessing postgresql with rust

This repo represents an example of creating a postgreSQL database in docker and accessing it via a rust application found in test_postgres/. Currently the rust code just creates some tables.

# Developing
- To continue writing the rust application, you can use a devcontainer with the vscode devcontainers extension. Configuration is in the .devcontainer folder.

# Setup
- Make .env files in the root directory (for docker compose) and in the test_postgres directory (for the rust app). Use the following template:

```env
DB_PASSWORD=...
DB_USER=...
DB_NAME=...
```

# Using
```sh
docker compose up --build -d
```
- Postgres is accessible on port 5432

# Accessing postgres from command line
How to submit queries yourself from the command line
```sh
docker exec -it phys_db bash
psql -U <username> <dbname>
```

How to extract the db into current (local) directory:
```sh
docker exec -t phys_db pg_dump -U john9francis modelmyphysics > modelmyphysics.sql
```

# Useful websites
- [postgres docker compose](https://medium.com/@agusmahari/docker-how-to-install-postgresql-using-docker-compose-d646c793f216)
- [Rust with postgres](https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html)
- [Client::connect args](https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.connect)
- [Client::connect config](https://docs.rs/postgres/0.17.2/postgres/config/struct.Config.html)