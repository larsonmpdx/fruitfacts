rem adds files to dvc (data version control, for large files that are too big for git)
dvc add --glob -R plant_database\**\*.pdf || goto :error
dvc add --glob -R frontend\public\data\**\*.jpg || goto :error
dvc diff || goto :error
dvc commit || goto :error
dvc push || goto :error

echo next step is to add .dvc files to git

goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
