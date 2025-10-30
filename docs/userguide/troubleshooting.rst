.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting:

Troubleshooting
===============

MacOS Finder errors
-------------------

Some file operations can fail when browsing your files through the macOS Finder.
This is due to some macOS system updates that may break compatibility with the
filesystem driver used by Parsec.

Make sure your version of **macFUSE** is up to date, either by checking in
`System Settings > macFUSE`, or through the `macFUSE <https://osxfuse.github.io/>`_
website. After update, you will need to allow the use of **macFUSE** in
`System Settings > Privacy and Security` and reboot your Mac.

If the issues persist on the latest version, you can still perform file
operations directly in Parsec until the issue is fixed.

The Parsec interface (GUI) allows you to:

* **copy** and **move** files within the same workspace
* **delete** and **rename** files from your workspace
* **import** files into your workspace either:

  * by drag & drop from the Finder into a directory in Parsec
  * by using the ``Import`` button


Microsoft Excel/Word or similar software says the file name is too long
-----------------------------------------------------------------------

This error is due to Microsoft's `Maximum File Path Limitation <https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation>`_.

When you save or open a file, its path (including the file name) cannot exceeds
260 characters. This limitation includes 3 characters for the drive (i.e. `D:`),
the characters in folder names, the backslash character between folders, and
the characters in the file name.

So the only solution is to rename the file or the folders containing it in
order to have shorter path.

Microsoft's article also mentions a more cumbersome approach by modifying a
registry entry. This might work, but it has not been tested in Parsec.
