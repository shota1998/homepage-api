#!/usr/bin/env bash

# Enable IAM database authentication.
# Define a value for each variable based on your own environment.
# I recommend yuo not to hard code in this script.

aws rds modify-db-instance \
    --db-instance-identifier $MY_AWS_RDS_DB_INSTANCE_IDENTIFIER \
    --apply-immediately \
    --enable-iam-database-authentication