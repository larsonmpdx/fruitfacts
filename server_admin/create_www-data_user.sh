#!/usr/bin/env bash
set -e

# these probably already exist
groupadd -g 1000 www-data
useradd -g www-data www-data
