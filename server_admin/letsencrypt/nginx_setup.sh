#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cp ./fruitfacts_frontend /etc/nginx/sites-available/
ln -s /etc/nginx/sites-available/fruitfacts_frontend /etc/nginx/sites-enabled/fruitfacts_frontend

cp ./fruitfacts_backend /etc/nginx/sites-available/
ln -s /etc/nginx/sites-available/fruitfacts_backend /etc/nginx/sites-enabled/fruitfacts_backend

sudo service nginx reload
