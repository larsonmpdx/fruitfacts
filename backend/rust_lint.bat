del Cargo.lock || goto :error
cargo update || goto :error
cargo upgrade --pinned || goto :error
cargo fmt || goto :error
cargo fix --allow-dirty || goto :error
cargo clippy --fix --allow-dirty || goto :error
cargo build || goto :error
cargo test -- --include-ignored || goto :error

goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
