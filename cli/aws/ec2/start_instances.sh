#!/usr/bin/env bash

INSTANCE_IDS=$1

aws ec2 start-instances \
  --instance-ids $INSTANCE_IDS && \
aws ec2 wait instance-running \
  --instance-ids $INSTANCE_IDS