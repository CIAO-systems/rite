#!/bin/bash

# Default values
export SERVER="localhost:50051"
export USERNAME="chuck.norris@example.us"
export PASSWORD="Chuck Norris.does.not.need.a.password"
export TIMETYPE="1"
export DEVICE=""
export PROJECT=""
export COST_CENTER=""
export PLAINTEXT=
export API_KEY=


# Function to display usage
usage() {
    echo "Usage: $0 [-s SERVER] [-u USERNAME] [-p PASSWORD] [--time-type TIMETYPE]"
    echo "  -s SERVER               Server address (default: $SERVER)"
    echo "  -u USERNAME             Username (default: $USERNAME)"
    echo "  -p PASSWORD             Password (default: $PASSWORD)"
    echo "  --plaintext             Flag. If the server does not use TLS, default is TLS"  
    echo "  --api-key=<value>       API-Key (default: $API_KEY)"
    echo "  --time-type=<id>        time-type number (default: $TIMETYPE)"
    echo "  --device-id=<id>        Device Id (default: $DEVICE)"
    echo "  --project-id=<id>       Project Id (default: $PROJECT)"
    echo "  --cost-center-id=<id>   Cost-Center Id (default: $COST_CENTER)"
    exit 1
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -s)
            SERVER="$2"
            shift 2
            ;;
        -u)
            USERNAME="$2"
            shift 2
            ;;
        -p)
            PASSWORD="$2"
            shift 2
            ;;
        --api-key=*)
            API_KEY="${1#*=}"
            shift
            ;;
        --plaintext)
            PLAINTEXT="-plaintext"
            shift
            ;;
        --time-type=*)
            TIMETYPE="${1#*=}"
            shift
            ;;
        --device-id=*)
            DEVICE="${1#*=}"
            shift
            ;;
        --project-id=*)
            PROJECT="${1#*=}"
            shift
            ;;
        --cost-center-id=*)
            COST_CENTER="${1#*=}"
            shift
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
