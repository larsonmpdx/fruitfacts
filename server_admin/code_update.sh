#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# rebuild and restart the frontend and backend after a code update

cd ../frontend/
sudo -u www-data npm install --force # "--force" is only for one broken package, see frontend README, remove it asap
sudo -u www-data npm run build

echo "restarting frontend"
systemctl restart frontend_fruitfacts.service

cd ../backend/
sudo -u www-data rm -f ./Cargo.lock
touch build.rs # make sure this runs each time so our env vars are updated
sudo -u www-data cargo build --release

echo "restarting backend"
systemctl restart backend_fruitfacts.service
