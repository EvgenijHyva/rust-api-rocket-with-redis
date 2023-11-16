# Run dev with docker

```bash
# The project can be run from folder contained docker-compose
cd cr8s
# this will create the services for dev with volumes
docker-compose up --build
# check
docker-compose ps
docker-compose logs
```

Migrations with diesel ORM

```bash
# APPLY NEW MIGRATIONS
docker-compose exec app diesel migration run
# REVERT MIGRATIONS
docker-compose exec app diesel migration revert

# compile in docker compose & run app
docker-compose exec app cargo run
```

```bash
# returns json body of non existing url '/'
docker-compose exec app curl 127.0.0.1:8000/ -H "Accept: application/json"

# Testing rustaceans url example
docker-compose exec app curl 127.0.0.1:8000/rustaceans
# post
docker-compose exec app curl 127.0.0.1:8000/rustaceans -d '{"name": "Url-test", "email": "test@me.now"}' -H "Content-type: application/json"
# update
docker-compose exec app curl 127.0.0.1:8000/rustaceans/3 -d '{"name": "Updating-test", "email": "test-update@me.now"}' -X PUT -H "Content-type: application/json"
# delete
docker-compose exec app curl 127.0.0.1:8000/rustaceans/1 -X DELETE

# Crates url test examples
docker-compose exec app curl 127.0.0.1:8000/crates
# delete
docker-compose exec app curl 127.0.0.1:8000/crates/2 -X DELETE
```

### Testing

```bash
# tests are contained under tests folder
docker-compose exec app cargo test
```

# Staring project from scratch

```bash
cargo init cr8s
cd cr8s
# diesel (ORM) is installed in container (ref Dockerfile)
docker-compose exec app diesel setup
docker-compose exec app diesel migration generate create_rustaceans
docker-compose exec app diesel migration generate create_crates
```

### Packages in Docker file:

- diesel_cli (with postgres feature, no other needed)
- cargo-watch (keep container running)

# Rust Web development Notes

There is some things that can be used for project just in case

#### Frameworks for web backend

- actix
- axum
- rocket
- warp

#### ORMS

- Diesel
- SeaORM

#### NoSQL

- Redis (cache backend)
- Elasticsearch
- MongoDB
