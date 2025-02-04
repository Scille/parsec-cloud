.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_parsec_workspaces:

Parsec workspaces
=================

In Parsec, your data is securely stored within **workspaces**.

.. image:: screens/parsec_workspace.png
    :align: center
    :alt: Parsec workspace
    :width: 300

You can import your data into a Parsec workspace and manage your files and
directories as you will do with a regular file explorer.

.. image:: screens/parsec_file_explorer.png
    :align: center
    :alt: Parsec file explorer

Workspaces are mounted by default in your system and will also appear in the
file explorer as regular folders. This is convenient to copy files from and to
Parsec.

Each workspace has its own :ref:`role-based policy <doc_userguide_parsec_workspaces_roles>`
for read and write access. This allows a fine-grained access control as each
user can have different roles in different workspaces.

.. note::

    When you are offline, you can still access documents provided they were
    synchronized by Parsec while connected. Synchronization will occur
    automatically as soon as the connection with the server is established.

.. mount/unmount function not yet available on V3
.. .. note::
..     Although workspaces are mounted by default, they can be unmounted or mounted back using the toggle at the bottom left of the workspace card. When a workspace is unmounted, his data are not accessible in Parsec, and it is not reachable through the regular file explorer of the computer.
..     .. image:: screens/workspace_unmounted_mounted.png
..         :align: center
..         :alt: workspaces unmounted and mounted
..
..
.. .. image:: screens/parsec_file_explorer.png
..    :align: center
..    :alt: Parsec in file explorer


.. _doc_userguide_parsec_workspaces_create:

Create a workspace
------------------

You can create a workspace by clicking ``New workspace`` and entering a name for
the workspace.

.. image:: screens/create_workspace.png
    :align: center
    :alt: Creating a workspace

When you create a workspace, you automatically get the :ref:`Owner role <doc_userguide_parsec_workspaces_roles>`
within the workspace.


.. _doc_userguide_parsec_workspaces_share:

Share a workspace
-----------------

If you have the **Owner** or **Manager** role, the ``Sharing and roles`` option
will be available from the workspace menu.

Find the user you want to share the workspace with and select its
:ref:`workspace roles <doc_userguide_parsec_workspaces_roles>`.

.. image:: screens/share_workspace.png
    :align: center
    :alt: Sharing a workspace

Depending on the user profile within the organization, some roles may not be
available for the selected user.

You can remove access to this workspace by selecting ``Not shared`` for a given
user.


.. _doc_userguide_parsec_workspaces_roles:

Workspace roles
---------------

The **workspace role** defines what the user is allowed to do within the
workspace. Since the role is specific to the workspace, a user can have
different roles in different workspaces.

The available roles and what they allow to do are shown in the following table.

.. list-table::
   :align: center
   :header-rows: 1

   * - User rights
     - Reader
     - Contributor
     - Manager
     - Owner
   * - Can view and open files
     - ✅
     - ✅
     - ✅
     - ✅
   * - Can edit, import and delete files
     - ❌
     - ✅
     - ✅
     - ✅
   * - Can manage user access to the workspace
     - ❌
     - ❌
     - ✅
     - ✅
   * - Can re-encrypt the workspace
     - ❌
     - ❌
     - ❌
     - ✅
   * - Can promote other users to Owner
     - ❌
     - ❌
     - ❌
     - ✅

Users without a role in the workspace, are not allowed to access nor see the
workspace.

.. warning::
  It is recommended to always share the workspace with other users.

  Strong cryptographic security prevents data recovery. If the workspace is
  not shared with others, and the user loses access to its device or cannot
  log in for any reason, data stored in the workspace will be lost forever.


Copy the roles of one user to another
-------------------------------------

Assigning roles on multiple workspaces for a single user can become a bit tedious. This feature exists to help you copy the roles from one user to another on all workspaces.

As an example, Mallory is an intern in your company, and uses Parsec. You shared four workspaces with her, two where she is a `Reader`, two where she is a `Contributor`. Bob is a newly hired intern, and you want to share with him the same workspaces you shared with Mallory, with the same roles. Instead of sharing each workspace one by one, you could instead go to ``Manage my organization`` in the top left corner and access the list of users. Right click on Mallory, and select ``Copy workspace roles to...``.

.. image:: screens/batch_workspace_context_menu.png
    :align: center
    :width: 800
    :alt: Batch workspace assignment screenshot

A dialog opens, asking you to select the user whose roles you want to copy. Simply start typing their name or email address, and select the option the application will give you.

.. image:: screens/batch_workspace_select_user.png
    :align: center
    :width: 400
    :alt: Batch workspace select user screenshot

Parsec gives you the list of updates that will be performed. If satisfied with the changes, click on ``Copy roles``.

.. image:: screens/batch_workspace_summary.png
    :align: center
    :width: 400
    :alt: Batch workspace select user screenshot

A few things to note:

* Usual rules still apply: a user with an `External` profile cannot be made `Manager` or `Owner`.
* If the target user already has a higher role on the workspace, it will not be changed.
* It will not change the roles the target user may have on other workspaces that are not shared with the source user.
* Only workspaces both you and the source have access to will be changed.
* Only the workspaces where you are either `Manager` or `Owner` will be changed.


Browse workspace history and restore files
------------------------------------------

Parsec allows you to browse a workspace at a given time, showing you all the files as they were.
You will need to have the **Owner** or **Manager** roles on the workspace.
The ``History`` option is available in the workspace context menu.

.. image:: screens/workspace_context_menu_history.png
    :align: center
    :width: 250
    :alt: Browse workspace history

Once you enter the History mode, you can navigate inside the workspace as you normally would.

.. image:: screens/workspace_history.png
    :align: center
    :alt: Workspace history

If you change the date or time, files and folders will be automatically updated to reflect the state of the workspace at this moment.
You can only select a time between the workspace's creation date and the current date.

.. image:: screens/workspace_history_select_date.png
    :align: center
    :width: 300
    :alt: Select a date and time

If you want to restore a file or a folder, select it and click ``Restore``. This will replace the current version of the file with the version from the selected date and time.

.. image:: screens/workspace_history_restore.png
    :align: center
    :alt: Restore a file

.. note::

  If you make a mistake, don't worry, the file history is incremental and therefore it is never deleted! Let's take an example with a file named **File.txt** whose content has been updated as follows:

    #. On April 1st, **Creation** of the file with the content **AAA**. This is **version 1**.
    #. On April 5th, **Update** of the file with the content **BBB** (replacing the previous content). This is **version 2**.
    #. On April 7th, **Update** of the file with the content **CCC** (replacing the previous content). This is **version 3**.

  If you look at this workspace history on April 6th, the content of the file will be **BBB**. Should you chose to restore this version, the content of **File.txt** (**CCC** currently) will be replaced by **BBB**. This will be **version 4**, which means that **version 3** has not been deleted, and if you later change your mind, you will still be able to restore it.


You can also explore the workspace history from a specific file. This will open the history page directly where the file is stored.

.. image:: screens/workspace_history_from_file.png
    :align: center
    :alt: Open workspace history from a file
