#!/bin/bash

# Check if the server address is provided as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <server_address>"
    exit 1
fi

# Set the server address from the argument
SERVER_ADDRESS=$1

# Optional: Uncomment and set the path to your CA certificate if using TLS
# CA_CERT_PATH="path/to/ca.crt"

# Optional: Uncomment and set the import path for proto files if needed
# IMPORT_PATH="path/to/proto"

# Function to describe a method
describe_method() {
    local service=$1
    local method=$2
    echo "Describing $service.$method..."
    if [ -n "$CA_CERT_PATH" ]; then
        grpcurl --cacert "$CA_CERT_PATH" "$SERVER_ADDRESS" describe "$service.$method"
    elif [ -n "$IMPORT_PATH" ]; then
        grpcurl --import-path "$IMPORT_PATH" "$SERVER_ADDRESS" describe "$service.$method"
    else
        grpcurl "$SERVER_ADDRESS" describe "$service.$method"
    fi
    echo ""
}

# Function to describe a service
describe_service() {
    local service=$1
    echo "Describing $service..."
    if [ -n "$CA_CERT_PATH" ]; then
        grpcurl --cacert "$CA_CERT_PATH" "$SERVER_ADDRESS" describe "$service"
    elif [ -n "$IMPORT_PATH" ]; then
        grpcurl --import-path "$IMPORT_PATH" "$SERVER_ADDRESS" describe "$service"
    else
        grpcurl "$SERVER_ADDRESS" describe "$service"
    fi
    echo ""
}

# List all services and methods
echo "Listing all services and methods..."
if [ -n "$CA_CERT_PATH" ]; then
    services=$(grpcurl --cacert "$CA_CERT_PATH" "$SERVER_ADDRESS" list)
elif [ -n "$IMPORT_PATH" ]; then
    services=$(grpcurl --import-path "$IMPORT_PATH" "$SERVER_ADDRESS" list)
else
    services=$(grpcurl "$SERVER_ADDRESS" list)
fi

# Loop through each service and method
echo "$services" | while IFS= read -r line; do
    describe_service $line
    if [[ "$line" == *"is a service"* ]]; then
        service=$(echo "$line" | awk '{print $1}')
        echo "Service: $service"
    elif [[ "$line" == *"is a method"* ]]; then
        method=$(echo "$line" | awk '{print $1}' | sed "s/$service.//")
        describe_method "$service" "$method"
    fi
done
