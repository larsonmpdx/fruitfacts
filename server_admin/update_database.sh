#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../backend/
systemctl stop backend_fruitfacts.service || true
sudo -u www-data cargo run --release -- --reload_db
systemctl start backend_fruitfacts.service
