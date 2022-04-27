#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"


# these are base files that will have ssl-stuff added to them by certbot
cp ./nginx_base_files/fruitfacts_frontend.conf /etc/nginx/sites-available/
ln -s /etc/nginx/sites-available/fruitfacts_frontend.conf /etc/nginx/sites-enabled/fruitfacts_frontend.conf

cp ./nginx_base_files/fruitfacts_backend.conf /etc/nginx/sites-available/
ln -s /etc/nginx/sites-available/fruitfacts_backend.conf /etc/nginx/sites-enabled/fruitfacts_backend.conf

sudo service nginx reload
