#!/usr/bin/env bash
set -e

git config --global --add safe.directory /var/www/fruitfacts

cd ..
sudo chown -R www-data:www-data .
cd -

