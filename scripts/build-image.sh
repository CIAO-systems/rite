#!/bin/env bash
export DOCKER_BUILDKIT=1
docker buildx \
    build \
    --cache-from rite:builder \
    -t rite:latest .
