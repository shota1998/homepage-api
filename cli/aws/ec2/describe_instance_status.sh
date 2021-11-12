#!/usr/bin/env bash

# Exapmle input.
# aws ec2 describe-instance-status \
#   --instance-ids i-089f6a0a0e2f413da | jq '.InstanceStatuses[] | {InstanceId, InstanceState: .InstanceState.Name, SystemStatus: .SystemStatus.Status, InstanceStatus: .InstanceStatus.Status}'
#
# Exapmle output.
# {
#   "InstanceId": "i-089f6a0a0e2f413da",
#   "InstanceState": "running",
#   "SystemStatus": "initializing",
#   "InstanceStatus": "initializing"
# }

INSTANCE_IDS=$1
JSON_PARENT_KEY='.InstanceStatuses[]'
JSON_CHILD_KEY='{
                  InstanceId, 
                  InstanceState: .InstanceState.Name, 
                  SystemStatus: .SystemStatus.Status, 
                  InstanceStatus: .InstanceStatus.Status
                }'

aws ec2 describe-instance-status \
  --instance-ids $INSTANCE_IDS | jq "$JSON_PARENT_KEY | $JSON_CHILD_KEY"