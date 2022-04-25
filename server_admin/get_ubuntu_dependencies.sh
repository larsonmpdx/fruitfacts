#!/usr/bin/env bash
set -e

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

# https://forge.rust-lang.org/infra/other-installation-methods.html#rustup
curl https://sh.rustup.rs -sSf | sh

# https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04
# https://github.com/nvm-sh/nvm#install--update-script
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash

