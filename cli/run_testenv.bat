@echo off
:: Here, we generate a file which will be sourced.
set source_file=%temp%\%random%-%random%-%random%.bat
type nul > %source_file%

:: Here, we retrieve the terminal's PID
setlocal disableDelayedExpansion
:getLock
set "lock=%temp%\%~nx0.%time::=.%.lock"
set "uid=%lock:\=:b%"
set "uid=%uid:,=:c%"
set "uid=%uid:'=:q%"
set "uid=%uid:_=:u%"
setlocal enableDelayedExpansion
set "uid=!uid:%%=:p!"
endlocal & set "uid=%uid%"
2>nul ( 9>"%lock%" (
  for /f "skip=1" %%A in (
    'wmic process where "name='cmd.exe' and CommandLine like '%%<%uid%>%%'" get ParentProcessID'
  ) do for %%B in (%%A) do set "PID=%%B"
  (call )
))||goto :getLock
del "%lock%" 2>nul

:: Here we run the testenv with the terminal PID: %PID%
cargo run --package parsec_cli --features testenv run-testenv --main-process-id %PID% --source-file "%source_file%" %*

:: Here we remove all the environment variables
endlocal

:: Source our environment variables
call %source_file%

:: Cleanup
del %source_file%
set source_file=
