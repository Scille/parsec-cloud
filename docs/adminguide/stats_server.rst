.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_adminguide_stats_server:

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

Information available in the extraction are :

- organization_id
- data_size
- metadata_size
- realms
- active_users
- admin_users_active
- admin_users_revoked
- standard_users_active
- standard_users_revoked
- outsider_users_active
- outsider_users_revoked
