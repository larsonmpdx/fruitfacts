#!/usr/bin/env bash
set -e

cp ./*.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable frontend_fruitfacts.service
systemctl enable backend_fruitfacts.service

# run like:
# sudo systemctl start fruitfacts_frontend.service
#  - start/stop/status/ etc.