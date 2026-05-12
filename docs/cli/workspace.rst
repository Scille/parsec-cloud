.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_workspace:

======================
Workspace CLI commands
======================

Create
======

To create a workspace.

.. code-block:: shell

    parsec-cli workspace create -d b1e1 new_workspace
    Enter password for the device:
    Workspace has been created with id: 0fe76194ad8649f6a8dc075020efaa4c

.. _cli_workspace_list:

List
====

To list workspaces the current user has access to.

.. code-block:: shell

    parsec-cli workspace list -d b1e1
    Enter password for the device:
    ✔ Poll server for new certificates
    Found 1 workspace(s)
    0fe76194ad8649f6a8dc075020efaa4c - new_workspace: owner

Share
=====

This is to share a workspace to a user.
To get the workspace id and the user id use `workspace list` and `user list` (see :ref:`workspace list <cli_workspace_list>` and :ref:`user list <cli_user_list>` )
The possible roles (by most rights to less rights) are owner, manager, contributor, reader.

.. code-block:: shell

    parsec-cli workspace share -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c -u 25d3bfbc-3738-4a37-ae20-84ad67b423cc -r manager
    Enter password for the device:
    Workspace has been shared

List user
=========

To list users that have access to the given workspace

.. code-block:: shell

    parsec-cli workspace list-users -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c
    Enter password for the device:
    ✔ Poll server for new certificates
    Workspace 0fe76194-ad86-49f6-a8dc-075020efaa4c is shared with 2 user(s)
    • User b35ae5d5-1734-42e6-802a-39024d25399d (ADMIN) - Alice (alice@example.com) has role owner
    • User 25d3bfbc-3738-4a37-ae20-84ad67b423cc (STANDARD) - Bob (bob@example.com) has role manager


Import
======

To import a local file or folder to a workspace.


The destination path must be absolute.
Unless the ``--parents`` option is specified, the parent directories must exist.
The ``--update`` option allows to control how existing files
are updated.

.. code-block:: shell

    parsec-cli workspace import -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c README.md /README.md
    Enter password for the device:


Archive or delete
=================

To change the accessibility status of the workspace:

- available: the workspace can be edited and shared normally
- archived: the workspace is still accessible to its members but in read-only
- deletion-planned-in-seconds: the workspace will be deleted in the specified amount of seconds.
  There is a minimum delay that depends on server configuration.

.. code-block:: shell

    # archive
    parsec-cli workspace archive -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c --archived
    Enter password for the device:
    Workspace archiving status has been updated

    # delete in a month
    parsec-cli workspace archive -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c --deletion-planned-in-seconds 2592000
    Enter password for the device:
    Workspace archiving status has been updated

    # available
    parsec-cli workspace archive -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c --available
    Enter password for the device:
    Workspace archiving status has been updated


List workspace content
======================

.. code-block:: shell

    parsec-cli ls -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c
    Enter password for the device:
    README.md


Remove file from workspace
==========================

.. code-block:: shell

    parsec-cli rm -d b1e1 -w 0fe76194ad8649f6a8dc075020efaa4c /README.md
    Enter password for the device:
