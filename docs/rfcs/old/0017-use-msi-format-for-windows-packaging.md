# Use msi format for windows packaging

From [ISSUE-1865](https://github.com/Scille/parsec-cloud/issues/1865)

Right now we use NSIS for packaging which has a few drawbacks:

- it generates a .exe as installer
- the `uninstall.exe` is generated during the packaging, hence there is no simple way to sign it (the only solution is to install the generated package, retrieve the `uninstall.exe` from the installation folder, sign it, add it to the source files and re-do the packaging :/ )

Compared to a .msi installer, the .exe installer makes the installation process not atomic. Worst, a .exe appear totally opaque for Windows & anti-virus so they are really suspicious about it (where a .msi is well known to be an installer so things *should* go more smoothly... 🤞 🙏 🎅 )

However a major drawback of .msi is it doesn't support nested .msi installation so we cannot install WinFSP during Parsec installation. This can be circumvented however by checking if WinFSP is available when starting Parsec and prompting user about installing it.

It is worth noting this is already the behavior on MacOS where user is invited to manually install MacFUSE when running Parsec for the first time.

On top of that, we should provide a Webdav or NFS mountpoint as a fallback way to access data if WinFSP is not installed (this would be also useful on MacOS where MacFUSE installation procedure is getting ridiculously complicated with the last MacOS version...).

Last but not least, creating a msi installer is *a bit* messy (that's why we went with NSIS in the first place, even if saying that NSIS is not a paramount of user-friendliness is an understatement 😄 ).
Long story short the standard tool for generating .msi is [WIX toolset](https://wixtoolset.org/), but it's a rabbit hole so you should be careful not to get lost ! My advice is to generate a working wix config file with existing tools (for instance following a tutorial that uses visual studio given it's the most common usecase), then keep this config file and customize to turn it into a skeleton and having a python script populating it for our needs.

Another msi tool worth mentioning is the [CPython's msilib module](https://docs.python.org/3/library/msilib.html) which seems much simpler. But it might be a deception (otherwise why CPython's own msi generation code [is so big and messy](https://github.com/python/cpython/tree/main/Tools/msi) ? :trollface: )
CX_Freeze packaging project [seems to use this msilib module](https://github.com/marcelotduarte/cx_Freeze/blob/main/cx_Freeze/windist.py) for it msi generation.
Github search to find uses of msilib module: <https://github.com/search?q=msilib+language%3Apython+-filename%3Atest_msilib.py&type=Code>

So what need to be done:

- [ ] test NFS support for Windows/MacOS/Linux (typically make sure read/write is ok, find out how to have the mountpoint displayed in the file explorer, find out how to open the file explorer on the mountpoint from the GUI)
- [ ] Play with Wix (typically looking a existing wix config files to understand the different parts)
- [ ] Write a script to generate Wix config file (so win32 packaging would be `freeze_program.py && make_installer.py` (with `freeze_program.py` responsible for using PyInstaller and signing parsec.exe, then `make_installer.py` responsible for generating wix config, using wix scripts to generate the msi and eventually signing the .msi)
- [ ] Add a WinFSP available detection routine and a related GUI message if not (should be taken from the MacOS-specific codebase)
- [ ] Add a NFS mountpoint implementation enabled by default
- [ ] Modify auto-updater to work with .msi
- [ ] Ensure previous auto-updater is able to download .msi and start the update from there (this is most likely not the case, so a bootstrap .exe might be needed to help previous Parsec version to update...)

related to [RFC-0015](0015-allow-windows-installation-without-administrator-rights.md) & [PR-1062](https://github.com/Scille/parsec-cloud/pull/1062)

Additional useful stuff:

- [Wix manual](https://wixtoolset.org/documentation/manual/v3/)
- Windows NFS doc:
  - <https://docs.microsoft.com/en-us/windows-server/storage/nfs/nfs-overview>
  - <https://docs.microsoft.com/en-us/windows-server/storage/nfs/deploy-nfs>
- PyOxidizer
  - [doc about wix packaging](https://pyoxidizer.readthedocs.io/en/pyoxidizer-0.13.2/tugger_wix.html)
  - [related codebase](https://github.com/indygreg/PyOxidizer/blob/main/tugger-wix/src/simple_msi_builder.rs)
- Python script to [generate msi installer from json conf using wix](https://github.com/jpakkane/msicreator)
