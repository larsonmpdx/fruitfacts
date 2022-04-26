#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# these probably already exist
groupadd -g 1000 www-data
useradd -g www-data www-data
