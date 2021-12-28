#!/usr/bin/env bash

# ******************************************************
# Move to the directory where needed files are located.
# ******************************************************
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P)"
cd $SCRIPTPATH

# ***********************
# Copy local file to EC2.
# ***********************
echo -e "\n----- Start  : Copy local file. -----\n"
scp -i "./rust_app.pem" \
    ./docker-compose.yml ${EC2_ADDRESS_WITH_USER}:/home/ec2-user/docker-compose.yml
scp -i "./rust_app.pem" \
    -r ./nginx ${EC2_ADDRESS_WITH_USER}:/home/ec2-user

# ***********************
# Execute command at EC2.
# ***********************

echo -e  "\n----- Start  : Execute command at EC2. -----\n"
# ssh -i "./rust_app.pem"  -t  ec2-user@54.178.7.242 << EOF
  
#   docker-compose up -d
#   docker container exec \
#     -t rust_app bash \
#     -c "echo DATABASE_URL=postgres://username:password@todo.ccuf26pumepg.ap-northeast-1.rds.amazonaws.com/to_do > .env"
#   docker container exec \
#     -t rust_app diesel migration run

#   rm -r nginx/
#   rm docker-compose.yml
# EOF
ssh -i "./rust_app.pem" -t ${EC2_ADDRESS_WITH_USER} << EOF
  sudo service docker start
  docker-compose up -d
  docker container exec \
    -t rust_app bash \
    -c "echo DATABASE_URL=postgres://username:password@todo.ccuf26pumepg.ap-northeast-1.rds.amazonaws.com/to_do > .env"
  docker container exec \
    -t rust_app diesel migration run

  rm -r nginx/
  rm docker-compose.yml
EOF

# *****************
# Create test user. 
# *****************
echo -e "\n----- Start : Create test user. -----\n"
curl --header  "Content-Type: application/json" \
     --request POST \
     --data    '{"name":"test", "email":"test", "password": "test"}' \
               ${MY_AWS_EC2_ADDRESS}/api/v1/user/create

# docker-compose stop 
#   docker container rm rust_app
#   docker image rm chan1998/actix_web_application