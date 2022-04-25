#!/usr/bin/env bash
set -e

cp ./*.service /etc/systemd/system/
systemctl daemon-reload

# run like:
# sudo systemctl start fruitfacts_frontend.service
#  - start/stop/status/ etc.