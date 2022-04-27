#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

sudo certbot --nginx -d fruitfacts.xyz -d www.fruitfacts.xyz -d api.fruitfacts.xyz

# ... there will be interactive stuff to do here

sudo service nginx reload
