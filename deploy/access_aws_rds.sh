#!/usr/bin/env bash

# Create a signed IAM authentication token.
# Define a value for each variable based on your own environment.
# I recommend yuo not to hard code in this script.

# MY_AWS_RDS_HOST      : RDS Instanece name. E.g. XXX.rds.amazonaws.com.
# MY_AWS_PORT          : Port which your DB is opening.
# MY_AWS_DATABASE_NAME : DB name which you named when you created DB.
# MY_AWS_IAM_USER      : IAM user name which you use when execute this script.

TOKEN="$(aws rds generate-db-auth-token \
                     --hostname $MY_AWS_RDS_HOST \
                     --port $MY_AWS_PORT \
                     --region $MY_AWS_REGION \
                     --username $MY_AWS_IAM_USER
                     )"

psql "
    host        = $MY_AWS_RDS_HOST
    port        = $MY_AWS_PORT
    sslrootcert = global-bundle.pem
    sslmode     = verify-full
    dbname      = $MY_AWS_DATABASE_NAME
    user        = $MY_AWS_IAM_USER
    password    = $TOKEN
  "