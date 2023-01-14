#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

echo "git update"
git checkout .
./server_admin/set_folder_permissions_for_www-data.sh
sudo -u www-data git pull

echo "dvc pull"
dvc pull

echo "setting maintenance html"
maintenance_page=/usr/share/nginx/html/maintenance.html
rm -f $maintenance_page
cp ./server_admin/letsencrypt/nginx_base_files/maintenance.html $maintenance_page

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

cd ./frontend/
sudo -u www-data npm install --force # "--force" is only for one broken package, see frontend README, remove it asap
sudo -u www-data npm run build

cd ../backend/
sudo -u www-data rm -f ./Cargo.lock
sudo -u www-data touch build.rs                              # make sure this runs each time so our env vars are updated

sudo -u www-data cargo run --release --no-default-features -- --reload_db

# view live tailed logs:
# journalctl -xefu backend_fruitfacts

echo "starting backend+frontend"
service backend_fruitfacts start
service frontend_fruitfacts start

echo "unsetting maintenance html"
rm -f $maintenance_page
