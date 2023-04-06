rem run after a rust version update to fetch it and update all tools
rustup update stable || goto :error
rustup toolchain install nightly || goto :error
cargo install cargo-edit --locked || goto :error
cargo install cargo-udeps --locked || goto :error
cargo test -- --include-ignored || goto :error
goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
