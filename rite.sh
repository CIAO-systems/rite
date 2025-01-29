#!/bin/env bash

# Make sure, you are logged in the GHCR
CONTAINER_IMAGE=ghcr.io/ciao-systems/rite:main
local_filename=""

# Function to parse command-line arguments
parse_args() {
  local file_arg
  local image_arg=$CONTAINER_IMAGE  # Default image

  while [[ $# -gt 0 ]]; do
    case "$1" in
        -f|--file)
            file_arg="$2"
            shift 2
            ;;
        -ci|--container-image)
            image_arg="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unkown option: $1"
            usage
            exit 1
            ;;
    esac
  done

  local_filename="$file_arg"
  CONTAINER_IMAGE="$image_arg"
}

usage() {
  echo "Usage: $(basename "$0") [OPTIONS]"
  echo ""
  echo "OPTIONS:"
  echo "  -f, --file <filename> "
  echo "     Path to the local configuration file. (Required)"
  echo "  -ci, --container-image <image_name> "
  echo "     Name of the Docker image to use. (Optional, default: \"$CONTAINER_IMAGE\")"
  echo "  -h, --help "
  echo "     Display this help message and exit"
  echo ""
  echo "Examples:"
  echo "  $(basename "$0") -f /path/to/config.xml"
  echo "  $(basename "$0") -f /path/to/config.xml -ci MyCustomImage"
  echo "  $(basename "$0") --file=/path/to/config.xml --container-image MyCustomImage"
  echo ""
  echo "Notes:"
  echo "  The script mounts the directory containing the specified configuration file to \"/data\" within the container."
  echo "  When the directory contains a log4rs.yaml file, it will be used for logging configuration."
  echo ""
  echo "  Log files, if configured, will be written in the sub directory 'logs'"
  echo ""
  exit 1
}

# Parse the arguments
parse_args "$@"

# Check if a filename is provided
if [ -z "$local_filename" ]; then
    usage
fi

local_directory="$(dirname "$local_filename")"
container_filename="$(basename "$local_filename")"

args=(
    run
    --network=host
    --rm
    -v "$local_directory:/data"
    -v "$local_directory/logs:/logs"
    $CONTAINER_IMAGE "/data/$container_filename"
)

echo "Running with image $CONTAINER_IMAGE"
# Execute the container image
docker "${args[@]}"
