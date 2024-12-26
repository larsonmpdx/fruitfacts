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

set "command=nvm install lts"
call %command% || goto :error

set "command=nvm use lts"
call %command% || goto :error

set "command=npm i -g npm-check-updates"
call %command% || goto :error

echo "store versions to a file"
echo | set /p dummy_name="node: " >node_versions.txt || goto :error
call nvm current >>node_versions.txt || goto :error
echo | set /p dummy_name="npm-check-updates: " >>node_versions.txt || goto :error
call npm-check-updates --version >>node_versions.txt || goto :error

echo finished
goto :EOF

:error
echo %command% Failed with error #%errorlevel%
exit /b %errorlevel%
