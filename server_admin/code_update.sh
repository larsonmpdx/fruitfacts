#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# rebuild and restart the frontend and backend after a code update

cd ../frontend/
sudo -u www-data npm install
sudo -u www-data npm run build

echo "restarting frontend"
systemctl restart frontend_fruitfacts.service

cd ../backend/
sudo -u www-data cargo build --release

echo "restarting backend"
systemctl restart backend_fruitfacts.service
