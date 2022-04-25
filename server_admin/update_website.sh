#!/usr/bin/env bash
set -e

git pull

cd ./frontend/
nvm install --lts
node --version

npm install
npm run build

cd ../backend/
cargo build
