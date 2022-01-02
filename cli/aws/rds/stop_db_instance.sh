#!/usr/bin/env bash

INSTANCE_IDS=$1

aws rds stop-db-instance \
  --db-instance-identifier $INSTANCE_IDS