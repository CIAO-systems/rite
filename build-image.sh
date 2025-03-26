#!/bin/env bash
eval "$(ssh-agent -s)"
ssh-add $1

export DOCKER_BUILDKIT=1
docker buildx \
    build \
    --cache-from rite:builder \
    --ssh default=$SSH_AUTH_SOCK \
    -t rite:latest .

#    --platform linux/arm64/v8,linux/amd64 \ # build for both ARM and x86 https://docs.docker.com/build/building/multi-platform/
