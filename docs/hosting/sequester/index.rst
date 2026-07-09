.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_sequester:

=================
Sequester service
=================

The sequester service allows to recover all data from an organization. It can
only be activated during the organization bootstrap and not afterwards.

The typical use case for this service is to respond to an investigation carried
out by an inspection service requiring access to data stored on the workspaces
of the person(s) involved in the investigation.

The service is managed by an *authority key* and a *service key*. Note that the
secret parts of both authority and service keys are only used in an off-line
mode (i.e. no direct communication with the Parsec server is required), so that
they can be stored in a secure location.

The environment in which the sequester service is used can be controlled as
much as possible.

Security considerations
=======================

The sequester service uses AES-256 keys which are the most sensitive elements
because they are used to decrypt the exported data.

These keys must therefore be backed up and stored on a physically protected
external medium (safe) and all decryption operations must be carried out on an
off-line workstation.

Requirements
============

In order to use the Sequester service you need:

- OpenSSL, for key generation
- Parsec CLI (``parsec-cli``)
- Parsec server CLI (from pypi ``pip install parsec-cloud``)

Authority key
=============

You can create the authority key with the following commands:

.. code-block:: bash

  openssl genrsa -aes256 -out authority_key.private 4096
  openssl rsa -in authority_key.private -out authority_key.pub -pubout -outform PEM


Bootstrap organization with sequester authority
===============================================

.. important::

  That step can only be done during the organization bootstrap and cannot be configured after the organization was bootstrapped.

Once you received a bootstrap link from the parsec server's administrator, typically done with the following command:

.. code-block:: bash

  ./parsec-cli organization create organization-sequester-test

.. note::

  That command needs ``PARSEC_SERVER_ADDR`` and ``PARSEC_ADMINISTRATION_TOKEN`` to be defined in the env variables.

  If you prefer, you can use the option ``--addr`` and ``--token`` respectively to pass those values (but we do not recommend that for ``--token`` as it is a sensible value that will be in the shell history).

You then need to bootstrap the organization while specifying the ``authority_key.pub`` file.

You can do that either via ``parsec-cli``:

.. code-block:: bash

  ./parsec-cli organization bootstrap \
               --addr="$BOOTSTRAP_LINK" \
               --device-label "my device" \
               --label "John Doe" \
               --email john.doe@example.com \
               --sequester-key ./authority_key.pub

Or using the GUI application:

#. Copy the provided bootstrap link
#. Similar to :ref:`start the invitation process <doc_userguide_join_organization_start_invitation>`, paste the provided link in the text field
#. The application will show a modal with a summary of the to be bootstrapped organization
#. In the same modal, extend ``Advanced Settings`` and enable ``Data Sequester`` feature and provide it with the ``authority_key.pub`` file:

   .. image:: imgs/create-org-with-sequester-main-key.png
#. Continue the process until the organization is bootstrapped


Sequester service key and certificate
=====================================

You can create the service key and certificate with the following commands:

#. Create the sequester keypair:

   .. code-block:: bash

      openssl genrsa -aes256 -out sequester_key.private 4096
      openssl rsa -in sequester_key.private -out sequester_key.pub -pubout -outform PEM

#. Create the service certificate

   .. caution::

      That step must be done after the organization as been bootstrapped,
      otherwise the service certificate will not be accepted by the server.

   .. danger::

      You must ensure that the computer clock is on time with the server (you can use ``date --utc`` to compare the clocks)

   .. code-block:: bash

     python -m parsec sequester generate_service_certificate \
               --service-label="Sequester service" \
               --service-public-key=./sequester_key.pub \
               --authority-private-key=./authority_key.private

   .. tip::

      This step while using the parsec server CLI does not require to be executed on the same machine as the service.

Enable sequester service
========================

#. Next, provide to the parsec server's administrator the recently generated service certificate.
#. The admin now needs to create the service on the server:

   .. code-block:: bash

     python -m parsec sequester create_service \
               --organization=organization-sequester-test \
               --service-certificate=./sequester_service_certificate-ef9adae7ee9f44cc9f974fdcaaff8839-2025-02-23T21:19:35.484948Z.pem

   .. note::

      You need to have access to the database credentials for that operation.

.. important::

  The sequester service can only be enabled on bootstrapped organization.

Exporting data with sequester service
=====================================

Realm vs Workspace: In Parsec vocabulary, workspace and realm are two sides of the same
coin. In a nutshell, the realm is a server-side concept that references the big pile of
encrypted data, while the workspace is the client-side concept that references those
data once decrypted.

Hence the "realm export" is the operation of exporting from the server all the encrypted
data that, when used with the right decryption keys, will allow us to have a full read
access on a workspace at any point in time up to the export date.

Overview:

- An organization exists with a workspace
- From the server CLI, a realm is exported. This generates a large ``.sqlite`` file
  containing all encrypted data belonging to this realm.
- The realm export file is transferred to a machine containing the decryption keys:

  - Sequestered service key in case of a sequestered organization.
  - Device key otherwise (i.e. decrypt the realm export from a machine normally
    used to run the Parsec client).

  .. code-block:: bash

    # On Parsec server, identity real (workspace) to export
    python -m parsec human_accesses \
              --organization=organization-sequester-test [--db=$POSTGRESQL_URL]

    # This creates a file "parsec-export-$ORG-realm-$REALM-$TIMESTAMP.sqlite".
    python -m parsec export_realm
              --organization=organization-sequester-test \
              --realm=<id-realm> [--db=$POSTGRESQL_URL --blockstore=$S3_URL]


Using exported data
===================

.. code-block:: bash

  # This command mount a disk drive with decrypted data
  ./parsec-cli mount-realm-export \
               parsec-export-organization-sequester-test-realm-f749b8035f6e4bd88e96ae557828a583-20250225T135312Z.sqlite \
               -d sequester:7bd707ce86df42288634f2a78db7f10e:./sequester_key.private
