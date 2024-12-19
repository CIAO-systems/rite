#!/bin/env bash
# For this script you need to install tomlq (https://github.com/cryptaliagy/tomlq):
# cargo install tomlq

# Use tq to extract the workspace members array
projects=$(tq -f Cargo.toml '.workspace.members')

# Remove the brackets and quotes from the array
projects=$(echo "$projects" | sed 's/[][]//g' | tr -d '"' | tr ',' ' ')

for project in $projects; do
    echo Publishing $project...
    pushd $project
    cargo publish
    popd
done

