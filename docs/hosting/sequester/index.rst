.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_sequester:

.. cspell:words linenos

=================
Sequester service
=================

The sequester service allows to recover all data from an organization. It can
only be activated during the organization creation and not afterwards.

The typical use case for this service is to respond to an investigation carried
out by an inspection service requiring access to data stored on the workspaces
of the person(s) involved in the investigation.

The service is managed by an *authority* and *service keys*. Note that the
secret parts of both authority and service keys are only used in an off-line
mode (i.e. no direct communication with the Parsec server is required), so that
they can be stored in a secure location.

The environment in which the sequester service is used can be controlled as
much as possible.

Security considerations
***********************

TODO: general recommendations about how to create and store the generated keys.

Requirements
************

In order to use the Sequester service you need:

- OpenSSL, for key generation
- Parsec CLI

Authority key
*************

You can create the authority key with the following commands:

  .. code-block:: bash

    openssl genrsa -out authority_key.private 4096
    openssl rsa -in authority_key.private -out authority_key.pub -pubout -outform PEM

Sequester service key and certificate
*************************************

You can create the service key and certificate with the following commands:

  .. code-block:: bash

    openssl genrsa -out sequester_key.private 4096
    openssl rsa -in sequester_key.private -out sequester_key.pub -pubout -outform PEM

TODO: create sequester service certificate with Parsec CLI?

Exporting data with sequester service
*************************************

TODO: create sequester realm export

Using exported data
*******************

TODO: how to visualize/process the exported data?
