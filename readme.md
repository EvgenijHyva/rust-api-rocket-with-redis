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
