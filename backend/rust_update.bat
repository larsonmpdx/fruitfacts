rem run after a rust version update to fetch it and update all tools
rustup update stable || goto :error
rustup toolchain install nightly || goto :error
cargo install cargo-edit --locked || goto :error
cargo install cargo-udeps --locked || goto :error
del Cargo.lock || goto :error
cargo test -- --include-ignored || goto :error

rem print versions of rust to a file ./rust_versions.txt
rustc --version >> rust_versions.txt || goto :error
cargo --version > rust_versions.txt || goto :error
cargo clippy --version >> rust_versions.txt || goto :error
cargo udeps --version >> rust_versions.txt || goto :error
cargo upgrade --version >> rust_versions.txt || goto :error

goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
