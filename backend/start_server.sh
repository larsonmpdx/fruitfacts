# I can't figure out why, but a "cargo build" run as the www-user doesn't save the built files
# in a way that the service (running as www-user) can use them, and the service always rebuilds
# so instead, we ask the service to check for a db reload and rebuild when it starts

set -e
cd "$(dirname "${BASH_SOURCE[0]}")"

maintenance_page=/usr/share/nginx/html/maintenance.html # also in data_and_code_update.sh
RELOAD_TRIGGER_FILE=./RELOAD_DB
if [[ -f "$RELOAD_TRIGGER_FILE" ]]; then
    cargo run --release --no-default-features -- --reload_db
    rm -f $RELOAD_TRIGGER_FILE
else
    # try a build first so we don't unset maintenance html unless it's a clean build
    cargo build --release --no-default-features
fi

echo "unsetting maintenance html"
rm -f $maintenance_page

cargo run --release
