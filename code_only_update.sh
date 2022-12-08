#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

./server_admin/set_folder_permissions_for_www-data.sh
sudo -u www-data git pull

./server_admin/code_update.sh
