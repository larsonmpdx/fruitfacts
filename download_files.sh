#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

./backend/src/gazetteer_load/download.sh
