.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting:

Troubleshooting
===============

Excel/Word or another MS Office software says the file name is too long
-----------------------------------------------------------------------

.. note::

    This error message occurs when you save or open a file if the path to the file (including the file name) exceeds 218 characters.
    This limitation includes three characters representing the drive, the characters in folder names, the backslash character between folders,
    and the characters in the file name.

    `Error message when you open or save a file in microsoft excel filename <https://support.microsoft.com/en-us/help/213983/error-message-when-you-open-or-save-a-file-in-microsoft-excel-filename>`_

So this is a Microsoft issue, and there is nothing Parsec can do about it. The only solution is to rename the file, or rename one or more folders that contain the file in order to have shorter names.

Too many old devices appear in the device selection dropdown
------------------------------------------------------------

This usually occurs when a user has tried Parsec beta versions in which test databases could be reset at any time, leaving old devices still registered on their computer.

A user can still erase these devices manually though:

**On Windows**, in the explorer's address bar, go to ``%APPDATA%/parsec**`` (or ``C:\Users\<YourUser>\AppData\Roaming``, AppData is a hidden folder).
**On Linux**, go to ``~/.config/parsec``.

- Remove unwanted devices in config/devices
- Remove unwanted devices in data

.. warning::

    Be very careful. Deleting a device virtually means deleting the data stored in that device (since there's no way to decrypt them).
