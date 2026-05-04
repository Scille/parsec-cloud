.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_maintenance_upgrade:

.. cspell:words linenos

=====================
Upgrade Parsec Server
=====================

This section describes the general steps required to upgrade Parsec Server.

This section assumes that you deployed Parsec following the instructions from
:ref:`Server deployment section <doc_hosting_deployment>`.
If you deployed Parsec differently, you might need to adapt this section to your custom deployment.

Follow the steps below depending on the method used to install Parsec Server:

- :ref:`Upgrade with Docker <doc_hosting_maintenance_upgrade_docker>`, if you installed Parsec Server with Docker.
- :ref:`Upgrade on Linux <doc_hosting_maintenance_upgrade_linux>`, if you installed Parsec Server directly on Linux.


.. _doc_hosting_maintenance_upgrade_docker:

Upgrade with Docker
===================

.. important::

  Before upgrading, make sure to back up the database as explained in the
  :ref:`Backup and Restore section <doc_hosting_maintenance_backup_restore>`
  in case you need to roll back the update.

The following steps show how to upgrade the ``parsec-server`` service from version ``v3.0.0`` to ``v3.1.0``.

#. Update the ``parsec-server`` Docker image tag in ``parsec-server.docker.yaml`` from this:

   .. code-block:: yaml
     :linenos:
     :emphasize-lines: 4

     services:
       # ...
       parsec-server:
         image: ghcr.io/scille/parsec-cloud/parsec-server:v3.0.0
         # ...

   To this:

   .. code-block:: yaml
     :linenos:
     :emphasize-lines: 4

     services:
       # ...
       parsec-server:
         image: ghcr.io/scille/parsec-cloud/parsec-server:v3.1.0
         # ...

#. List migrations to be applied:

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

   The lines ending with ``already applied`` are migrations already applied on the database
   whereas the one ending with ``✔`` are migrations to be applied.

#. Apply database migrations:

   .. code-block:: bash

     docker compose -f parsec-server.docker.yaml run parsec-server migrate

.. _restart_parsec_server_container:

#. Restart the ``parsec-server`` container:

   .. code-block:: bash

     docker compose -f parsec-server.docker.yaml restart parsec-server


.. _doc_hosting_maintenance_upgrade_linux:

Upgrade on Linux
================

.. important::

  Before upgrading, make sure to back up the database as explained in the
  :ref:`Backup and Restore section <doc_hosting_maintenance_backup_restore>`
  in case you need to roll back the update.

.. important::

  Some :ref:`Parsec customizations <doc_hosting_custom_branding>` require you to modify source
  files *inside* the installation directory. This files are overwritten during upgrade so you have
  to make a copy before starting, and restore them after upgrade.

  Since Parsec v3.5, email templates and server pages can be loaded from a template directory
  *outside* the installation directory. This is achieved by using ``--template-dir`` option when
  running Parsec Server.

The following steps show how to upgrade Parsec Server directly on Linux.

#. Stop the running Parsec Server

   This step would depend on how you run Parsec Server.

   For example if you used :command:`systemctl` you would run something like:

   .. code-block:: bash

     systemctl stop parsec.service

#. Install Parsec Server and apply database migrations

   Follow the steps described in :ref:`Deploy with Linux <doc_hosting_deployment_with_linux>`.

#. Check that the right version is installed

   .. code-block:: bash

     python -m parsec --version

#. Restart Parsec Server

   This step would depend on how you run Parsec Server.

   For example if you used :command:`systemctl` you would run something like:

   .. code-block:: bash

     systemctl restart parsec.service

#. Check that the Parsec Server is up and running.

   Open a web browser and go to the URL were you have deployed your Parsec Server.

   **If web application is enabled**: you should be redirected to the Welcome page.

   **If web application is not enabled**: you should see the page that the server is running.

   .. figure:: ../figures/parsec_server_running.png
       :align: center
       :alt: Parsec Server is up and running
