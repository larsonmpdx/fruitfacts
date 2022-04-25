#!/usr/bin/env bash
set -e

sudo apt update
sudo apt upgrade -y
sudo apt install -y build-essential pkg-config libssl-dev

# https://forge.rust-lang.org/infra/other-installation-methods.html#rustup
curl https://sh.rustup.rs -sSf | sh

# https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04
# https://github.com/nvm-sh/nvm#install--update-script
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash

