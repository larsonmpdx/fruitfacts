#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

git pull
./server_admin/set_folder_permissions_for_www-data.sh
./server_admin/update_website.sh
