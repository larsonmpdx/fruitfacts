rem calls the thumbnail update scripts to generate thumbnails for any that don't already have one
PUSHD backend
cargo run --bin pdf_to_thumbnail || goto :error
cargo run --bin web_thumbnails || goto :error
POPD

echo next step is "dvc_add.bat" if any new thumbnails were generated

goto :EOF

:error
POPD
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
