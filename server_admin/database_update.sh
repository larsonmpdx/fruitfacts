#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# reload the database from the .json5 file set

cd ../backend/
echo "stopping backend"
systemctl stop backend_fruitfacts.service || true
sudo -u www-data cargo run --release --no-default-features -- --reload_db
echo "starting backend"
systemctl start backend_fruitfacts.service
