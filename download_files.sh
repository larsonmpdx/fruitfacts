#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# this is a one-time download. could go into a fancy build.rs script I guess
./backend/src/gazetteer_load/download.sh
