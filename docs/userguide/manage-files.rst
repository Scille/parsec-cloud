.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_manage_files:

Files and folders
==============================

You can import files into a workspace. Files in a workspace are automatically shared with the users who have access to the workspace (see :ref:`Share a workspace <doc_userguide_parsec_workspaces_share>`). Depending on your :ref:`workspace role <doc_userguide_parsec_workspaces_roles>`, some features may not be available.


Importing files
---------------

Using the import button
^^^^^^^^^^^^^^^^^^^^^^^

Click on the button ``Import`` in the action bar and select ``Import files`` or ``Import a folder``.  Select the files or folder to import from your file system. The files will be imported in the current directory.

.. image:: screens/files/import_buttons.png
    :align: center
    :alt: import buttons screenshot
    :width: 400

Using drag and drop
^^^^^^^^^^^^^^^^^^^

You can drag and drop files and folders directly from your file explorer to Parsec. This is the preferred method as you are able to import both files and folders at the same time. You can either drop the files on an empty space to import them in the current folder, or on a specific folder to place them inside it.

.. image:: screens/files/drag_and_drop.png
    :align: center
    :alt: drag and drop screenshot

The import window
^^^^^^^^^^^^^^^^^

When you import files, the import windows appears automatically to show the progress. You can use Parsec during the import process. We strongly recommend that you do not close Parsec or log out before it has finished to guarantee that all your files are being imported.

.. image:: screens/files/import_window.png
    :align: center
    :alt: import window screenshot
    :width: 400

When a file is imported, Parsec will start synchronizing it right away with the server. The cloud icon next to the file name indicates its synchronization status:

* File/Folder has not yet been synchronized yet.

  .. image:: screens/files/not_synced.png
    :width: 64


* File/Folder has been fully synchronized with the server and is available to other users in the workspace.

  .. image:: screens/files/synced.png
    :width: 64


Working with files and folders
------------------------------

Creating a folder
^^^^^^^^^^^^^^^^^

To create a new folder, simply click the ``New folder`` button, or right-click in an empty space in the file list to bring out the context menu and click ``New folder``.

.. image:: screens/files/create_folder.png
    :align: center
    :alt: create folder screenshot

Deleting files
^^^^^^^^^^^^^^

Either right-click on a file and select ``Delete``, or select a file and click ``Delete`` in the action bar, and then answer ``Yes`` when asked for confirmation. You can delete multiple files at the same time.

.. image:: screens/files/delete_files.png
    :align: center
    :alt: delete files screenshot

Renaming files
^^^^^^^^^^^^^^

Either right-click on a file and select ``Rename``, or select a file and click ``Rename`` in the action bar. Enter the file's new name in the pop-up. You can only rename one file at a time.

.. image:: screens/files/rename_files.png
    :align: center
    :alt: rename files screenshot

Copying files
^^^^^^^^^^^^^

Either right-click on a file and select ``Copy``, or select a file and click ``Copy`` in the action bar. In the pop-up, select the folder you want to copy your files into. You can copy multiple files at the same time. You can only copy files inside the same workspace.

.. image:: screens/files/copy_files.png
    :align: center
    :alt: copy files screenshot

Moving files
^^^^^^^^^^^^

Either right-click on a file and select ``Move``, or select a file and click ``Move`` in the action bar. In the pop-up, select the folder you want to move your files into. You can move multiple files at the same time. You can only move files inside the same workspace.

.. image:: screens/files/move_files.png
    :align: center
    :alt: move files screenshot

.. _userguide-share-file:

Share a link to a file
^^^^^^^^^^^^^^^^^^^^^^

You can securely share a link to a file with other users. Parsec ensures only users from the organization having access to the workspace will be able to open the link.

When a user clicks on the link, if they have the proper access and permissions, Parsec will open and point to the specific file.

In order to get a link to a file, either right-click on a file and select ``Copy link``, or select a file and click ``Copy link`` in the action bar. This will copy the file link that you can share with other users of the workspace (in an email for example).

.. image:: screens/files/share_link.png
    :align: center
    :alt: share file link screenshot

File explorer integration
-------------------------

When you're logged-in to Parsec, your workspaces are available from the file explorer so you can access your data directly from your computer.

Each workspace is exposed in a specific folder matching the workspace name. You can add, delete, rename or open files, or any other other operations that you would normally do, as if your workspace was any regular folder on your computer.

Any operation performed in the file explorer will be automatically synchronized in Parsec.

If you logout or quit Parsec, your workspaces will no longer be accessible from the file explorer, until you log back in.

.. image:: screens/files/file_explorer.png
    :align: center
    :alt: file explorer screenshot

Opening a file
--------------

When you open a file in Parsec, it will be downloaded and stored locally so files that you open often will not have to be downloaded again. Large files may take some time to open the first time for this reason. If you are offline, only the files that have been downloaded while you were online can be opened.

