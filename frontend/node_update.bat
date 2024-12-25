rem updates node and globally-installed tools

rem get admin for "nvm use..."
net file 1>NUL 2>NUL
if not '%errorlevel%' == '0' (
    powershell Start-Process -FilePath "%0" -ArgumentList "%cd%" -verb runas >NUL 2>&1
    pause
    exit /b
)

:: Change directory with passed argument. Processes started with
:: "runas" start with forced C:\Windows\System32 workdir
cd /d %1

call nvm install lts || goto :error
call nvm use lts || goto :error
call npm i -g npm-check-updates || goto :error

echo "store versions to a file"
echo | set /p dummy_name="node: " >node_versions.txt || goto :error
call nvm current >>node_versions.txt || goto :error
echo | set /p dummy_name="npm-check-updates: " >>node_versions.txt || goto :error
call npm-check-updates --version >>node_versions.txt || goto :error

echo "finished"
goto :EOF

:error
echo Failed with error #%errorlevel%.
exit /b %errorlevel%
