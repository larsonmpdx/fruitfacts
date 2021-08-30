# hints
* `rustup update stable`
* `cargo fetch` install packages
* `cargo outdated` after installing `cargo install --locked cargo-outdated`

# diesel install
* https://vcpkg.io/en/getting-started.html
* `.\vcpkg\vcpkg --triplet x64-windows-static-md install sqlite3`
* copy vcpkg libs from `\vcpkg\installed\x64-windows-static-md\lib` to `C:\Users\user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
* `cargo install diesel_cli --no-default-features --features "sqlite"` helper tool
