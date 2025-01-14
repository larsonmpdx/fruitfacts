cd ./backend/

set "command=rust_update.bat"
call %command% || goto :error

set "command=rust_lint.bat"
call %command% || goto :error

cd ../frontend/

set "command=node_update.bat"
call %command% || goto :error

set "command=node_lint.bat"
call %command% || goto :error

cd ../

echo finished
goto :EOF

:error
echo %command% Failed with error #%errorlevel%
exit /b %errorlevel%
