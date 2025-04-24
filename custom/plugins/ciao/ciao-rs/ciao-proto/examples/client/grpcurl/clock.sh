#!/bin/bash

#
#   Prerequisites: 
#     - grpcurl is installed (https://github.com/fullstorydev/grpcurl)
#     - jq is installed (https://jqlang.github.io/jq/), usually available in the package manager
#       of your Linux distribution
#

# Parse commandline parameters 
source ./parameters.sh
source ./functions.sh

function clock() {
    local TOKEN=$1
    local USER_ID=$2
    local TIMESTAMP_UTC=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local TIMESTAMP_OFFSET=$(convert_offset_to_seconds $(date +%z))s

    local parameters=$(jq -n \
        --arg user "$USER_ID" \
        --arg timetypeid "$TIMETYPE" \
        --arg deviceid "$DEVICE" \
        --arg projectid "$PROJECT" \
        --arg costcenterid "$COST_CENTER" \
        --arg timestamp "$TIMESTAMP_UTC" \
        --arg offset "$TIMESTAMP_OFFSET" \
        '{timestamp:{time_utc:$timestamp,offset:$offset},user_id:$user,time_type_id:$timetypeid,device_id:$deviceid,project_id:$projectid,cost_center_id:$costcenterid}')

    call_auth \
        ciao/time_tracking/service.proto \
        ciao.time_tracking.TimeTrackingService/Clock \
        "$parameters" \
        $TOKEN
}

function main() {
    echo 
    echo Welcome to the CIAO cli clock service
    echo  - Clock on server: "$SERVER"
    echo  - For user "$USERNAME"
    echo  - With time-type "$TIMETYPE"
    echo  - At $(date -u +"%Y-%m-%dT%H:%M:%SZ")
    echo    - with offset "$(date +%z)" = "$(convert_offset_to_seconds $(date +%z))s"
    echo

    local result=$(login $USERNAME $PASSWORD)

    local token=$(echo $result | jq -r '.token')
    local userId=$(echo $result | jq -r '.account.id')

    local message=$(clock $token $userId | jq -r '.message') 
    echo -e "$message"
}


main