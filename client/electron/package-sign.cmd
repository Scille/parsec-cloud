:: This script is used to create a signed release of Parsec
:: using a single command from the pre-built windows artefact.

:: This is meant to be run in the `client/electron` directory.
powershell.exe -executionpolicy bypass -file scripts\package-sign.ps1
