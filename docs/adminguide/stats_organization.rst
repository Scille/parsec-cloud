.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_adminguide_stats_organization:

Extract organization statistics
===============================

In order to extract organization statistics, the administrator needs to provide :

- The parsec metadata server location, through a parsec url ``parsec://hostname:port``
- The ``administration_token`` configured in the parsec metadata server
- The organization name

.. code-block:: shell

    $ parsec.cli core stats_organization --addr=parsec://example.com --administration-token=s3cr3t TestOrganization

    active_users: 31
    data_size: 130464340
    metadata_size: 154131
    realms: 56
    users: 31
    users_per_profile_detail: [{'revoked': 0, 'active': 31, 'profile': Admin}, {'revoked': 0, 'active': 0, 'profile': Standard}, {'revoked': 0, 'active': 0, 'profile': Outsider}]
