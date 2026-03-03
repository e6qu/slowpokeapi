#!/bin/sh
set -e

DATA_DIR="${SLOWPOKEAPI_DATA_DIR:-/data}"

if [ ! -d "$DATA_DIR" ]; then
    mkdir -p "$DATA_DIR"
fi

exec /slowpokeapi "$@"
