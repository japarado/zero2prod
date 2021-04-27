#!/usr/bin/env zsh
set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'pam'
DB_USER=${POSTGRES_USER:=pam}
# Check if a custom password has been set, otherwise default to 'pam'
DB_PASSWORD=${POSTGRES_PASSWORD:=pam}
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME=${POSTGRES_DB:=newsletter}
# Check if a custom port has been set, otherwise default to 5432
DB_PORT=${POSTGRES_PORT:=5432}

# Launch postgres using docker
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

#echo >&2 "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
