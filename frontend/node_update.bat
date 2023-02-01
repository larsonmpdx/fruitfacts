rem updates node and globally-installed tools

rem get admin for "nvm use..."
Net session >nul 2>&1 || (
    PowerShell start -verb runas '%~0'
    pause
    exit /b
)

nvm install lts || goto :error
nvm use lts || goto :error
npm i -g npm-check-updates || goto :error
pause
goto :EOF

:error
echo Failed with error #%errorlevel%.
pause
exit /b %errorlevel%
