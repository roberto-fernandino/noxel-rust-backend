# noxel-rust-backend

Axum + SQLx (Postgres) + Tokio runtime + Utoipa OpenAPI.

## Local (Docker)

```bash
docker compose up --build
```

- API: http://localhost:8080
- Health: http://localhost:8080/health
- Swagger UI: http://localhost:8080/docs
- OpenAPI JSON: http://localhost:8080/api-doc/openapi.json

## Database migrations

This repo includes SQL migrations under `./migrations`.

Recommended workflow:
- Run migrations using `sqlx migrate run` (requires `sqlx-cli` installed)

Example (once you have Rust toolchain + sqlx-cli):
```bash
export DATABASE_URL=postgres://noxel:noxel@localhost:5432/noxel
sqlx migrate run
```
