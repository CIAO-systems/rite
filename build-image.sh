#!/bin/env bash
#
#   Script to build the container image using the Open SSH private key for GitHub access
#   ./build-image.sh ~/.ssh/<your-github-private-key-for-ciao>
#
eval "$(ssh-agent -s)"
ssh-add $1

export DOCKER_BUILDKIT=1
docker buildx \
    build \
    --cache-from rite:builder \
    --ssh default=$SSH_AUTH_SOCK \
    -t rite:latest .

