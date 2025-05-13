# Electron app for Parsec

## Signing a Windows release

The `package-client` workflow produces an artefact called `Windows-X64-electron-app-exe-pre-built`.
This artefact contains all the assets, build artefacts and scripts necessary to build a windows release (i.e an NSIS installer).
More precisely, the artefact contains the following files and directories:

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

## Building snap locally

Build configuration is stored in `snap/snapcraft.yaml` file. Currently, it includes a single app (`parsec`).

You can find an overview of the `snapcraft.yaml` schema in
[Snapcraft Build Configuration](https://snapcraft.io/docs/build-configuration).

The instructions below summarize how to build snap locally using `snapcraft` and `lxd`.
You may need to adapt it for your specific system (see [Snapcraft Quickstart](https://snapcraft.io/docs/snapcraft-quickstart))

1. Install `snapcraft`

   ```shell
   sudo snap install --classic snapcraft
   ```

2. Install `lxd`

   ```shell
   sudo snap install lxd
   ```

3. Add your user to the snap `lxd` group

   ```shell
   sudo usermod -a -G lxd $USER
   ```

4. Logout and re-open your user session for the new group to become active.

5. Initialize LXD (press ENTER to select default value each time)

   ```shell
   lxd init
   ```

6. Create a symbolic link to the snap directory from the repo root directory

   ```shell
   ln -sv client/electron/snap
   ```

7. (optional) Clean your repo so the default lxd storage (30Go) does not run out of space

   ```shell
   fd node_modules --type d --prune --no-ignore --exec-batch rm -rf
   cargo clean
   snapcraft clean
   ```

8. Build the snap

   ```shell
   snapcraft pack --use-lxd -v
   ```

   - In case of error, `snapcraft` will print the path to the log file.
     Make sure to check it for hints about how to fix it.

Your Parsec snap should now be available at the root of the repo.