Using an external application
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

When you double-click on a file, it will be opened with the default application that is associated with the file type on your system. This is therefore defined by your system and not by Parsec.

Using Parsec preview
^^^^^^^^^^^^^^^^^^^^

Some files can be displayed directly in Parsec without using an external application. The preview is available to all users of a workspace, by selecting ``Preview`` in a file menu.

.. image:: screens/files/context_preview.png
    :align: center
    :alt: Context menu preview

If the file type is supported (see below), Parsec will display a preview of it. While this preview does not allow you to make any change, and may lack some of the functionality provided by external applications, it is the more secure way of viewing your files as they will not leave Parsec.

.. image:: screens/files/pdf_viewer.png
    :align: center
    :alt: PDF viewer screenshot

Parsec preview is limited to files with size inferior to 15MB. The following file formats are supported:

* PDF
* Documents: DOCX (Microsoft Word), ODT (OpenOffice)
* Spreadsheet: XLSX (Microsoft Excel), XLS, ODS (OpenOffice)
* Presentation: PPTX (Microsoft Powerpoint). ODP (OpenOffice) support is coming soon.
* Plain Text and Code file formats: TXT, MD, CSV, RST, JSON...
* Audio: MP3, OGG, WAV
* Video: MP4, WEBM
* Images: PNG, JPG, JPEG, WEBP, GIF

On the desktop app, Parsec preview can be disabled in the application settings.

.. image:: screens/files/disable_viewers.png
    :align: center
    :alt: Disable viewers screenshot
    :width: 800


File editing in Parsec
^^^^^^^^^^^^^^^^^^^^^^

Some files can be edited directly in Parsec without using an external application.

The editor is available to **Owners**, **Managers** and **Contributors** of a workspace, by clicking on ``Edit`` in the file menu, clicking the file name or double-clicking the file in Parsec.

.. image:: screens/files/context_edit.png
    :align: center
    :alt: Context menu edit

If the file type is supported (see below), Parsec will open it in the editor.

.. image:: screens/files/docx_editor.png
    :align: center
    :alt: DOCX editor screenshot

File editing in Parsec is limited to files with size inferior to 15MB.

The following file formats are supported:

* Documents: DOCX (Microsoft Word), ODT (OpenOffice)
* Spreadsheet: XLSX (Microsoft Excel), XLS, ODS (OpenOffice)
* Presentation: PPTX (Microsoft Powerpoint). ODP (OpenOffice) support is coming soon.
* Plain Text and Code file formats: TXT, MD, CSV, RST, JSON...

.. note::

    File editing is currently in active development. For the moment, if two users edit a file simultaneously, you might see conflict files appear on one or both clients and the changes applied to the file might not reflect your changes.

On the desktop app, file editing in Parsec can be disabled from the application settings using the same option shown for preview.

.. image:: screens/files/disable_viewers.png
    :align: center
    :alt: Disable file editing in Parsec screenshot
    :width: 800

Recent documents
^^^^^^^^^^^^^^^^

Files you have recently opened are shown in the quick access menu in the sidebar for convenience.

.. image:: screens/files/quick_access.png
    :align: center
    :alt: Quick access screenshot
    :width: 400


Operational limits and tested volumes
-------------------------------------

Tested capacity
^^^^^^^^^^^^^^^

The application does not impose any hardcoded limits of the number of files, folders, or total volume of data that can be transferred (either uploaded or downloaded). Actual limits depend of available system resources, mainly storage and network bandwidth. There may also be differences depending on methods used: importing files by copying into a mountpoint VS drag&drop in the :abbr:`GUI (Graphical User Interface)`.

The largest workloads validated during testing are:

* Import of 100,000 files averaging 160 KB each, distributed across ~4,000 folders, for a total volume of approximately 15 GB, using drag&drop in the web GUI.
* Import of a single ~4 GB file, using drag&drop in the web GUI.
* Download of an archive containing 3885 files, distributed across ~150 folders, for a total volume of approximately 600 MB.

.. note::

    Larger volumes are expected to work but have not yet been tested.

Resource usage
^^^^^^^^^^^^^^

Testing has not shown any significant increase in CPU or memory consumption as the number of files or total data volume increases. Resource usage remains generally stable across the validated workloads.

The application does however require temporary storage space while processing imported data. As a rule of thumb, sufficient free disk space should be available to accommodate the size of the imported dataset. For example, importing 15 GB of data required approximately 15 GB of additional temporary storage during processing.

Processing time
^^^^^^^^^^^^^^^

Observed throughput during testing was approximately 2 MB/s for import and encryption and 0.5 MB/s for upload (regardless of internet bandwidth).

.. note::
    The current implementation prioritizes data integrity and reliability over raw performance.
