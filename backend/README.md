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

# rust hints
* copy the .env.example file and fill in secrets as needed
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

# js hints
* install nvm (there is a related windows project)
  * `nvm install lts` and `nvm use lts`
* `npm run dev` run a node host (with server-side rendering)
* `ncu -u` update package.json versions after installing `npm i -g npm-check-updates`

# debugging in vs code
* see extensions.json for recommended extensions
  * 2021: on windows vs code, `codelldb+rust-analyzer` debugger works slightly better than cppvsdbg or the official "rust" extension. see launch.json
  * 2021: vs code rust plugins work best when the folder opened has cargo.toml in its root (don't open the whole repo).  hopefully this gets better over time

# diesel (rust ORM) things
* see https://diesel.rs/guides/getting-started
* there's no way to specify a dependency version with a regular `cargo install` command, and diesel seems to randomly pick an old bundled sqlite version, so to get diesel_cli with a new bundled sqlite (needed for fts trigram) we need to git clone it and edit cargo.toml. sad!
  * see https://github.com/rust-lang/cargo/issues/3266
  * check out diesel github 1.x version
  * set rust version (not sure why this is necessary) `rustup override set 1.56.0`
  * edit cargo.toml in diesel_cli folder to increase minimum sqlite version like `>=0.22.2`
  * delete examples from top-level diesel cargo.toml (dependency problems in git checkout version)
  * in diesel_cli folder: `cargo install diesel_cli --no-default-features --features "sqlite-bundled" --path .`
* adding a new table with diesel_cli:
* `diesel migration generate [new table name]`
* `diesel migration run` - or omit this and just run all tests, there are embedded migrations
* `diesel migration redo` (checks up+down)

## sveltekit
* https://kit.svelte.dev/
* `npm init svelte@next frontend` - in frontend dir - `npm install`  -  `npm run dev -- -open`

# external issues I'm tracking
* support loading sqlite modules in diesel in order to use spatialite
  * https://github.com/diesel-rs/diesel/issues/1867
  * https://github.com/diesel-rs/diesel/pull/2180
* windows better support for long paths so references can have long names
  * https://github.com/rust-lang/rust/issues/67403
* rust cargo: use lld on windows for faster builds. will eventually be default and I can remove the /.cargo/config.toml entry
  * in my testing this doesn't improve full build time at all
  * https://github.com/rust-lang/rust/issues/71520
* sveltekit: better support for relative paths in static sites
  * https://github.com/sveltejs/kit/issues/1480
  * https://github.com/sveltejs/kit/issues/595#issuecomment-842278606
* sveltekit: debug server side code
  * https://github.com/sveltejs/kit/issues/1144
  * https://github.com/vitejs/vite/pull/3928
* sveltekit: routing is totally insane and broken and needs a location.ts store workaround
  * https://github.com/sveltejs/kit/issues/552
