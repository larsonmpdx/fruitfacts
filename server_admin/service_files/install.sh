#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cp ./*.service /etc/systemd/system/

echo "enabling systemd services"
systemctl daemon-reload
systemctl enable frontend_fruitfacts.service
systemctl enable backend_fruitfacts.service

# run like:
# sudo systemctl start fruitfacts_frontend.service
#  - start/stop/status/ etc.

# live logs:
# journalctl -u backend_fruitfacts.service -f