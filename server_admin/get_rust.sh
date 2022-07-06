#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

# use this method so it's installed for all users (rustup is only for one user)
# https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers
rm -rf ./rust*/
rm ./*.tar.gz
wget https://static.rust-lang.org/dist/rust-1.62.0-x86_64-unknown-linux-gnu.tar.gz
tar -xf ./rust*.tar.gz
cd ./rust*/
sudo ./install.sh
cd ..
