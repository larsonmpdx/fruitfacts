#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

sudo -u www-data git pull
./server_admin/update_code.sh
