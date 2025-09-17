#!/bin/env bash
cargo tarpaulin \
    --skip-clean --locked \
    --all-features \
    --workspace \
    --timeout 300 \
    --exclude-files "base/rite/src/main.rs" \
    --exclude-files "base/rite/src/processor/process.rs" \
    --exclude-files "base/plugins/llm/src/importers/**" \
    --exclude-files "custom/**" \
    --exclude-files "extended/**" \
    --out Html \
    --output-dir target/coverage
