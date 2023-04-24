#!/bin/bash
docker compose down
docker compose up --build
#Install Postgres DB
# cargo install diesel_cli --no-default-features --features postgres
# diesel setup --database-url postgres://postgres:simong@localhost:5416/registration
# diesel migration run --database-url postgres://postgres:simong@localhost:5416/registration