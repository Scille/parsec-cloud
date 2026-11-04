.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_stats_organization:

=============================
Organization usage statistics
=============================

In order to extract organization usage statistics, the administrator needs to provide:

- The Parsec Server address as a Parsec URL ``parsec3://hostname:port``
- The ``administration_token`` configured in the Parsec Server
- The organization name

.. code-block:: shell

    parsec-cli organization stats --addr=parsec3://example.com --token=s3cr3t --organization-id=TestOrganization

Example of output:

.. code-block:: json

    {
      "active_users": 1,
      "data_size": 18333,
      "metadata_size": 1158,
      "realms": 2,
      "users": 1,
      "users_per_profile_detail": {
        "ADMIN": {
          "active": 1,
          "revoked": 0
        },
        "OUTSIDER": {
          "active": 0,
          "revoked": 0
        },
        "STANDARD": {
          "active": 0,
          "revoked": 0
        }
      }
    }
