#!/bin/env bash

# Make sure, you are logged in the GHCR
CONTAINER_IMAGE=ghcr.io/ciao-systems/rite:main
DEBUG=false
SILENT=false
PULL=false
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
        --debug)
          DEBUG=true
          shift 1
          ;;
        --silent)
          SILENT=true
          shift 1
          ;;
        --pull)
          PULL=true
          shift 1
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
  echo "  --debug "
  echo "     Start a bash shell instead of the rite application"
  echo "  --silent "
  echo "     Do not print anything from this script"
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
local_logs_directory="$local_directory/logs"

# Create logs directory, so the container does not create it with root
mkdir -p "$local_logs_directory"

if [[ "$DEBUG" == "true" ]]; then
  args=(
      run
      -u $(id -u):$(id -g)
      --network=host
      --rm
      --platform linux/amd64
      -it 
      --entrypoint /bin/bash
      -v "$local_directory:/data"
      -v "$local_logs_directory:/app/logs:rw"
      $CONTAINER_IMAGE 
  )
else
  args=(
      run
      -u $(id -u):$(id -g)
      --network=host
      --rm
      --platform linux/amd64
      -v "$local_directory:/data"
      -v "$local_logs_directory:/app/logs:rw"
      $CONTAINER_IMAGE "/data/$container_filename"
  )
fi

if [[ "$SILENT" == "false" ]]; then
  echo "Image               : $CONTAINER_IMAGE"
  echo "Local directory     : $local_directory"
  echo "Local filename      : $local_filename"
  echo "Local log directory : $local_logs_directory"
  echo
fi

if [[ "$PULL" == "true" ]]; then
  docker pull $CONTAINER_IMAGE
fi
# Execute the container image
docker "${args[@]}"
