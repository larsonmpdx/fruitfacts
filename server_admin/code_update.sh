#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# rebuild and restart the frontend and backend after a code update

echo "dvc pull"
cd ..
dvc pull
cd -

echo "stopping backend+frontend"
service backend_fruitfacts stop
service frontend_fruitfacts stop

cd ../frontend/
sudo -u www-data npm install --force # "--force" is only for one broken package, see frontend README, remove it asap
sudo -u www-data npm run build

cd ../backend/
sudo -u www-data rm -f ./Cargo.lock
touch build.rs # make sure this runs each time so our env vars are updated
sudo -u www-data cargo build --release --no-default-features # --no-default-features: skip our support binaries

echo "starting backend+frontend"
service backend_fruitfacts start
service frontend_fruitfacts start
