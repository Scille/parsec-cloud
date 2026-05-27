.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting_ms_filename_too_long:

Microsoft Word or similar software says the file name is too long
-----------------------------------------------------------------

This error is due to Microsoft's `Maximum File Path Limitation <https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation>`_.

When you save or open a file, its full path cannot exceed 260 characters. This limitation includes 3 characters for the
drive (i.e. `D:`), the characters  in all the intermediate folder names, the backslash character between folders, and
the characters in the file name.

The only solution is to rename the file or the folders containing it in order to have a shorter path.

Microsoft's article also mentions a more cumbersome approach by modifying a registry entry. This might work, but it has
not been tested in Parsec.
