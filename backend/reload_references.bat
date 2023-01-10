rem do this through a test run, the rest of the tests don't take any time
cargo test -- --include-ignored || goto :error
goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
