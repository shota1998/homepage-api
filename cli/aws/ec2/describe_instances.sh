#!/usr/bin/env bash

# Exaple input.
# aws ec2 describe-instances | jq '.Reservations[].Instances[] | {InstanceId, InstanceType, PrivateIpAddress}'
#
# Exaple output.
# {
#   "InstanceId": "i-089f6a0a0e2f413da",
#   "InstanceType": "t2.micro",
#   "PrivateIpAddress": "172.31.35.37"
# }

JSON_PARENT_KEY='.Reservations[].Instances[]'
JSON_CHILD_KEY='{
                  InstanceId,
                  KeyName,
                  PrivateIpAddress,
                  PublicIpAddress,
                  State
                }'

# if $1
#   JSON_PARENT_KEY=$1
# fi
# if $2
#  JSON_CHILD_KEY=$2
# fi

aws ec2 describe-instances | jq "$JSON_PARENT_KEY | $JSON_CHILD_KEY"