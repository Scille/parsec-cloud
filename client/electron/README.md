# Electron app for Parsec

## Signing Windows releases

The `package-client` workflow produces an artefact called `Windows-X64-electron-app-exe-pre-built`. This artefact contains all the assets, build artefacts and scripts necessary to build a windows release (i.e an NSIS installer). More precisely, the artefact contains the following files and directories:

- `client/electron/app`
- `client/electron/build`
- `client/electron/assets`
- `client/electron/scripts`
- `client/electron/package.js`
- `client/electron/package.json`
- `client/electron/package-lock.json`
- `client/electron/sign-windows-package.cmd`

Once this artefact is downloaded and extracted, simply run `sign-windows-package.cmd` from its directory:

```shell
C:\Users\User\Downloads\Windows-X64-electron-app-pre-built-exe
$ .\sign-windows-package.cmd

powershell.exe -executionpolicy bypass -file scripts\sign-windows-package.ps1
up to date, audited 384 packages in 1s
[...]
> parsec@3.0.0-b.11.dev.19916+299b464 electron:sign
> node package.js --mode prod --sign
[...]
  • electron-builder  version=24.13.3 os=10.0.22000
  • writing effective config  file=dist\builder-effective-config.yaml
Downloading WinFSP
WinFSP downloaded to build/winfsp-2.0.23075.msi
  • packaging       platform=win32 arch=x64 electron=30.1.0 appOutDir=dist\win-unpacked
  • signing         file=dist\win-unpacked\parsec.exe subject=CN=Scille, O=Scille
[...]
```

There are two requirements:

- `node` has to be available at the right version (see `misc/version_updater.py`)
- `SimplySign` has to be installed (you will be prompted for a token during the first signing operation)

Then the installer is available in `dist`, for instance as `dist/Parsec_3.0.0-b.11.dev.19916+299b464_win_x86_64.exe`.
