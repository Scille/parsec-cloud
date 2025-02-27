.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_sequester:

.. cspell:words linenos

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
***********************

The sequester service uses AES-256 keys which are the most sensitive elements
because they are used to decrypt the exported data.

These keys must therefore be backed up and stored on a physically protected
external medium (safe) and all decryption operations must be carried out on an
off-line workstation.

Requirements
************

In order to use the Sequester service you need:

- OpenSSL, for key generation
- Parsec CLI (`parsec-cli`)
- Parsec server CLI (from pypi `pip install parsec-cloud`)

Authority key
*************

You can create the authority key with the following commands:

  .. code-block:: bash

    openssl genrsa -aes256 -out authority_key.private 4096
    openssl rsa -in authority_key.private -out authority_key.pub -pubout -outform PEM

Create and bootstrap organization with sequester authority
**********************************************************

  .. code-block:: bash

    ./parsec-cli organization create --token=s3cr3t --addr=parsec3://<server-url> organization-sequester-test
    ./parsec-cli organization bootstrap --addr="parsec3://<server-url>/organization-sequester-test?a=bootstrap_organization&p=<bootstrap-token>" --device-label "my device" --label "John Doe" --email <user-email> --sequester-key ./authority_key.pub

Sequester service key and certificate
*************************************

You can create the service key and certificate with the following commands:

  .. code-block:: bash

    # Private key must not have passphrase
    openssl genrsa -aes256 -out sequester_key.private 4096
    # If it has a passphrase
    openssl rsa -in sequester_key.private -out sequester_key_private_decrypted.pem
    openssl rsa -in sequester_key.private -out sequester_key.pub -pubout -outform PEM
    # With Parsec server CLI
    # Certificate must be generated AFTER organization bootstrapped
    python -m parsec sequester generate_service_certificate --service-label="Sequester service" \
    --service-public-key=./sequester_key.pub --authority-private-key=./authority_key.private

Enable sequester service
************************

  .. code-block:: bash

    # On Parsec server
    python -m parsec sequester create_service --db=$POSTGRESQL_URL --organization=organization-sequester-test --service-certificate=./sequester_service_certificate-ef9adae7ee9f44cc9f974fdcaaff8839-2025-02-23T21:19:35.484948Z.pem

Exporting data with sequester service
*************************************

Realm vs Workspace: In Parsec vocabulary, workspace and realm are two sides of the same
coin. In a nutshell, the realm is a server-side concept that references the big pile of
encrypted data, while the workspace is the client-side concept that references those
data once decrypted.

Hence the "realm export" is the operation of exporting from the server all the encrypted
data that, when used with the right decryption keys, will allow us to have a full read
access on a workspace at any point in time up to the export date.

Overview:

- An organization exists with a workspace
- From the server CLI, a realm is exported. This generates a large `.sqlite` file
  containing all encrypted data belonging to this realm.
- The realm export file is transferred to a machine containing the decryption keys:

  - Sequestered service key in case of a sequestered organization.
  - Device key otherwise (i.e. decrypt the realm export from a machine normally
    used to run the Parsec client).

  .. code-block:: bash

    # On Parsec server, identity real (workspace) to export
    python -m parsec human_accesses --organization=organization-sequester-test [--db=$POSTGRESQL_URL]
    python -m parsec export_realm --organization=organization-sequester-test --realm=<id-realm> [--db=$POSTGRESQL_URL --blockstore=$S3_URL]
    # This creates a file `parsec-export-$ORG-realm-$REALM-$TIMESTAMP.sqlite`.

Using exported data
*******************

  .. code-block:: bash

    # This command mount a disk drive with decrypted data
    ./parsec-cli mount-realm-export parsec-export-organization-sequester-test-realm-f749b8035f6e4bd88e96ae557828a583-20250225T135312Z.sqlite -d sequester:7bd707ce86df42288634f2a78db7f10e:./sequester_key.private
