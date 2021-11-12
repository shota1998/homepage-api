#!/usr/bin/env bash

INSTANCE_IDS=$1

aws ec2 stop-instances \
  --instance-ids $INSTANCE_IDS