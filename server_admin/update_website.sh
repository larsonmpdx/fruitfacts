#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../frontend/
npm install
npm run build

cd ../backend/
cargo build --release
