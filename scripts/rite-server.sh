#!/bin/bash
set -e

# Function to display usage
usage() {
    echo "Usage: $0 -d|--directory <directory> -f|--filename <config-filename>"
    echo "       $0 --directory <directory> --filename <config-filename>"
    echo ""
    echo "Options:"
    echo "  -d, --directory   Directory to zip and send"
    echo "  -f, --filename    Main config filename"
    echo "  -h, --help        Show this help message"
    exit 1
}

# Initialize variables
DIR=""
CONFIG_FILENAME=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--directory)
            DIR="$2"
            shift 2
            ;;
        -f|--filename)
            CONFIG_FILENAME="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unknown option: $1"
            usage
            ;;
    esac
done

# Check if both required parameters are provided
if [ -z "$DIR" ] || [ -z "$CONFIG_FILENAME" ]; then
    echo "Error: Both directory and config filename are required."
    usage
fi

# Check if directory exists
if [ ! -d "$DIR" ]; then
    echo "Error: Directory '$DIR' does not exist."
    exit 1
fi

# Check if config file exists in the directory
if [ ! -f "$DIR/$CONFIG_FILENAME" ]; then
    echo "Error: Config file '$CONFIG_FILENAME' does not exist in directory '$DIR'."
    echo "Available files in directory:"
    ls -la "$DIR"
    exit 1
fi

ZIPFILE="/tmp/rite-server.$RANDOM.zip"
B64FILE="/tmp/rite-server.$RANDOM.b64"
GRPC_SERVER="localhost:50051"
GRPC_METHOD="rite.v1.RiteService/Process"

source .env

# Zip the directory (preserving directory structure, with given directory as root)
cd "$DIR"
zip -r "$ZIPFILE" . > /dev/null
cd - > /dev/null

# Base64 encode the zip file (no line wrapping)
if base64 --help 2>&1 | grep -q -- '-w '; then
    base64 -w0 "$ZIPFILE" > "$B64FILE"
else
    base64 "$ZIPFILE" | tr -d '\n' > "$B64FILE"
fi

B64_CONTENT=$(cat "$B64FILE")

# Prepare JSON payload with both zipped_configuration and main_config
JSON_PAYLOAD="{\"zipped_configuration\": \"$B64_CONTENT\", \"main_config\": \"$CONFIG_FILENAME\"}"

# Send with grpcurl
grpcurl -plaintext \
    -H "x-api-key: $GRPC_API_KEY" \
    -d "$JSON_PAYLOAD" "$GRPC_SERVER" "$GRPC_METHOD"

# Clean up
rm -f "$ZIPFILE" "$B64FILE"