#!/bin/env bash
export DOCKER_BUILDKIT=1

# Build base image
docker buildx build -t rite-base:latest -f Dockerfile.base .

# Build extended image
docker buildx build -t rite-extended:latest -f Dockerfile.extended .

# Build custom image (for now, this is the main image)
docker buildx build -t rite:latest -f Dockerfile.custom .
