.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_maintenance_downgrade:

.. cspell:words linenos

Downgrade Parsec Server
=======================

This section explains how to downgrade your Parsec Server. This shouldn't be necessary
under normal circumstances, but it can be useful if a critical issue arises.

.. warning::

  Currently, the only way to rollback is to use the last database backup.
  This means that you will lose the delta between last database backup and the current database.

.. warning::

  Before downgrading, make sure to back up the database as explained in the
  :ref:`Backup and Restore section <doc_hosting_maintenance_backup_restore>`
  in case you need to roll back the downgrade.

Downgrade with Docker
---------------------

The following steps show how to downgrade the ``parsec-server`` service from version ``v3.1.0`` to ``v3.0.0``.

#. Downgrade the ``parsec-server`` docker image tag in ``parsec-server.docker.yaml``.

   This is equivalent to the first step in :ref:`Upgrade with Docker <doc_hosting_maintenance_upgrade_docker>`.

#. Replace the current database with the backup on the PostgreSQL database.

#. :ref:`Restart the parsec-server container<restart_parsec_server_container>`.
