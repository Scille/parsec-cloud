.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting_macos_finder_errors:

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
