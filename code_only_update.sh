#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

./server_admin/set_folder_permissions_for_www-data.sh
sudo -u www-data git pull

service backend_fruitfacts stop
service frontend_fruitfacts stop

./server_admin/code_update.sh

service backend_fruitfacts start
service frontend_fruitfacts start
