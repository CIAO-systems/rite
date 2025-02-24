#!/bin/env bash

DOCKER_BUILDKIT=1 docker buildx build --ssh default=$SSH_AUTH_SOCK -t rite .