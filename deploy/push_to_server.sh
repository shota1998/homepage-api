#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P)"
cd $SCRIPTPATH

scp -i "./rust_app.pem" ./docker-compose.yml ec2-user@54.178.7.242:/home/ec2-user/docker-compose.yml
scp -i "./rust_app.pem"  -r ./nginx ec2-user@54.178.7.242:/home/ec2-user

scp -i "./rust_app.pem"  -t  ec2-user@54.178.7.242 << EOF
  docker-compose up -d
  docker container exec -t rust_app diesel migration run
    rm -r nginx/
    rm docker-compose.yml

EOF

#curl --header "Content-Type: application/json" --request POST --data '{"name":"test", "email":"test", "password": "test"}' 54.178.7.242/api/v1/user/create
