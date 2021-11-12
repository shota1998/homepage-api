#!/usr/bin/env bash

DB_INSTANCE_IDENTIFIER=$1
JSON_PARENT_KEY='.DBInstances[]'
JSON_CHILD_KEY='{
                  DBInstanceIdentifier,
                  DBInstanceStatus,
                  MasterUsername,
                  Endpoint
                }'

# if $1 exist
#
# aws rds describe-db-instances \
#   --db-instance-identifier $DB_INSTANCE_IDENTIFIER | \
#     jq "$JSON_PARENT_KEY | $JSON_CHILD_KEY"
#
#else

aws rds describe-db-instances \
  | jq "$JSON_PARENT_KEY | $JSON_CHILD_KEY"