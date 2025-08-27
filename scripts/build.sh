#!/bin/bash

# Export ESP-IDF environment variables
export IDF_PATH=/home/esp/esp-idf

# Source ESP environment if not already done
which idf.py >/dev/null || {
    source ~/export-esp.sh >/dev/null 2>&1
}

# Clean any previous build artifacts
rm -rf target .embuild

case "$1" in
"" | "release")
    cargo build --release
    ;;
"debug")
    cargo build
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac
