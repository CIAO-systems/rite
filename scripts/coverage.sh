#!/bin/env bash
cargo tarpaulin \
    --skip-clean --locked \
    --all-features \
    --workspace \
    --timeout 300 \
    --exclude-files "base/rite/src/main.rs" \
    --out Html \
    --output-dir target/coverage
