#!/bin/bash
#Install Postgres DB
diesel setup --database-url postgres://postgres:simong@postgres_db:5432/registration
diesel migration run --database-url postgres://postgres:simong@postgres_db:5432/registration