@echo off
set source_file=%temp%\%random%-%random%-%random%.bat
type nul > %source_file%
python %~dp0run_testenv.py --source-file %source_file% %*
call %source_file%
del %source_file%
set source_file=
