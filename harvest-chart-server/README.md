# harvest chart
1. a project to track typical harvest times for crops especially tree fruits which have consistent harvest times year to year
2. a cross-referencing system for common tree fruits, with an emphasis on information from university agricultural extension publications and other evidence-based sources so gardeners can quickly research the best varieties for their situation
3. an emphasis on citations rather than on editorializing or paraphrasing other works

# goals
* able to reproduce all of the common charts like DWN, ACN, and charts in university extension publications with some level of beauty. charts can be private or public and can be saved to a permalink
* an extensive plant database with harvest dates and references that users can start with, pull into their own charts and then modify. when adding new varieties, a default harvest time should be suggested based on the closest available data or some formula
* each variety's page should contain a list of references with harvest dates and also dates from users if they've set their data to be public
* support a few methods for harvest windows: day of year ranges, relative start like "redhaven+5", or "early/mid/late" (rated in % through the season)
* users shouldn't need to stick to the existing plant database when creating their own charts, but the existing one should be selected as a linked variety whenever possible to help share data and provide good references
* the plant database should be in a simple text format and hosted on github so it can be shared and extended by semi-technical users without working with a database or programming environment
* a map interface to see what nearby u-picks or public gardens are growing so users can find proven varieties to fill in harvest windows
* the web UI should be simple enough to be used by typical retiree gardeners
* all of an individual's data should be able to be imported/exported in a simple text format

# hints
* `cargo build`
* `cargo run`
* `cargo test` and `cargo test -- --include-ignored` (include long tests)
* `rustup update stable`
* `cargo fetch` install packages
* `cargo outdated` after installing `cargo install --locked cargo-outdated` (same command to update)
* `cargo fmt` after installing `rustup component add rustfmt`
* `cargo fix`a
* `cargo clippy` after installing `rustup component add clippy`
  * `cargo clippy --fix`

# debugging in vs code
* see extensions.json for recommended extensions
  * 2021: on windows vs code, `codelldb+rust-analyzer` debugger works slightly better than cppvsdbg or the official "rust" extension. see launch.json
  * 2021: vs code rust plugins work best when the folder opened has cargo.toml in its root (don't open the whole repo).  hopefully this gets better over time

# diesel setup on windows
* `cargo install diesel_cli --no-default-features --features "sqlite-bundled"` helper tool

# diesel (rust ORM) things
* see https://diesel.rs/guides/getting-started
* adding a new table with diesel-cli:
* `diesel migration generate [new table name]`
* `diesel migration run` - or omit this and just run all tests, there are embedded migrations
* `diesel migration redo` (checks up+down)
