#!/bin/env bash
docker run --network=host --rm -v ./data:/data -v ./log:/logs ciao-rite $1