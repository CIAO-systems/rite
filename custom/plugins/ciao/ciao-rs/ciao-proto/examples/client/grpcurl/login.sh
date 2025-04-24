#/bin/env bash

# To run this, grpcurl has to be installed. See https://github.com/fullstorydev/grpcurl

# Parse commandline parameters 
source ./parameters.sh
source ./functions.sh

result=$(login $USERNAME $PASSWORD)
echo $SERVER returnd: $result

