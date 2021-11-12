#!/usr/bin/env bash

# ***************************
# Initial setting.
# ***************************
# (project_root)/dev
cd ../dev

# ***************************
# Check prerequisite.
# ***************************
if [ ! -e variables.sample.json ]; then
  echo "You have to create (project_root)/dev/variables.sample.json"
  exit 1
fi

# ***************************
# Create working enviroment.
# ***************************
FILE="variables.json"

if [ ! -e $FILE ]; then
  touch $FILE
  chmod 755 $FILE
  cp variables.sample.json $FILE
fi

# ***************************
# Read variables.json.
# ***************************
FILE="$(pwd)/variables.json"

# n=1
# while read line; do
# # echo "Line No. $n : $line"
# # n=$((n+1))
# echo $line
# done < $FILE

JSON_PARENT_KEY='.aws[].ec2'
JSON_CHILD_KEY='{
                  address,
                  cli_user
                }'

# jq "$JSON_PARENT_KEY | $JSON_CHILD_KEY"
# X=$(cat $FILE | jq ".aws")
X=$(cat $FILE | jq ".aws.ec2.address")
# echo ${X[address]}
echo $X
# echo $FILE | jq ".aws[] | {ec2}"

# declare -A X
# X["a"]=aaa
# X["b"]=bbb
# echo ${X[@]}
echo "----- End. -----"