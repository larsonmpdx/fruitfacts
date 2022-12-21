rem run after a rust version update to fetch it and update all tools
dvc add --glob -R plant_database\**\*.pdf || goto :error
dvc add --glob -R plant_database\**\*.jpg || goto :error
dvc diff || goto :error

echo next steps are "dvc commit" then "dvc push" and adding also .dvc files to git

goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
