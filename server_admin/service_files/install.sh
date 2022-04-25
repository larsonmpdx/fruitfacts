#!/usr/bin/env bash
set -e

cp ./*.service /etc/systemd/system/

# run like:
# sudo systemctl start fruitfacts_frontend.service
#  - start/stop/status/ etc.