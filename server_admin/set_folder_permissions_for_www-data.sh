#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

git config --global --add safe.directory /var/www/fruitfacts

cd ..
sudo chown -R www-data:www-data .
cd -

