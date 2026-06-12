.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_administration_workspace_deletion:



==================
Workspace deletion
==================

Workspace deletion is a complex operation that requires access to the to-be-deleted workspace in Parsec,
to the Parsec server and to the object storage. First the workspace's deletion is planned, then, when the delay has elapsed,
Workspace deletion is a multi-step operation:

#. A User deletes a workspace (see the :ref::`userguide _userguide_delete_workspace` or the :ref::`cli guide cli_delete_workspace`)
#. The workspace is marked as scheduled for deletion in the Server (the delay depends on the Organization configuration)
#. When the deletion date has passed, the effective removal can be triggered in the Parsec Server.
#. The corresponding blocks can be deleted from the object storage.

This section describes steps 3 and 4 in order to free up space occupied by workspaces that have been deleted by users.




#. List the technical ID of workspaces (realms) to be deleted.

   .. code-block:: shell

      $ python -m parsec list_deletable_realms --organization $ORGANIZATION_NAME --db $PG_URL
      Found 2 deletion candidate(s):
      Realm b65752f922e4413eb61b87dacbd3f804  kind=deletion_planned  deletion_date=2026-06-09T13:06:02.295211Z
      Realm d78799261b744bc89bae9e52234e6bf3  kind=orphaned          orphaned_since=2026-06-09T13:54:21.458044Z


   If the delay has not elapsed, the workspace will not appear here.

   .. note::

      The workspace name does not appear here, as this is considered a sensitive information:
      only the workspace users can know their name.

   .. note::

      A workspace is *orphaned* when no one has access to it. This can happen when all the users with access to it have been revoked. Unless the organization has a :ref:`Sequester <doc_hosting_sequester>`, no one will be able to recover the data, so it can be deleted.

#. Delete the to-be-deleted workspace.

   .. code-block:: console

      $ python -m parsec delete_realm --organization $ORGANIZATION_NAME --db $PG_URL --realm 7208844033874dfb9bb49da961d3f65a --dump-realm-blocks paths-to-delete.txt
      Exporting a list of all the realm's block that can be removed  [####################################]  1/1
      Exported 1 block slug(s) to paths-to-delete.txt
      Deleting metadata for realm 7208844033874dfb9bb49da961d3f65a... ✔
      ⚠️ The realm has been deleted, however its blocks are still present in the object storage
      ⚠️ You should now manually clean the object storage by removing all the path listed in paths-to-delete.txt

   This command does two things:

   - export the paths to the blocks as they are referenced in the object storage
   - delete the workspace from the postgresql database

   This command does *not* remove the blocks from your object storage.

#. Purge the object storage from useless objects listed in the output file of the previous command (e.g. ``paths-to-delete.txt``).

   This step depends on your configuration and you backup policy.
