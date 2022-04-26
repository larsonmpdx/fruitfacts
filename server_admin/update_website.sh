#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../frontend/
npm install
npm run build

cd ../backend/
cargo build --release
systemctl stop backend_fruitfacts.service || true
cargo run -- --reload_db
systemctl start backend_fruitfacts.service

./set_folder_permissions_for_www-data.sh
