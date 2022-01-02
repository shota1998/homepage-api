#!/usr/bin/env bash

DB_INSTANCE_IDENTIFIER=$1

aws rds start-db-instance \
  --db-instance-identifier $DB_INSTANCE_IDENTIFIER && \
aws rds wait db-instance-available \
  --db-instance-identifier $DB_INSTANCE_IDENTIFIER