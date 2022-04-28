#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# first create an ini file:
# https://www.digitalocean.com/community/tutorials/how-to-acquire-a-let-s-encrypt-certificate-using-dns-validation-with-certbot-dns-digitalocean-on-ubuntu-20-04
# -> save to ~/digitalocean.ini
chmod go-rwx ~/digitalocean.ini # certbot complains if access isn't limited

# then run this
certbot certonly \
  --dns-digitalocean \
  --dns-digitalocean-credentials ~/digitalocean.ini \
  -d fruitfacts.xyz \
  -d www.fruitfacts.xyz \
  -d api.fruitfacts.xyz

# ... there will be interactive stuff to do here
