.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_user:

=================
User CLI commands
=================

.. _cli_user_list:

List
====

List all users of the organization, providing the following information:

- user id
- user name
- email
- profile

.. code-block:: shell

    parsec-cli user list -d b1e1
    Enter password for the device:
    ✔ Poll server for new certificates
    Found 3 user(s):
    b35ae5d5-1734-42e6-802a-39024d25399d - Alice <alice@example.com>: ADMIN
    25d3bfbc-3738-4a37-ae20-84ad67b423cc - Bob <bob@example.com>: STANDARD
    8df7218c-c927-4487-9bcf-5b74bb9960ed - Toto <toto@example.com>: OUTSIDER


Revoke
======

To remove access for a user. The user doing the revocation must have an Administrator profile.

.. important::

  Revocation is irreversible.
  The revoked user will loose access to the organization, but any data
  already downloaded on their physical device (local cache) will not be
  removed, neither updated.

.. code-block:: shell

    parsec-cli user revoke 'toto@example.com' -d 0529
    Enter password for the device:
    ✔ Poll server for new certificates
    User toto@example.com has been revoked
