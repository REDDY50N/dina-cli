#!/bin/bash

readonly REPO_ROOT="$(git rev-parse --show-toplevel)"

echo "DATABASE_URL=sqlite:${REPO_ROOT}/foody.db" > ${REPO_ROOT}/.env

# Create DB and use migrations
cargo install sqlx-cli

# Use .env file database path
sqlx database create
sqlx migrate run --source ${REPO_ROOT}/db/migrations

# Set database manually for tests
#sqlx database create --database-url "sqlite:${REPO_ROOT}/foody.db"
#sqlx migrate run --source ${REPO_ROOT}/db/migrations --database-url "sqlite:${REPO_ROOT}/foody.db"

