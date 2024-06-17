.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting:

Troubleshooting
===============


MS Excel/Word or another MS Office software says the file name is too long
--------------------------------------------------------------------------

.. note::

    This error message occurs when you save or open a file if the path to the file (including the file name) exceeds 218 characters.
    This limitation includes three characters representing the drive, the characters in folder names, the backslash character between folders,
    and the characters in the file name.

    `Error message when you open or save a file in microsoft excel filename <https://support.microsoft.com/en-us/help/213983/error-message-when-you-open-or-save-a-file-in-microsoft-excel-filename>`_

So this is a Microsoft issue, and there is nothing Parsec can do about it. The only solution is to rename the file, or rename one or more folders that contain the file in order to have shorter names.

Specific to Parsec V2
---------------------

Copying files from Parsec V2 take some time / files are not accessible during copy
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

When copying files from a mounted workspace in Parsec V2 the operation may take some time and the files may not be accessible to open them. After some time, files are copied and can be open normally.

This is most likely due to data is not being present locally in cache. Reading data when is not in cache is much slower than reading data from from a regular on-disk filesystem and performance is heavily dependent on your internet connection speed.

Moreover, while the file is under copy, its displayed size only grows when a *flush* operation occurs. Flush are not explicitly issued by Parsec V2 and happen automatically when data written since previous flush is bigger than the blocksize of the file (i.e. 512KB).

So this explain why the file can appear with a size of 0 for some time: data is being downloaded into cache and then copied via occasional flush operations.

You could try enabling :ref:`off-line mode <doc_userguide_workspace_offline_mode>` on workspaces before copying large amount of files. In any case, you will need to wait for the copy to be fully finished to be able to open files.

How to remove old devices from the login page
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Old devices remain registered on the computer and are displayed in the login page. This is usually the case after having tried multiple Parsec beta versions.

You can still remove these devices manually:

1. Go to Parsec configuration directory

   - **On Windows**, open the file explorer and put ``%APPDATA%/parsec**`` in the address bar (or go to ``C:\Users\<YourUser>\AppData\Roaming``, note that AppData is a hidden folder).
   - **On Linux**, open ``~/.config/parsec``.

2. Remove unwanted devices in config/devices
3. Remove unwanted devices in data

.. warning::

    Be very careful. Deleting a device means virtually deleting the data stored in that device (since there's no way to decrypt it).
