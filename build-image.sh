#!/bin/env bash
eval "$(ssh-agent -s)"
ssh-add $1

export DOCKER_BUILDKIT=1
docker buildx \
    build \
    --cache-from rite:builder \
    --ssh default=$SSH_AUTH_SOCK \
    --platform linux/arm64/v8,linux/amd64 \
    -t rite:latest .