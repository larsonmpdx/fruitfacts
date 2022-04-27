#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# this command has certbot go through our file in /etc/nginx/sites-enabled/ and add managed ssl bits to it
sudo certbot --nginx -d fruitfacts.xyz -d www.fruitfacts.xyz -d api.fruitfacts.xyz

# ... there will be interactive stuff to do here

sudo service nginx reload
