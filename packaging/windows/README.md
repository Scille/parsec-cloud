# NSIS installer for Parsec

Inspired by [Deluge NSIS installer](https://github.com/deluge-torrent/deluge/blob/3f9ae337932da550f2623daa6dedd9c3e0e5cfb3/packaging/win32/Win32%20README.txt)

## 1 - Build the application

Run the `freeze_program.py` Python script with the path to the Parsec sources to use:

```shell
$ python freeze_program.py ../..
```

Note the Python version embedded inside the build will be taken from the interpreter
you run the script with.

On top of the build, the script will generate `install_files.nsh`, `uninstall_files.nsh`
and `manifest.ini` files that will be used by the packaging nsis script.
It will also download a WinFSP installer which is also needed by the packaging nsis script.

## 2 - Package and sign the application

### 2.1 - Install signtool & Nsis

The `signtool` command is [part of the Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk/)

The `NSIS` install packager is [available on SourceForge](https://sourceforge.net/projects/nsis/)
(SourceForge ? Now, that's a name I haven't heard in a long time...)

The `signtool` and `makensis` command must be added to the $PATH:

Can be done for example doing

```shell
set PATH=C:\Program Files (x86)\Windows Kits\10\bin\$VERSION\x64;%PATH%
set PATH=C:\Program Files (x86)\NSIS;%PATH%
```

### 2.2 - The simple way

Run the `make_installer.py` Pythons script:

```shell
python make_installer.py --sign-mode exe
```

This will generate a `build\parsec-<version>-<platform>-setup.exe` installer.

### 2.3 - The advanced way

Under the hood, `make_installer.py` does 3 things:

- Sign the `parsec.exe` in the application distribution
- Generate an installer from the application distribution
- Sign the installer

The signature are done with a command of the type:

```shell
signtool sign /n Scille /t http://time.certum.pl /fd sha256 /d "Parsec by Scille" /v $EXECUTABLE
```

And the NSIS script is run by doing:

```shell
"C:\Program Files (x86)\NSIS\makensis.exe" installer.nsi
```
