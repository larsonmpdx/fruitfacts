#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../frontend/
sudo -u www-data npm install
sudo -u www-data npm run build

cd ../backend/
sudo -u www-data cargo build --release
systemctl stop backend_fruitfacts.service || true
sudo -u www-data cargo run -- --reload_db
systemctl start backend_fruitfacts.service

# ./set_folder_permissions_for_www-data.sh
