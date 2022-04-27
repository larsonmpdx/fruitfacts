#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

sudo apt update
sudo apt upgrade -y

# some of these might be able to be trimmed now that we're using llvm
sudo apt install -y build-essential pkg-config libssl-dev software-properties-common

# install llvm so we can link with lld
# it takes less memory so it can be done right on the server
# https://apt.llvm.org/
wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key|sudo apt-key add
sudo add-apt-repository 'deb http://apt.llvm.org/focal/ llvm-toolchain-focal-14 main' # focal is ubuntu 20.04 (lts)
sudo apt-get -y install clang-14 lldb-14 lld-14 # llvm 14 released march 2022
ln -s /usr/bin/clang-14 /usr/bin/clang
ln -s /usr/bin/lld-14 /usr/bin/lld

# use this method so it's installed for all users (rustup is only for one user)
# https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers
wget https://static.rust-lang.org/dist/rust-1.60.0-x86_64-unknown-linux-gnu.tar.gz
tar -xf ./rust*.tar.gz
cd ./rust*/
sudo ./install.sh
cd ..

# use this method so it's installed for all users (nvm is only for one user)
# https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04
curl -o- https://deb.nodesource.com/setup_16.x | bash
sudo apt install -y nodejs
node -v

# certbot / let's encrypt stuff
sudo snap install core; sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot

sudo apt install -y nginx
