#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# these probably already exist (I think they're part of the base ubuntu distro)
groupadd -g 1000 www-data
useradd -g www-data www-data
