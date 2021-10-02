# harvest chart
* a project to track typical harvest times for crops especially tree fruits which have consistent harvest times year to year

# goals
* able to reproduce all of the common charts like DWN, ACN, and charts in university extension publications with some level of beauty. charts can be private or public and can be saved to a permalink
* an extensive plant database with harvest dates and references that users can start with, pull into their own charts and then modify
* each variety's page should contain a list of references with harvest dates and also dates from users if they've set their data to be public. when pulling in varieties, allow selecting which reference date to start with based on distance, reputation, etc.
* support a few methods for harvest windows: day of year ranges, relative start like "redhaven+5", or "early/mid/late" (rated in % through the season)
* users shouldn't need to stick to the existing plant database when creating their own charts, but the existing one should be selected as a linked variety whenever possible to help share data and provide good references
* the plant database should be in a simple text format and hosted on github so it can be shared and extended by semi-technical users without working with a database or programming environment
* a map interface to see what nearby u-picks or public gardens are growing so users can find proven varieties to fill in harvest windows
* the web UI should be simple enough to be used by typical retiree gardeners
* all of an individual's data should be able to be imported/exported in a simple text format

# hints
* `cargo build`
* `cargo run`
* `rustup update stable`
* `cargo fetch` install packages
* `cargo outdated` after installing `cargo install --locked cargo-outdated`
* `cargo fmt` after installing `rustup component add rustfmt`
* `cargo fix`
* `cargo clippy` after installing `rustup component add clippy`
  * see also `cargo clippy --fix`

# debugging in vs code
* 2021: vs code rust plugins work best when the folder opened has cargo.toml in its root (don't open the whole repo).  hopefully this gets better over time
* `install codelldb extension` and `rust-analyzer extension`
  * 2021: on windows vs code, `codelldb+rust-analyzer` debugger works slightly better than cppvsdbg or the official "rust" extension. see launch.json

# diesel (rust ORM) things
* see https://diesel.rs/guides/getting-started
* adding a new table with diesel-cli:
* `diesel migration generate [new table name]`
* `diesel migration run`
* `diesel migration redo` (checks up+down)

## diesel setup on windows
* see https://vcpkg.io/en/getting-started.html
* `.\vcpkg\vcpkg --triplet x64-windows-static-md install sqlite3`
* copy vcpkg libs from `\vcpkg\installed\x64-windows-static-md\lib` to `C:\Users\user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
  * needed Sep 2021, may improve in the future
* `cargo install diesel_cli --no-default-features --features "sqlite"` helper tool
