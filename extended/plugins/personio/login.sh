#!/bin/bash
source ../../.env
curl --request POST \
     --url https://api.personio.de/v2/auth/token \
     --header 'accept: application/json' \
     --header 'content-type: application/x-www-form-urlencoded' \
     --data grant_type=client_credentials \
     --data client_id=$PERSONIO_CLIENT_ID \
     --data client_secret=$PERSONIO_CLIENT_SECRET | jq