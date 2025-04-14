.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_maintenance_upgrade_downgrade:

.. cspell:words linenos

.. important::

  This section assumes that you deployed Parsec following the instructions from
  :ref:`Server deployment section <doc_hosting_deployment>`. If you deployed
  Parsec differently, you might need to adapt this section to your custom
  deployment.

Upgrade Parsec Server
*********************

.. warning::

  Before upgrading, make sure to back up the database as explained in the,
  :ref:`Backup and Restore section <doc_hosting_maintenance_backup_restore>`
  in case you need to roll back the update.

In this guide, we will migrate ``parsec-server`` from version ``v3.0.0`` to ``v3.1.0``.

.. _update_docker_image_tag:

1. Update the ``parsec-server``'s docker image tag of in docker-compose file (``parsec-server.docker.yaml``):

  .. code-block:: yaml
    :linenos:
    :emphasize-lines: 5

    services:
      # ...

      parsec-server:
        image: ghcr.io/scille/parsec-cloud/parsec-server:v3.0.0
        # ...

  You need to change the used tag (at line **5**) to ``v3.1.0``.

2. List the pending migrations to be applied:

  .. code-block:: bash

    docker compose -f parsec-server.docker.yaml run parsec-server migrate --dry-run

  The output should look like this:

  .. code-block::

    0001_initial.sql (already applied)
    0002_add_migration_table.sql (already applied)
    0003_human_handle.sql (already applied)
    0004_invite.sql (already applied)
    0005_redacted_certificates.sql (already applied)
    0006_outsider_enabled.sql (already applied)
    0007_users_limit.sql (already applied)
    0008_apiv1_removal.sql (already applied)
    0009_add_realm_user_change_table.sql (already applied)
    0010_add_pki_certificate_table.sql (already applied)
    0011_add_sequester_tables.sql (already applied)
    0012_add_sequester_webhook.sql (already applied)
    0013_add_shamir_recovery.sql ✔
    0014_add_realm_archiving.sql ✔

  .. note::

    This output is provided as an example. Don't expect it to match your output.

  The lines ending with ``already applied`` are migrations already present on the database
  whereas the one ending with ``✔`` are migrations to be applied.

3. Apply the database migration:

  .. code-block:: bash

    docker compose -f parsec-server.docker.yaml run parsec-server migrate

.. _restart_parsec_server_container:

4. Restart the ``parsec-server`` container:

  .. code-block:: bash

    docker compose -f parsec-server.docker.yaml restart parsec-server

Downgrade Parsec Server
***********************

.. warning::

  Rollback is currently limited in Parsec.
  The only possible way to rollback is to use the previous database backup.
  So you will lose the delta of backup vs current database.

To roll back to a previous version, let's say we want to downgrade ``parsec-server`` from version ``v3.1.0`` to ``v3.0.0``.

1. Downgrade the ``parsec-server``'s docker image tag in the docker-compose file (``parsec-server.docker.yaml``).
   Like in :ref:`Update the parsec-server tag <update_docker_image_tag>` change the tag ``v3.1.0`` to ``v3.0.0``.

2. Replace the current database with the backup on the Postgres database.

3. :ref:`Restart the parsec-server container<restart_parsec_server_container>`
