#!/bin/bash

set -e

if [ $# -ne 1 ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

DIR="$1"
ZIPFILE="/tmp/rite-server.$RANDOM.zip"
B64FILE="/tmp/rite-server.$RANDOM.b64"
GRPC_SERVER="localhost:50051"
GRPC_METHOD="rite.v1.RiteService/Process"

source .env

# Zip the directory
zip -j -r "$ZIPFILE" "$DIR" > /dev/null

# Base64 encode the zip file (no line wrapping)
if base64 --help 2>&1 | grep -q -- '-w '; then
  base64 -w0 "$ZIPFILE" > "$B64FILE"
else
  base64 "$ZIPFILE" | tr -d '\n' > "$B64FILE"
fi

B64_CONTENT=$(cat "$B64FILE")

# Prepare JSON payload
JSON_PAYLOAD="{\"zipped_configuration\": \"$B64_CONTENT\"}"

# Send with grpcurl
grpcurl -plaintext \
    -H "x-api-key: $GRPC_API_KEY" \
    -d "$JSON_PAYLOAD" "$GRPC_SERVER" "$GRPC_METHOD"

# Clean up
rm -f "$ZIPFILE" "$B64FILE"
