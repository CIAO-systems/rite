#!/bin/env bash

# Print environment information
echo "==== Environment Info ===="
rustc --version
cargo --version
cargo tarpaulin --version
echo "==== End Environment Info ===="

# Show all targets that will be tested
echo "==== Test Targets ===="
cargo test --no-run --message-format=json | jq -r "select(.reason == \"compiler-artifact\") | select(.target.kind[] | contains(\"test\")) | .target.name"
echo "==== End Test Targets ===="

# First run tests to ensure they pass
cargo test --verbose --all-features --workspace

cargo tarpaulin \
    --skip-clean --locked \
    --all-features \
    --workspace \
    --timeout 300 \
    --out Html \
    --output-dir target/coverage
