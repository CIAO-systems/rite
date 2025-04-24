#!/bin/bash

convert_offset_to_seconds() {
    local offset="$1"
    
    # Extract sign, hours, and minutes
    local sign="${offset:0:1}"
    local hours="${offset:1:2}"
    local minutes="${offset:3:2}"
    
    # Remove leading zeros
    hours="${hours#0}"
    minutes="${minutes#0}"
    
    # Calculate total seconds
    local total_seconds=$((hours * 3600 + minutes * 60))
    
    # Apply sign
    if [ "$sign" = "-" ]; then
        total_seconds=$((-total_seconds))
    fi
    
    echo "$total_seconds"
}

function login() {
    local USER="$1"
    local PASSWORD="$2"

    local parameters=$(jq -n --arg email "$USER" --arg secret "$PASSWORD" \
         '{secret: $secret, email: $email}')

    GRPCCALL='grpcurl '$PLAINTEXT
    GRPCCALL+=' -proto ciao/core/auth/service.proto'
    GRPCCALL+=' -import-path ../../../proto'
    if [[ -n "$API_KEY" ]]; then
        GRPCCALL+=' -H "X-API-Key: '$API_KEY'"'
    fi 
    GRPCCALL+=' -d "${parameters}" '$SERVER' ciao.core.auth.AuthenticationService/Login'
    eval "$GRPCCALL"

    echo $result
}

function call_auth() {
    local PROTO=$1
    local SERVICE=$2
    local REQUEST=$3
    local TOKEN=$4

    GRPCCALL='grpcurl '$PLAINTEXT
    GRPCCALL+=' -proto '$PROTO
    GRPCCALL+=' -import-path ../../../proto'
    if [[ -n "$API_KEY" ]]; then
        GRPCCALL+=' -H "X-API-Key: '$API_KEY'"'
    fi 
    GRPCCALL+=' -H "Authorization: Bearer '$TOKEN'"'
    GRPCCALL+=' -d "${REQUEST}" '$SERVER' $SERVICE'
    eval "$GRPCCALL"

    # grpcurl $PLAINTEXT \
    #     -proto ../../../proto/$PROTO \
    #     -import-path ../../../proto \
    #     $([ -n "$API_KEY"] && echo "-H \"X-API-Key: $API_KEY\"") \
    #     -H "Authorization: Bearer $TOKEN" \
    #     -d "${REQUEST}" $SERVER $SERVICE
}
