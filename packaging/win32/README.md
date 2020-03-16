NSIS installer for parsec
=========================

Inspired by (Deluge NSIS installer)[https://github.com/deluge-torrent/deluge/blob/3f9ae337932da550f2623daa6dedd9c3e0e5cfb3/packaging/win32/Win32%20README.txt]


Build steps
-----------


### 1 - Build the application

First make sure you are within a cmd.exe shell with visual studio configured
for the target architecture you want to build.

Example for Visual Studio 2017 with amd64 target:
```shell
$ "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvarsall.bat"  amd64
```

Run the `freeze_parsec.py` Python script with the branch/tag to build:
```shell
$ python3.exe freeze_parsec.py v1.4.0
```

Note the Python version embedded inside the build will be taken from the interpreter
you run the script with.
For this reason, you must run the script with a Python interpreter of the same
architecture than the one configured for visual studio.

On top of the build, the script will generate `install_files.nsh`, `uninstall_files.nsh`
and `BUILD.tmp` files that will be used by the packaging nsis script.


### 2 - Package the application

Run the NSIS script `installer.nsi`:
```shell
$ "C:\\Program Files (x86)\NSIS\makensis.exe" installer.nsi
```

This will generate a `build\parsec-<version>-<platform>-setup.exe` installer.


### 3 - Sign the application

<TODO>
