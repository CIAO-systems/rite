#!/bin/env bash
docker run --network=host --rm -v ./data:/data -v ./tmp:/logs ciao-rite $1