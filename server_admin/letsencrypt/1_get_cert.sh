#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# digitalocean credentials file notes (needs to be manually created)
# https://certbot-dns-digitalocean.readthedocs.io/en/stable/

# first create an ini file (see page)

# then run this
certbot certonly \
  --dns-digitalocean \
  --dns-digitalocean-credentials ~/digitalocean.ini \
  -d fruitfacts.xyz \
  -d www.fruitfacts.xyz
