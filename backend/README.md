# rust hints
* copy the .env.example file and fill in secrets as needed
* `cargo build`
* `cargo run`
* `cargo test` and `cargo test -- --include-ignored` (include long tests: the json database loading one)
* `cargo fetch` install packages
* `cargo run --bin pdf_to_thumbnail` create thumbnails for each reference pdf

# rust linting
* `cargo fmt` after installing `rustup component add rustfmt`
* `cargo fix`
* `cargo clippy` after installing `rustup component add clippy`
  * `cargo clippy --fix`
* `cargo outdated -d1` find outdated packages (-d1: direct only) or `cargo outdated` (all) after installing `cargo install --locked cargo-outdated` (same command to update)
* `cargo +nightly udeps` to find unused dependencies, after installing (`rustup toolchain install nightly` then `cargo install cargo-udeps --locked`) see https://crates.io/crates/cargo-udeps
* `cargo tree --duplicates` find dependencies with multiple required versions

# after a rust version release
* see `rust_update.bat`:
  * `rustup update stable`
  * `rustup toolchain install nightly`
  * `cargo install --locked cargo-outdated`
  * `cargo install cargo-udeps --locked`
  * `cargo test -- --include-ignored`

# oauth account stuff
* google https://console.cloud.google.com/apis/dashboard?pli=1 click credentials, click the edit pen, add an "authorized redirect URI" like http://domain.com/api/authRedirect

# debugging in vs code
* see extensions.json for recommended extensions
  * 2021: on windows vs code, `codelldb+rust-analyzer` debugger works slightly better than cppvsdbg or the official "rust" extension. see launch.json

# diesel (rust ORM) things
* see https://diesel.rs/guides/getting-started
* there's no way to specify a dependency version with a regular `cargo install` command, and diesel seems to randomly pick an old bundled sqlite version, so to get diesel_cli with a new bundled sqlite (needed for fts trigram) we need to git clone it and edit cargo.toml. sad!
  * see https://github.com/rust-lang/cargo/issues/3266
  * check out diesel github 1.x version
  * set rust version (not sure why this is necessary) `rustup override set 1.66.0`
  * edit cargo.toml in diesel_cli folder to increase minimum sqlite version like `>=0.22.2`
  * delete examples from top-level diesel cargo.toml (dependency problems in git checkout version)
  * in diesel_cli folder: `cargo install diesel_cli --no-default-features --features "sqlite-bundled" --path .`
* adding a new table with diesel_cli:
* `diesel migration generate [new table name]`
* `diesel migration run` - or omit this and just run all tests, there are embedded migrations
* `diesel migration redo` (checks up+down)

# diesel 2.0 upgrade - todo
* https://github.com/actix/examples/tree/master/databases/diesel
* I'm waiting for this example to be updated, the changes required were too confusing:
  * https://github.com/actix/examples/blob/master/databases/diesel/Cargo.toml#L8

# external issues I'm tracking
* support loading sqlite modules in diesel in order to use spatialite
  * https://github.com/diesel-rs/diesel/issues/1867
  * https://github.com/diesel-rs/diesel/pull/2180
* rust cargo: use lld on windows for faster builds. will eventually be default and I can remove the /.cargo/config.toml entry
  * in my testing this doesn't improve full build time at all
  * https://github.com/rust-lang/rust/issues/71520
