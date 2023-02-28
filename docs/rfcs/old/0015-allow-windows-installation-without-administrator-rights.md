# Allow Windows installation without administrator rights

From [ISSUE-1651](https://github.com/Scille/parsec-cloud/issues/1651)

Currently administrator rights are needed for the installation due to:

1) Installing in program files folder
2) Registering the `parsec://` custom url scheme in the registry database
3) Install WinFSP

Considering 1) & 2), we could do a user only installation (so not a global installation).
This would mean installing into `%LOCALAPPDATA%\Programs\Parsec\` and registergin the custom url scheme for the current user only.

For 2) we could ship a WebDAV implementation of the mountpoint (it would probably have much worst performances and integration but it's ok as a fail-safe mode).
This way the installation of Parsec wouldn't automatically install WinFSP, instead when launching Parsec a popup would be displayed to invite user to install WinFSP if it is not already installed (or if an incompatible version of WinFSP is installed).

This could fail-safe mechanism may also be useful for the MacOS version where OSXFuse is needed but not installed by default.
