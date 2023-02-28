# Proposal for updating Parsec/winfsp

From [ISSUE-977](https://github.com/Scille/parsec-cloud/issues/977)

## Problem

At the moment, winfsp is never updated: it is only installed by the parsec installer if it's not already present. At the moment, the winfsp installer doesn't support updating so the procedure is:

- uninstall winfsp
- reboot
- install winfsp

This makes it hard to integrate into our installer.

## Solution

There is an alternative solution though. The same way we check at startup if a new version of parsec is available, we can check if winfsp is present or if a new version available. For instance, the following checks and dialogs could be used:

- Is parsec up-to-date? If yes:
  - prompt the user for a parsec update
  - if the user agrees:
    - the parsec installer is downloaded
    - parsec shuts down
    - the parsec installer is run

- Is winfsp installed? If no:
  - prompt the user for a winfsp installation
  - if the user agrees:
    - the winfsp installer is downloaded
    - the winfsp installer is run
    - parsec is restarted

- Is winfsp up-to-date? If no:
  - prompt the user for a winfsp update
  - mention that a reboot is required
  - if the user agrees:
    - inform the user that winfsp needs to be uninstalled first
    - shutdown parsec
    - run the winfsp uninstaller
    - at the end of the installation the machine is rebooted
    - then the user starts parsec again
    - winfsp is not present and the second routine starts (see "Is winfsp installed?")

> During the parsec installation, winfsp can be installed silently to avoid too much confusion.
