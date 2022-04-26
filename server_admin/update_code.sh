#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../frontend/
sudo -u www-data npm install
sudo -u www-data npm run build

systemctl restart frontend_fruitfacts.service

cd ../backend/
sudo -u www-data cargo build --release

systemctl restart backend_fruitfacts.service
