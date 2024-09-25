.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_stats_server:

Extract server statistics
=========================

In order to extract server statistics, the administrator needs to provide :

- The parsec metadata server location, through a parsec url ``parsec3://hostname:port``
- The ``administration_token`` configured in the parsec metadata server
- The extraction date, ignore everything after the date
- The output filename
- The output file format which can be CSV or JSON

.. code-block:: shell

    parsec-cli server stats --addr=parsec3://example.com --token=s3cr3t --end-date=2024-08-31 --format=csv > 202408-my_server_stats.csv

Example of output:

.. code-block:: JSON

    {
      "stats": [
        {
          "active_users": 1,
          "data_size": 18333,
          "metadata_size": 1158,
          "organization_id": "JohnOrg",
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
      ]
    }
