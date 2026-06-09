.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_administration_workspace_deletion:



==================
Workspace deletion
==================

Workspace deletion is a complex operation that requires access to the to-be-deleted workspace in Parsec,
to the Parsec server and to the object storage. First the workspace's deletion is planned, then, when the delay has elapsed,
the removal from the Parsec server can be triggered, and finally the blocks can be deleted from the object storage.


#. Mark the workspace for deletion:

    This step must be done by someone who has ownership of the to-be-deleted workspace.

    Using the CLI:

    .. code-block:: shell

        parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_NAME \
        --deletion-planned-in-seconds 2592000

    Or the GUI

        .. figure:: ../figures/delete_workspace_context_menu_en.png
            :align: center
            :alt: Workspace contextual menu


        - click on the delete workspace button.
        - follow the instructions. You will be asked to type the workspace's name to confirm.

    .. note::

        You can customize the delay only using the CLI, but it can not be less than the organization configured delay (see the ``realm_minimum_archiving_period_before_deletion`` option in :ref:`Create Organization <doc_hosting_administration_create_organization_create>`).

    This operation makes the workspace read only, and when the delay will elapse, it will be inaccessible from the GUI.
    But the files will still exist in storage.


#. Wait for the delay to pass. The next steps will need to be executed by a server admin.

#. Find the technical id for the to-be-deleted workspace.

    .. code-block:: shell

        $ python -m parsec list_deletable_realms --organization $ORGANIZATION_NAME --db $PG_URL
        Found 2 deletion candidate(s):
        Realm b65752f922e4413eb61b87dacbd3f804  kind=deletion_planned  deletion_date=2026-06-09T13:06:02.295211Z
        Realm d78799261b744bc89bae9e52234e6bf3  kind=orphaned          orphaned_since=2026-06-09T13:54:21.458044Z


    If the delay has not elapsed, the workspace will not appear here.

    .. note::

        The workspace name does not appear here, as the server admin is not a Parsec user.

    .. note::

        A workspace is *orphaned* when no one has access to it.  This can happen when all the users with access to it have been revoked. Unless the organization has a :ref:`Sequester <doc_hosting_sequester>`, no one will be able to recover the data, so it can be deleted.

#. Delete the to-be-deleted workspace.

    .. code-block:::: shell

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

#. Purge the object storage from useless objects listed in the output file.

    This step depends on your configuration and you backup policy.
