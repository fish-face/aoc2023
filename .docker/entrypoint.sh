#!/bin/bash
set -euo pipefail

if [[ -f "src/bin/day-${1}.rs" ]]; then
    cargo build --release --bin "day-${1}"
    time "target/release/day-${1}" "inputs/day-${1}"
fi
