#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# rebuild and restart the frontend and backend after a code update

echo "dvc pull"
cd ..
dvc pull
cd -

echo "setting maintenance html"
maintenance_page=/usr/share/nginx/html/maintenance.html
rm -f $maintenance_page
cp ./letsencrypt/nginx_base_files/maintenance.html $maintenance_page

# replace templated elements that look like %NAME% in this file
# initially done so we can see elapsed time from when maintenance started
echo "editing $maintenance_page"
d=$(date +%s)
replacers=("UNIX_TIME_S=$d") # can add more elements to this
for i in "${replacers[@]}"; do
    echo "replacing $i"
    setting="$(echo "$i" | cut -d '=' -f 1)"
    value="$(echo "$i" | cut -d '=' -f 2-)"

    sed -i -e "s;%${setting}%;${value};g" "$maintenance_page"
done

echo "stopping backend+frontend"
service backend_fruitfacts stop
service frontend_fruitfacts stop

cd ../frontend/
sudo -u www-data npm install --force # "--force" is only for one broken package, see frontend README, remove it asap
sudo -u www-data npm run build

cd ../backend/
sudo -u www-data rm -f ./Cargo.lock
touch build.rs                                               # make sure this runs each time so our env vars are updated
sudo -u www-data cargo build --release --no-default-features # --no-default-features: skip our support binaries

echo "starting backend+frontend"
service backend_fruitfacts start
service frontend_fruitfacts start

echo "unsetting maintenance html"
rm -f $maintenance_page
