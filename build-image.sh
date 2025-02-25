#!/bin/env bash
eval "$(ssh-agent -s)"
ssh-add $1

DOCKER_BUILDKIT=1 docker buildx build --ssh default=$SSH_AUTH_SOCK -t rite .