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

# Staring project from scratch

```bash
cargo init cr8s
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
