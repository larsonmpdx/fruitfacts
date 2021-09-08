# harvest chart
* a project to track typical harvest times for crops especially tree fruits which have consistent harvest times year to year

# goals
* able to reproduce all of the common charts like DWN, ACN, and charts in university extension publications
* an extensive plant database with default harvest dates and references that users can start with, pull into their own charts and then modify
* users shouldn't need to stick to the existing plant database
* the plant database should be in a simple text format and hosted on github so it can be shared and extended
* a map interface to see what nearby u-picks or public gardens are growing so users can find proven varieties to fill in harvest windows
* web UI should be simple enough to be used by typical retiree gardeners

# hints
* `cargo build`
* `cargo run`
* `rustup update stable`
* `cargo fetch` install packages
* `cargo outdated` after installing `cargo install --locked cargo-outdated`
* `cargo fmt` after installing `rustup component add rustfmt`

# debugging in vs code
* 2021: on windows vs code, `codelldb+rust-analyzer` debugger works slightly better than cppvsdbg or the official "rust" extension. see launch.json
* `install codelldb extension` and `rust-analyzer extension`

# diesel things
* see https://diesel.rs/guides/getting-started
* adding a new table with diesel-cli:
* `diesel migration generate [new table name]`
* `diesel migration run`
* `diesel migration redo` (checks up+down)

## diesel setup on windows
* see https://vcpkg.io/en/getting-started.html
* `.\vcpkg\vcpkg --triplet x64-windows-static-md install sqlite3`
* copy vcpkg libs from `\vcpkg\installed\x64-windows-static-md\lib` to `C:\Users\user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
* `cargo install diesel_cli --no-default-features --features "sqlite"` helper tool
